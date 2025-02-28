// Copyright 2022 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Email transport backends

use std::{ffi::OsString, num::NonZeroU16, sync::Arc};

use async_trait::async_trait;
use lettre::{
    address::Envelope,
    transport::{
        sendmail::AsyncSendmailTransport,
        smtp::{authentication::Credentials, AsyncSmtpTransport},
    },
    AsyncTransport, Tokio1Executor,
};
use mas_http::ClientInitError;
use thiserror::Error;

pub mod aws_ses;

/// Encryption mode to use
#[derive(Debug, Clone, Copy)]
pub enum SmtpMode {
    /// Plain text
    Plain,
    /// StartTLS (starts as plain text then upgrade to TLS)
    StartTls,
    /// TLS
    Tls,
}

/// A wrapper around many [`AsyncTransport`]s
#[derive(Default, Clone)]
pub struct Transport {
    inner: Arc<TransportInner>,
}

enum TransportInner {
    Blackhole,
    Smtp(AsyncSmtpTransport<Tokio1Executor>),
    Sendmail(AsyncSendmailTransport<Tokio1Executor>),
    AwsSes(aws_ses::Transport),
}

impl Transport {
    fn new(inner: TransportInner) -> Self {
        let inner = Arc::new(inner);
        Self { inner }
    }

    /// Construct a blackhole transport
    #[must_use]
    pub fn blackhole() -> Self {
        Self::new(TransportInner::Blackhole)
    }

    /// Construct a SMTP transport
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying SMTP transport could not be built
    pub fn smtp(
        mode: SmtpMode,
        hostname: &str,
        port: Option<NonZeroU16>,
        credentials: Option<Credentials>,
    ) -> Result<Self, lettre::transport::smtp::Error> {
        let mut t = match mode {
            SmtpMode::Plain => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(hostname),
            SmtpMode::StartTls => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(hostname)?,
            SmtpMode::Tls => AsyncSmtpTransport::<Tokio1Executor>::relay(hostname)?,
        };

        if let Some(credentials) = credentials {
            t = t.credentials(credentials);
        }

        if let Some(port) = port {
            t = t.port(port.into());
        }

        Ok(Self::new(TransportInner::Smtp(t.build())))
    }

    /// Construct a Sendmail transport
    #[must_use]
    pub fn sendmail(command: impl Into<OsString>) -> Self {
        Self::new(TransportInner::Sendmail(
            AsyncSendmailTransport::new_with_command(command),
        ))
    }

    /// Construct a AWS SES transport
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client failed to initialize
    pub async fn aws_ses() -> Result<Self, ClientInitError> {
        let transport = aws_ses::Transport::from_env().await?;
        Ok(Self::new(TransportInner::AwsSes(transport)))
    }
}

impl Transport {
    /// Test the connection to the underlying transport. Only works with the
    /// SMTP backend for now
    ///
    /// # Errors
    ///
    /// Will return `Err` if the connection test failed
    pub async fn test_connection(&self) -> Result<(), Error> {
        match self.inner.as_ref() {
            TransportInner::Smtp(t) => {
                t.test_connection().await?;
            }
            TransportInner::Blackhole | TransportInner::Sendmail(_) | TransportInner::AwsSes(_) => {
            }
        }

        Ok(())
    }
}

impl Default for TransportInner {
    fn default() -> Self {
        Self::Blackhole
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Smtp(#[from] lettre::transport::smtp::Error),
    Sendmail(#[from] lettre::transport::sendmail::Error),
    AwsSes(#[from] self::aws_ses::Error),
}

#[async_trait]
impl AsyncTransport for Transport {
    type Ok = ();
    type Error = Error;

    async fn send_raw(&self, envelope: &Envelope, email: &[u8]) -> Result<Self::Ok, Self::Error> {
        match self.inner.as_ref() {
            TransportInner::Blackhole => {
                tracing::warn!(
                    "An email was supposed to be sent but no email backend is configured"
                );
            }
            TransportInner::Smtp(t) => {
                t.send_raw(envelope, email).await?;
            }
            TransportInner::Sendmail(t) => {
                t.send_raw(envelope, email).await?;
            }
            TransportInner::AwsSes(t) => {
                t.send_raw(envelope, email).await?;
            }
        };

        Ok(())
    }
}

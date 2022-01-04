// Copyright 2021 The Matrix.org Foundation C.I.C.
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

//! Handle client authentication

use headers::{authorization::Basic, Authorization};
use mas_config::{OAuth2ClientConfig, OAuth2Config};
use mas_jose::{DecodedJsonWebToken, JsonWebTokenParts, SharedSecret};
use oauth2_types::requests::ClientAuthenticationMethod;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::skip_serializing_none;
use thiserror::Error;
use warp::{reject::Reject, Filter, Rejection};

use super::headers::typed_header;
use crate::errors::WrapError;

/// Protect an enpoint with client authentication
#[must_use]
pub fn client_authentication<T: DeserializeOwned + Send + 'static>(
    oauth2_config: &OAuth2Config,
    audience: String,
) -> impl Filter<Extract = (ClientAuthenticationMethod, OAuth2ClientConfig, T), Error = Rejection>
       + Clone
       + Send
       + Sync
       + 'static {
    // First, extract the client credentials
    let credentials = typed_header()
        .and(warp::body::form())
        // Either from the "Authorization" header
        .map(|auth: Authorization<Basic>, body: T| {
            let client_id = auth.0.username().to_string();
            let client_secret = Some(auth.0.password().to_string());
            (
                ClientCredentials::Pair {
                    via: CredentialsVia::AuthorizationHeader,
                    client_id,
                    client_secret,
                },
                body,
            )
        })
        // Or from the form body
        .or(warp::body::form().map(|form: ClientAuthForm<T>| {
            let ClientAuthForm { credentials, body } = form;

            (credentials, body)
        }))
        .unify()
        .untuple_one();

    let clients = oauth2_config.clients.clone();
    warp::any()
        .map(move || clients.clone())
        .and(warp::any().map(move || audience.clone()))
        .and(credentials)
        .and_then(authenticate_client)
        .untuple_one()
}

#[derive(Error, Debug)]
enum ClientAuthenticationError {
    #[error("no client secret found for client {client_id:?}")]
    NoClientSecret { client_id: String },

    #[error("wrong client secret for client {client_id:?}")]
    ClientSecretMismatch { client_id: String },

    #[error("could not find client {client_id:?}")]
    ClientNotFound { client_id: String },

    #[error("client secret required for client {client_id:?}")]
    ClientSecretRequired { client_id: String },

    #[error("wrong audience in client assertion: expected {expected:?}, got {got:?}")]
    AudienceMismatch { expected: String, got: String },

    #[error("invalid client assertion")]
    InvalidAssertion,
}

impl Reject for ClientAuthenticationError {}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct ClientAssertionClaims {
    #[serde(rename = "iss")]
    issuer: String,
    #[serde(rename = "sub")]
    subject: String,
    #[serde(rename = "aud")]
    audience: String,
    // TODO: use the JTI and ensure it is only used once
    #[serde(default, rename = "jti")]
    jwt_id: Option<String>,
}

async fn authenticate_client<T>(
    clients: Vec<OAuth2ClientConfig>,
    audience: String,
    credentials: ClientCredentials,
    body: T,
) -> Result<(ClientAuthenticationMethod, OAuth2ClientConfig, T), Rejection> {
    let auth_type = credentials.authentication_type();
    let client = match credentials {
        ClientCredentials::Pair {
            client_id,
            client_secret,
            ..
        } => {
            let client = clients
                .iter()
                .find(|client| client.client_id == client_id)
                .ok_or_else(|| ClientAuthenticationError::ClientNotFound {
                    client_id: client_id.to_string(),
                })?;

            match (client_secret, client.client_secret.as_ref()) {
                (None, None) => Ok(client),
                (Some(ref given), Some(expected)) if given == expected => Ok(client),
                (Some(_), Some(_)) => {
                    Err(ClientAuthenticationError::ClientSecretMismatch { client_id })
                }
                (Some(_), None) => Err(ClientAuthenticationError::NoClientSecret { client_id }),
                (None, Some(_)) => {
                    Err(ClientAuthenticationError::ClientSecretRequired { client_id })
                }
            }
        }
        ClientCredentials::Assertion {
            client_id,
            client_assertion_type: ClientAssertionType::JwtBearer,
            client_assertion,
        } => {
            let token: JsonWebTokenParts = client_assertion.parse().wrap_error()?;
            let decoded: DecodedJsonWebToken<ClientAssertionClaims> =
                token.decode().wrap_error()?;

            // client_id might have been passed as parameter. If not, it should be inferred
            // from the token, as per rfc7521 sec. 4.2
            let client_id = client_id.unwrap_or_else(|| decoded.claims().subject.clone());

            let client = clients
                .iter()
                .find(|client| client.client_id == client_id)
                .ok_or_else(|| ClientAuthenticationError::ClientNotFound {
                    client_id: client_id.to_string(),
                })?;

            if let Some(client_secret) = &client.client_secret {
                let store = SharedSecret::new(client_secret);
                token.verify(&decoded, &store).await.wrap_error()?;
                let claims = decoded.claims();
                // TODO: validate the times again

                // rfc7523 sec. 3.3: the audience is the URL being called
                if claims.audience != audience {
                    Err(ClientAuthenticationError::AudienceMismatch {
                        expected: audience,
                        got: claims.audience.clone(),
                    })
                // rfc7523 sec. 3.1 & 3.2: both the issuer and the subject must
                // match the client_id
                } else if claims.issuer != claims.subject || claims.issuer != client_id {
                    Err(ClientAuthenticationError::InvalidAssertion)
                } else {
                    Ok(client)
                }
            } else {
                Err(ClientAuthenticationError::ClientSecretRequired {
                    client_id: client_id.to_string(),
                })
            }
        }
    }?;

    Ok((auth_type, client.clone(), body))
}

#[derive(Deserialize)]
enum ClientAssertionType {
    #[serde(rename = "urn:ietf:params:oauth:client-assertion-type:jwt-bearer")]
    JwtBearer,
}

enum CredentialsVia {
    FormBody,
    AuthorizationHeader,
}

impl Default for CredentialsVia {
    fn default() -> Self {
        Self::FormBody
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ClientCredentials {
    // Order here is important: serde tries to deserialize enum variants in order, so if "Pair"
    // was before "Assertion", a client_assertion with a client_id would match the "Pair"
    // variant first
    Assertion {
        client_id: Option<String>,
        client_assertion_type: ClientAssertionType,
        client_assertion: String,
    },
    Pair {
        #[serde(skip)]
        via: CredentialsVia,
        client_id: String,
        client_secret: Option<String>,
    },
}

impl ClientCredentials {
    fn authentication_type(&self) -> ClientAuthenticationMethod {
        match self {
            ClientCredentials::Pair {
                via: CredentialsVia::FormBody,
                client_secret: None,
                ..
            } => ClientAuthenticationMethod::None,
            ClientCredentials::Pair {
                via: CredentialsVia::FormBody,
                client_secret: Some(_),
                ..
            } => ClientAuthenticationMethod::ClientSecretPost,
            ClientCredentials::Pair {
                via: CredentialsVia::AuthorizationHeader,
                ..
            } => ClientAuthenticationMethod::ClientSecretBasic,
            ClientCredentials::Assertion { .. } => ClientAuthenticationMethod::ClientSecretJwt,
        }
    }
}

#[derive(Deserialize)]
struct ClientAuthForm<T> {
    #[serde(flatten)]
    credentials: ClientCredentials,

    #[serde(flatten)]
    body: T,
}

#[cfg(test)]
mod tests {
    use headers::authorization::Credentials;
    use mas_config::ConfigurationSection;
    use mas_jose::{JsonWebSignatureAlgorithm, SigningKeystore};
    use serde_json::json;

    use super::*;

    // Long client_secret to support it as a HS512 key
    const CLIENT_SECRET: &str = "leek2zaeyeb8thai7piehea3vah6ool9oanin9aeraThuci9EeghaekaiD1upe4Quoh7xeMae2meitohj0Waaveiwaorah1yazohr6Vae7iebeiRaWene5IeWeeciezu";

    fn oauth2_config() -> OAuth2Config {
        let mut config = OAuth2Config::test();
        config.clients.push(OAuth2ClientConfig {
            client_id: "public".to_string(),
            client_secret: None,
            redirect_uris: Vec::new(),
        });
        config.clients.push(OAuth2ClientConfig {
            client_id: "confidential".to_string(),
            client_secret: Some(CLIENT_SECRET.to_string()),
            redirect_uris: Vec::new(),
        });
        config.clients.push(OAuth2ClientConfig {
            client_id: "confidential-2".to_string(),
            client_secret: Some(CLIENT_SECRET.to_string()),
            redirect_uris: Vec::new(),
        });
        config
    }

    #[derive(Deserialize)]
    struct Form {
        foo: String,
        bar: String,
    }

    #[tokio::test]
    async fn client_secret_jwt_hs256() {
        client_secret_jwt(JsonWebSignatureAlgorithm::Hs256).await;
    }

    #[tokio::test]
    async fn client_secret_jwt_hs384() {
        client_secret_jwt(JsonWebSignatureAlgorithm::Hs384).await;
    }

    #[tokio::test]
    async fn client_secret_jwt_hs512() {
        client_secret_jwt(JsonWebSignatureAlgorithm::Hs512).await;
    }

    async fn client_secret_jwt(alg: JsonWebSignatureAlgorithm) {
        let audience = "https://example.com/token".to_string();
        let filter = client_authentication::<Form>(&oauth2_config(), audience.clone());

        let store = SharedSecret::new(&CLIENT_SECRET);
        let claims = ClientAssertionClaims {
            issuer: "confidential".to_string(),
            subject: "confidential".to_string(),
            audience,
            jwt_id: None,
        };
        let header = store.prepare_header(alg).await.expect("JWT header");
        let jwt = DecodedJsonWebToken::new(header, claims);
        let jwt = jwt.sign(&store).await.expect("signed token");
        let jwt = jwt.serialize();

        // TODO: test failing cases
        //  - expired token
        //  - "not before" in the future
        //  - subject/issuer mismatch
        //  - audience mismatch
        //  - wrong secret/signature

        let (auth, client, body) = warp::test::request()
            .method("POST")
            .header("Content-Type", mime::APPLICATION_WWW_FORM_URLENCODED.to_string())
            .body(serde_urlencoded::to_string(json!({
                "client_id": "confidential",
                "client_assertion": jwt,
                "client_assertion_type": "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                "foo": "baz",
                "bar": "foobar",
            })).unwrap())
            .filter(&filter)
            .await
            .unwrap();

        assert_eq!(auth, ClientAuthenticationMethod::ClientSecretJwt);
        assert_eq!(client.client_id, "confidential");
        assert_eq!(body.foo, "baz");
        assert_eq!(body.bar, "foobar");

        // Without client_id
        let res = warp::test::request()
            .method("POST")
            .header("Content-Type", mime::APPLICATION_WWW_FORM_URLENCODED.to_string())
            .body(serde_urlencoded::to_string(json!({
                "client_assertion": jwt,
                "client_assertion_type": "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                "foo": "baz",
                "bar": "foobar",
            })).unwrap())
            .filter(&filter)
            .await;
        assert!(res.is_ok());

        // client_id mismatch
        let res = warp::test::request()
            .method("POST")
            .body(serde_urlencoded::to_string(json!({
                "client_id": "confidential-2",
                "client_assertion": jwt,
                "client_assertion_type": "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                "foo": "baz",
                "bar": "foobar",
            })).unwrap())
            .filter(&filter)
            .await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn client_secret_post() {
        let filter = client_authentication::<Form>(
            &oauth2_config(),
            "https://example.com/token".to_string(),
        );

        let (auth, client, body) = warp::test::request()
            .method("POST")
            .header(
                "Content-Type",
                mime::APPLICATION_WWW_FORM_URLENCODED.to_string(),
            )
            .body(
                serde_urlencoded::to_string(json!({
                    "client_id": "confidential",
                    "client_secret": CLIENT_SECRET,
                    "foo": "baz",
                    "bar": "foobar",
                }))
                .unwrap(),
            )
            .filter(&filter)
            .await
            .unwrap();

        assert_eq!(auth, ClientAuthenticationMethod::ClientSecretPost);
        assert_eq!(client.client_id, "confidential");
        assert_eq!(body.foo, "baz");
        assert_eq!(body.bar, "foobar");
    }

    #[tokio::test]
    async fn client_secret_basic() {
        let filter = client_authentication::<Form>(
            &oauth2_config(),
            "https://example.com/token".to_string(),
        );

        let auth = Authorization::basic("confidential", CLIENT_SECRET);
        let (auth, client, body) = warp::test::request()
            .method("POST")
            .header(
                "Content-Type",
                mime::APPLICATION_WWW_FORM_URLENCODED.to_string(),
            )
            .header("Authorization", auth.0.encode())
            .body(
                serde_urlencoded::to_string(json!({
                    "foo": "baz",
                    "bar": "foobar",
                }))
                .unwrap(),
            )
            .filter(&filter)
            .await
            .unwrap();

        assert_eq!(auth, ClientAuthenticationMethod::ClientSecretBasic);
        assert_eq!(client.client_id, "confidential");
        assert_eq!(body.foo, "baz");
        assert_eq!(body.bar, "foobar");
    }

    #[tokio::test]
    async fn none() {
        let filter = client_authentication::<Form>(
            &oauth2_config(),
            "https://example.com/token".to_string(),
        );

        let (auth, client, body) = warp::test::request()
            .method("POST")
            .header(
                "Content-Type",
                mime::APPLICATION_WWW_FORM_URLENCODED.to_string(),
            )
            .body(
                serde_urlencoded::to_string(json!({
                    "client_id": "public",
                    "foo": "baz",
                    "bar": "foobar",
                }))
                .unwrap(),
            )
            .filter(&filter)
            .await
            .unwrap();

        assert_eq!(auth, ClientAuthenticationMethod::None);
        assert_eq!(client.client_id, "public");
        assert_eq!(body.foo, "baz");
        assert_eq!(body.bar, "foobar");
    }
}

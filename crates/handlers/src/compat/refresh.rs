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

use axum::{extract::State, response::IntoResponse, Json};
use chrono::Duration;
use hyper::StatusCode;
use mas_data_model::{TokenFormatError, TokenType};
use mas_storage::compat::{
    add_compat_access_token, add_compat_refresh_token, consume_compat_refresh_token,
    expire_compat_access_token, lookup_active_compat_refresh_token,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationMilliSeconds};
use sqlx::PgPool;
use thiserror::Error;

use super::MatrixError;
use crate::impl_from_error_for_route;

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    refresh_token: String,
}

#[derive(Debug, Error)]
pub enum RouteError {
    #[error(transparent)]
    Internal(Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("invalid token")]
    InvalidToken,
}

impl IntoResponse for RouteError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Internal(_) => MatrixError {
                errcode: "M_UNKNOWN",
                error: "Internal error",
                status: StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::InvalidToken => MatrixError {
                errcode: "M_UNKNOWN_TOKEN",
                error: "Invalid refresh token",
                status: StatusCode::UNAUTHORIZED,
            },
        }
        .into_response()
    }
}

impl_from_error_for_route!(sqlx::Error);
impl_from_error_for_route!(mas_storage::DatabaseError);

impl From<TokenFormatError> for RouteError {
    fn from(_e: TokenFormatError) -> Self {
        Self::InvalidToken
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct ResponseBody {
    access_token: String,
    refresh_token: String,
    #[serde_as(as = "DurationMilliSeconds<i64>")]
    expires_in_ms: Duration,
}

pub(crate) async fn post(
    State(pool): State<PgPool>,
    Json(input): Json<RequestBody>,
) -> Result<impl IntoResponse, RouteError> {
    let (clock, mut rng) = crate::clock_and_rng();
    let mut txn = pool.begin().await?;

    let token_type = TokenType::check(&input.refresh_token)?;

    if token_type != TokenType::CompatRefreshToken {
        return Err(RouteError::InvalidToken);
    }

    let (refresh_token, access_token, session) =
        lookup_active_compat_refresh_token(&mut txn, &input.refresh_token)
            .await?
            .ok_or(RouteError::InvalidToken)?;

    let new_refresh_token_str = TokenType::CompatRefreshToken.generate(&mut rng);
    let new_access_token_str = TokenType::CompatAccessToken.generate(&mut rng);

    let expires_in = Duration::minutes(5);
    let new_access_token = add_compat_access_token(
        &mut txn,
        &mut rng,
        &clock,
        &session,
        new_access_token_str,
        Some(expires_in),
    )
    .await?;
    let new_refresh_token = add_compat_refresh_token(
        &mut txn,
        &mut rng,
        &clock,
        &session,
        &new_access_token,
        new_refresh_token_str,
    )
    .await?;

    consume_compat_refresh_token(&mut txn, &clock, refresh_token).await?;
    expire_compat_access_token(&mut txn, &clock, access_token).await?;

    txn.commit().await?;

    Ok(Json(ResponseBody {
        access_token: new_access_token.token,
        refresh_token: new_refresh_token.token,
        expires_in_ms: expires_in,
    }))
}

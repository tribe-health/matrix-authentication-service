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

use std::str::FromStr;

use mas_data_model::{Client, User};
use oauth2_types::scope::{Scope, ScopeToken};
use rand::Rng;
use sqlx::PgExecutor;
use ulid::Ulid;
use uuid::Uuid;

use crate::{Clock, DatabaseError, DatabaseInconsistencyError};

#[tracing::instrument(
    skip_all,
    fields(
        %user.id,
        %client.id,
    ),
    err,
)]
pub async fn fetch_client_consent(
    executor: impl PgExecutor<'_>,
    user: &User,
    client: &Client,
) -> Result<Scope, DatabaseError> {
    let scope_tokens: Vec<String> = sqlx::query_scalar!(
        r#"
            SELECT scope_token
            FROM oauth2_consents
            WHERE user_id = $1 AND oauth2_client_id = $2
        "#,
        Uuid::from(user.id),
        Uuid::from(client.id),
    )
    .fetch_all(executor)
    .await?;

    let scope: Result<Scope, _> = scope_tokens
        .into_iter()
        .map(|s| ScopeToken::from_str(&s))
        .collect();

    let scope = scope.map_err(|e| {
        DatabaseInconsistencyError::on("oauth2_consents")
            .column("scope_token")
            .source(e)
    })?;

    Ok(scope)
}

#[tracing::instrument(
    skip_all,
    fields(
        %user.id,
        %client.id,
        %scope,
    ),
    err,
)]
pub async fn insert_client_consent(
    executor: impl PgExecutor<'_>,
    mut rng: impl Rng + Send,
    clock: &Clock,
    user: &User,
    client: &Client,
    scope: &Scope,
) -> Result<(), sqlx::Error> {
    let now = clock.now();
    let (tokens, ids): (Vec<String>, Vec<Uuid>) = scope
        .iter()
        .map(|token| {
            (
                token.to_string(),
                Uuid::from(Ulid::from_datetime_with_source(now.into(), &mut rng)),
            )
        })
        .unzip();

    sqlx::query!(
        r#"
            INSERT INTO oauth2_consents
                (oauth2_consent_id, user_id, oauth2_client_id, scope_token, created_at)
            SELECT id, $2, $3, scope_token, $5 FROM UNNEST($1::uuid[], $4::text[]) u(id, scope_token)
            ON CONFLICT (user_id, oauth2_client_id, scope_token) DO UPDATE SET refreshed_at = $5
        "#,
        &ids,
        Uuid::from(user.id),
        Uuid::from(client.id),
        &tokens,
        now,
    )
    .execute(executor)
    .await?;

    Ok(())
}

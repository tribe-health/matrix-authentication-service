// Copyright 2021, 2022 The Matrix.org Foundation C.I.C.
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

use axum::{
    extract::{Form, State},
    response::IntoResponse,
};
use axum_extra::extract::PrivateCookieJar;
use mas_axum_utils::{
    csrf::{CsrfExt, ProtectedForm},
    FancyError, SessionInfoExt,
};
use mas_keystore::Encrypter;
use mas_router::{PostAuthAction, Route};
use mas_storage::{user::end_session, Clock};
use sqlx::PgPool;

pub(crate) async fn post(
    State(pool): State<PgPool>,
    cookie_jar: PrivateCookieJar<Encrypter>,
    Form(form): Form<ProtectedForm<Option<PostAuthAction>>>,
) -> Result<impl IntoResponse, FancyError> {
    let clock = Clock::default();
    let mut txn = pool.begin().await?;

    let form = cookie_jar.verify_form(clock.now(), form)?;

    let (session_info, mut cookie_jar) = cookie_jar.session_info();

    let maybe_session = session_info.load_session(&mut txn).await?;

    if let Some(session) = maybe_session {
        end_session(&mut txn, &clock, &session).await?;
        cookie_jar = cookie_jar.update_session_info(&session_info.mark_session_ended());
    }

    txn.commit().await?;

    let destination = if let Some(action) = form {
        action.go_next()
    } else {
        mas_router::Login::default().go()
    };

    Ok((cookie_jar, destination))
}

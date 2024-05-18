//! This module implements authentication primitives.

use std::convert::Infallible;

use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::request::Parts,
    response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
    Extension, RequestExt,
};
use axum_extra::{
    extract::{
        cookie::{self, Cookie, SameSite},
        PrivateCookieJar,
    },
    headers::UserAgent,
    TypedHeader,
};

use crate::domain::{
    database::Database,
    sessions::find_session_by_id,
    users::{find_user_by_id, User},
    Id,
};

use super::{Hx, Redirect, State};

const COOKIE_NAME: &str = "x-pluto-session";

/// A authentication/authorization principal.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Auth(pub User);

/// Creates a session.
#[derive(Debug, Clone)]
pub struct CreateAuth(pub cookie::Key, pub Id);

/// Deletes a session.
#[derive(Debug, Clone)]
pub struct DeleteAuth(pub cookie::Key);

impl Auth {
    pub async fn try_from_request(state: &State, request: &mut Request) -> Option<Self> {
        let jar: PrivateCookieJar = request.extract_parts_with_state(state).await.ok()?;
        let session: Id = jar.get(COOKIE_NAME)?.value_trimmed().try_into().ok()?;
        let agent: TypedHeader<UserAgent> = request.extract_parts().await.ok()?;

        let mut transaction = state.database.begin().await.ok()?;
        let session = find_session_by_id(&mut transaction, session)
            .await
            .ok()
            // filter based on whether the agents match
            .filter(|v| v.agent == agent.0.as_str())?;
        let user = find_user_by_id(&mut transaction, session.user_id)
            .await
            .ok()?;

        Some(Self(user))
    }
}

#[async_trait]
impl FromRequestParts<State> for Auth {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &super::State,
    ) -> Result<Self, Self::Rejection> {
        let hx = Hx::from_request_parts(parts, state).await.unwrap();

        Extension::<Auth>::from_request_parts(parts, state)
            .await
            .map(|v| v.0)
            .map_err(|_| Redirect::see_other(hx, "/login").into_response())
    }
}

impl IntoResponseParts for CreateAuth {
    type Error = Infallible;

    /// Set parts of the response
    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        let cookie = Cookie::build((COOKIE_NAME, self.1.to_string()))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Strict)
            .permanent()
            .build();

        PrivateCookieJar::new(self.0)
            .add(cookie)
            .into_response_parts(res)
    }
}

impl IntoResponseParts for DeleteAuth {
    type Error = Infallible;

    /// Set parts of the response
    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        let cookie = Cookie::build(COOKIE_NAME)
            .path("/")
            .http_only(true)
            .same_site(SameSite::Strict)
            .removal()
            .build();

        PrivateCookieJar::new(self.0)
            .add(cookie)
            .into_response_parts(res)
    }
}

//! This module implements authentication primitives.

use std::convert::Infallible;

use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::request::Parts,
    response::{IntoResponse, IntoResponseParts, Redirect, Response, ResponseParts},
    Extension, RequestExt,
};
use axum_extra::{
    extract::{
        cookie::{Cookie, SameSite},
        PrivateCookieJar,
    },
    headers::UserAgent,
    TypedHeader,
};

use crate::domain::{
    authorization::{authorize_by_session, force_authorize_by_email},
    database::{Database, Transaction},
    Id, Session, User,
};

use super::GlobalState;

const COOKIE_NAME: &str = "x-pluto-session";

/// A authorization principal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Auth {
    pub user: User,
    pub session: Option<Session>,
}

/// Creates a session.
#[derive(Debug, Clone)]
pub struct CreateAuth(pub Session);

/// Deletes a session.
#[derive(Debug, Clone)]
pub struct DeleteAuth;

impl Auth {
    #[tracing::instrument(skip(state, request))]
    pub async fn try_from_request_header(
        state: &GlobalState,
        request: &mut Request,
    ) -> Option<Self> {
        let header = state.cfg.authorization_header.as_ref()?;
        let email = request.headers().get(header)?.to_str().ok()?;

        let mut tx = state.database.begin().await.ok()?;
        let user = force_authorize_by_email(&mut tx, email).await.ok()?;

        tx.commit().await.ok()?;
        Some(Self {
            user,
            session: None,
        })
    }

    #[tracing::instrument(skip(state, request))]
    pub async fn try_from_request_cookies(
        state: &GlobalState,
        request: &mut Request,
    ) -> Option<Self> {
        let jar: PrivateCookieJar = request.extract_parts_with_state(state).await.ok()?;
        let session: Id = jar.get(COOKIE_NAME)?.value_trimmed().try_into().ok()?;
        let agent: TypedHeader<UserAgent> = request.extract_parts().await.ok()?;

        let mut tx = state.database.begin().await.ok()?;
        let (user, session) = authorize_by_session(&mut tx, session, agent.as_str())
            .await
            .ok()?;

        Some(Self {
            user,
            session: Some(session),
        })
    }
}

#[async_trait]
impl FromRequestParts<GlobalState> for Auth {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &super::GlobalState,
    ) -> Result<Self, Self::Rejection> {
        Extension::<Auth>::from_request_parts(parts, state)
            .await
            .map(|v| v.0)
            .map_err(|_| Redirect::to("/login").into_response())
    }
}

impl CreateAuth {
    pub fn from_response(resp: &Response) -> Option<&Self> {
        resp.extensions().get()
    }

    pub fn to_response_parts(&self, state: &GlobalState) -> impl IntoResponseParts {
        let cookie = Cookie::build((COOKIE_NAME, self.0.id.to_string()))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Strict)
            .permanent()
            .build();

        PrivateCookieJar::new(state.cookie_key.clone()).add(cookie)
    }
}

impl IntoResponseParts for CreateAuth {
    type Error = Infallible;

    /// Set parts of the response
    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.extensions_mut().insert(self);
        Ok(res)
    }
}

impl DeleteAuth {
    pub fn from_response(resp: &Response) -> Option<&Self> {
        resp.extensions().get()
    }

    pub fn to_response_parts(&self, state: &GlobalState) -> impl IntoResponseParts {
        let cookie = Cookie::build(COOKIE_NAME)
            .path("/")
            .http_only(true)
            .same_site(SameSite::Strict)
            .removal()
            .build();

        PrivateCookieJar::new(state.cookie_key.clone()).add(cookie)
    }
}

impl IntoResponseParts for DeleteAuth {
    type Error = Infallible;

    /// Set parts of the response
    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.extensions_mut().insert(self);
        Ok(res)
    }
}

//! This module implements authentication primitives.

use std::convert::Infallible;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, IntoResponseParts, Redirect, Response, ResponseParts},
};
use axum_extra::extract::{
    cookie::{self, Cookie, SameSite},
    PrivateCookieJar,
};

use crate::domain::identifier::Id;

use super::State;

const COOKIE_NAME: &str = "x-pluto-session";

/// A authentication/authorization principal.
pub struct Auth(pub Id);

/// Creates a session.
pub struct CreateAuth(pub cookie::Key, pub Id);

/// Deletes a session.
pub struct DeleteAuth(pub cookie::Key);

#[async_trait]
impl FromRequestParts<State> for Auth {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &super::State,
    ) -> Result<Self, Self::Rejection> {
        fn construct_error() -> Response {
            Redirect::to("/login").into_response()
        }

        let jar: PrivateCookieJar = PrivateCookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| construct_error())?;
        let session = jar.get(COOKIE_NAME).ok_or_else(construct_error)?;
        let session = session
            .value_trimmed()
            .try_into()
            .map_err(|_| construct_error())?;
        Ok(Auth(session))
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

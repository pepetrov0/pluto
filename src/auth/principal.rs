//! Implements extraction of auth principal in axum handlers

use axum::{
    async_trait, extract::FromRequestParts, http::request::Parts, response::Redirect,
    RequestPartsExt,
};

use crate::{user::{User, UserRepository}, AppState};

use super::session::Session;

/// Guard for redirecting authenticated user away from protected handlers
pub struct NoAuthPrincipal;

/// Represent an authentication/authorization principal
#[derive(Debug)]
pub struct AuthPrincipal(pub User);

#[async_trait]
impl FromRequestParts<AppState> for NoAuthPrincipal {
    type Rejection = Redirect;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        AuthPrincipal::from_request_parts(parts, state)
            .await
            .err()
            .map(|_| Self)
            .ok_or_else(|| Redirect::to("/"))
    }
}

#[async_trait]
impl FromRequestParts<AppState> for AuthPrincipal {
    type Rejection = Redirect;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user = parts
            .extract_with_state::<Session, _>(state)
            .await
            .map(|v| v.usr)
            .map_err(|_| Redirect::to("/login"))?;

        state
            .clone()
            .database
            .find_user(&user)
            .await
            .map(AuthPrincipal)
            .ok_or(Redirect::to("/login"))
    }
}

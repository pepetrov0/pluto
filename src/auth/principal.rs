//! Implements extraction of auth principal in axum handlers

use axum::{
    async_trait, extract::FromRequestParts, http::request::Parts, Extension,
    RequestPartsExt,
};

use crate::{errors::AppError, user::User, AppState};

use super::session::Session;

/// Guard for redirecting authenticated user away from protected handlers
pub struct NoAuthPrincipal;

/// Represent an authentication/authorization principal
#[derive(Debug)]
pub struct AuthPrincipal(pub User);

#[async_trait]
impl FromRequestParts<AppState> for NoAuthPrincipal {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        AuthPrincipal::from_request_parts(parts, state)
            .await
            .err()
            .map(|_| Self)
            .ok_or_else(|| AppError::NotAllowedHere)
    }
}

#[async_trait]
impl FromRequestParts<AppState> for AuthPrincipal {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user = parts
            .extract_with_state::<Extension<Session>, _>(state)
            .await
            .map(|v| v.0.user)
            .map_err(|_| AppError::Unauthorized)?;

        state
            .user_repository
            .find_user(&user)
            .await
            .map(|v| AuthPrincipal(v))
            .ok_or(AppError::Unauthorized)
    }
}

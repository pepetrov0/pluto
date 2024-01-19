//! Implements extraction of auth principal in axum handlers

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Extension, RequestPartsExt,
};

use crate::{user::User, AppState};

use super::session::Session;

/// Represent an authentication/authorization principal
pub struct AuthPrincipal(pub User);

#[async_trait]
impl FromRequestParts<AppState> for AuthPrincipal {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user = parts
            .extract_with_state::<Extension<Session>, _>(state)
            .await
            .map(|v| v.0.user)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        state
            .user_repository
            .find_user(&user)
            .await
            .map(|v| AuthPrincipal(v))
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

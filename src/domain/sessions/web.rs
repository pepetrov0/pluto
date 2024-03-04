use super::Session;
use crate::AppState;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, Extension};

#[async_trait]
impl FromRequestParts<AppState> for Session {
    type Rejection = <Extension<Session> as FromRequestParts<AppState>>::Rejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Extension::<Session>::from_request_parts(parts, state)
            .await
            .map(|v| v.0)
    }
}

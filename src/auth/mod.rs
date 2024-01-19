//! Implements authentication/authorization

use axum::Router;

use crate::AppState;

pub mod password_hasher;
pub mod principal;
pub mod session;
pub mod session_providers;
mod source_providers;
mod logout;

pub fn router() -> Router<AppState> {
    Router::new().merge(source_providers::router()).merge(logout::router())
}

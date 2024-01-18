//! Enables all authentication/authorization functionalities

use axum::Router;

use crate::AppState;

mod local;
pub mod password_hasher;

pub fn router() -> Router<AppState> {
    Router::new().merge(local::router())
}

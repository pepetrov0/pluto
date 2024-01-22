//! Implements the accounts feature

use axum::Router;

use crate::AppState;

pub mod component;

pub fn router() -> Router<AppState> {
    Router::new()
}

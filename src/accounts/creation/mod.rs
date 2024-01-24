//! Implements account creation feature

use axum::{routing, Router};

use crate::AppState;

mod api;
pub(super) mod error;
mod page;

pub(super) const CSRF_TOKEN_USAGE: &str = "new-account";

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/new-account", routing::get(page::handler))
        .route("/new-account", routing::post(api::handler))
}

//! Implements asset creation

use axum::{routing, Router};

use crate::AppState;

mod api;
pub(super) mod error;
mod page;

pub(super) const CSRF_TOKEN_USAGE: &str = "new-asset";

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/new-asset", routing::get(page::handler))
        .route("/new-asset", routing::post(api::handler))
}

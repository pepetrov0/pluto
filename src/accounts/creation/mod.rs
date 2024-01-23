//! Implements account creation feature

use axum::{routing, Router};

use crate::AppState;

mod page;

pub fn router() -> Router<AppState> {
    Router::new().route("/new-account", routing::get(page::handler))
}

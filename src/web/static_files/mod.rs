use axum::{routing, Router};

use super::core::State;

mod action;
mod responder;

#[tracing::instrument]
pub fn router() -> Router<State> {
    tracing::debug!("constructing router (static_files)..");
    Router::new().route("/static/:path", routing::get(action::invoke))
}

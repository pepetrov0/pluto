use axum::{routing, Router};

use super::_core::State;

mod action;
mod responder;

#[tracing::instrument]
pub fn router() -> Router<State> {
    tracing::debug!("constructing router (index)..");
    Router::new().route("/", routing::get(action::invoke))
}

use axum::{routing, Router};

use super::_core::State;

mod action;
mod core;
mod responder;

pub use core::url;

#[tracing::instrument]
pub fn router() -> Router<State> {
    tracing::debug!("constructing router (static_files)..");
    Router::new().route("/static/:path", routing::get(action::invoke))
}

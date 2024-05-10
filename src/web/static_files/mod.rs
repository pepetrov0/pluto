use axum::{routing, Router};
use super::State;

mod core;
mod action;
mod responder;

pub use core::url;

#[tracing::instrument]
pub fn router() -> Router<State> {
    tracing::debug!("constructing router (static_files)..");
    Router::new().route("/static/:path", routing::get(action::invoke))
}

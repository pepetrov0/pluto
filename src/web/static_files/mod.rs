use axum::{routing, Router};

mod action;
mod responder;

#[tracing::instrument]
pub fn router() -> Router {
    tracing::debug!("constructing router (static_files)..");
    Router::new().route("/static/:path", routing::get(action::invoke))
}


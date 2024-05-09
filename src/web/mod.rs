//! This module implements the web interface of the application.

use axum::{routing, Router};
use tower_http::compression::CompressionLayer;

mod core;
mod static_files;

#[cfg(test)]
mod tests;

/// Constructs the primary router to be used for serving the application.
#[tracing::instrument]
pub fn router() -> Router {
    tracing::debug!("constructing router..");
    Router::new()
        .route("/health", routing::any(()))
        .merge(static_files::router())
        .layer(axum::middleware::from_fn(core::cache_control::layer))
        .layer(CompressionLayer::new())
}

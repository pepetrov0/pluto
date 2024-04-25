//! This module implements the web interface of the application.

use axum::{routing, Router};
use tower_http::compression::CompressionLayer;

/// Constructs the primary router to be used for serving the application.
#[tracing::instrument]
pub fn router() -> Router {
    tracing::debug!("constructing router..");
    Router::new()
        .route("/health", routing::any(()))
        .layer(CompressionLayer::new())
}

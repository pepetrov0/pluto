//! This module implements the web interface of the application.

use axum::Router;

/// Constructs the primary router to be used for serving the application.
#[tracing::instrument]
pub fn router() -> Router {
    tracing::debug!("constructing router..");
    Router::new()
}

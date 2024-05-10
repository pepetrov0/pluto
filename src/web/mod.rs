//! This module implements the web interface of the application.

use axum::{routing, Router};
use tower_http::compression::CompressionLayer;

use crate::domain::database::AnyDatabase;

mod components;
mod core;

mod static_files;

mod index;

#[cfg(test)]
mod tests;

/// Constructs the primary router to be used for serving the application.
#[tracing::instrument(skip(database))]
pub fn router(database: AnyDatabase) -> Router<()> {
    tracing::debug!("constructing router..");
    Router::new()
        .merge(index::router())
        .route("/health", routing::any(()))
        .merge(static_files::router())
        .layer(axum::middleware::from_fn(core::cache_control::layer))
        .layer(CompressionLayer::new())
        .with_state(core::State { database })
}

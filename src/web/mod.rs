//! This module implements the web interface of the application.

use axum::{routing, Router};
use axum_extra::extract::cookie;
use tower_http::compression::CompressionLayer;

use crate::domain::database::AnyDatabase;

mod _components;
mod _core;

mod index;
mod static_files;

pub use static_files::url as static_file_url;

#[cfg(test)]
mod tests;

/// Constructs the primary router to be used for serving the application.
#[tracing::instrument(skip(database))]
pub fn router(database: AnyDatabase, key: cookie::Key) -> Router<()> {
    tracing::debug!("constructing router..");
    Router::new()
        .merge(index::router())
        .route("/health", routing::any(()))
        .merge(static_files::router())
        .layer(axum::middleware::from_fn(
            _core::middleware::cache_control_layer,
        ))
        .layer(CompressionLayer::new())
        .with_state(_core::State { database, key })
}

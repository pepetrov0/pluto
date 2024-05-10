//! This module implements the web interface of the application.

use axum::{
    extract::Request,
    http::header,
    middleware::Next,
    response::{IntoResponse, Response},
    routing, Router,
};
use axum_extra::{headers::CacheControl, TypedHeader};
use tower_http::compression::CompressionLayer;

use crate::domain::database::AnyDatabase;

mod _components;

mod index;
mod static_files;

pub use static_files::url as static_file_url;

#[cfg(test)]
mod tests;

/// State shared between all databases.
#[derive(Clone)]
struct State {
    #[allow(unused)]
    pub database: AnyDatabase,
}

/// A middleware layer that adds cache control header to all responses that do not have one.
async fn cache_control_layer(req: Request, next: Next) -> Response {
    let response = next.run(req).await;

    match response.headers().contains_key(header::CACHE_CONTROL) {
        true => response,
        false => (TypedHeader(CacheControl::new().with_no_store()), response).into_response(),
    }
}

/// Constructs the primary router to be used for serving the application.
#[tracing::instrument(skip(database))]
pub fn router(database: AnyDatabase) -> Router<()> {
    tracing::debug!("constructing router..");
    Router::new()
        .merge(index::router())
        .route("/health", routing::any(()))
        .merge(static_files::router())
        .layer(axum::middleware::from_fn(cache_control_layer))
        .layer(CompressionLayer::new())
        .with_state(State { database })
}

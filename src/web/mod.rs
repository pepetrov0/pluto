//! This module implements the web interface of the application.

use axum::{routing, Router};
use axum_extra::extract::cookie;
use tower_http::compression::CompressionLayer;

use crate::domain::database::AnyDatabase;

mod _components;
mod _core;

mod get_static_file;
mod register;
mod show_dashboard;
mod show_login;
mod show_register;
mod logout;

#[cfg(test)]
mod tests;

/// Constructs the primary router to be used for serving the application.
#[tracing::instrument(skip(database))]
pub fn router(database: AnyDatabase, key: cookie::Key) -> Router<()> {
    let state = _core::GlobalState { database, key };

    // middleware
    let auth_layer =
        axum::middleware::from_fn_with_state(state.clone(), _core::middleware::auth_layer);
    let cache_control_layer = axum::middleware::from_fn(_core::middleware::cache_control_layer);

    Router::new()
        // ops
        .route("/health", routing::any(()))
        // auth
        .merge(show_login::router())
        .merge(show_register::router())
        .merge(register::router())
        .merge(logout::router())
        // home
        .merge(show_dashboard::router())
        // other
        .merge(get_static_file::router())
        .layer(auth_layer)
        .layer(cache_control_layer)
        .layer(CompressionLayer::new())
        .with_state(state)
}

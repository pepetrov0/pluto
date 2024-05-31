//! This module implements the web interface of the application.

use axum::{routing, Router};
use axum_extra::extract::cookie;
use tower_http::compression::CompressionLayer;

use crate::domain::{database::AnyDatabase, Configuration};

mod _components;
mod _core;

mod change_email;
mod change_password;
mod get_static_file;
mod login;
mod logout;
mod register;
mod show_dashboard;
mod show_login;
mod show_profile;
mod show_register;
mod validate_change_email;
mod validate_change_password;
mod validate_register;

#[cfg(test)]
mod tests;

/// Constructs the primary router to be used for serving the application.
#[tracing::instrument(skip(database))]
pub fn router(cfg: Configuration, database: AnyDatabase, cookie_key: cookie::Key) -> Router<()> {
    let state = _core::GlobalState {
        cfg,
        database,
        cookie_key,
    };

    // middleware
    let header_authorization_layer = axum::middleware::from_fn_with_state(
        state.clone(),
        _core::middleware::header_authorization_layer,
    );
    let cookie_authorization_layer = axum::middleware::from_fn_with_state(
        state.clone(),
        _core::middleware::cookie_authorization_layer,
    );
    let cache_control_layer = axum::middleware::from_fn(_core::middleware::cache_control_layer);
    let redirects_layer = axum::middleware::from_fn(_core::middleware::redirects_layer);

    Router::new()
        // ops
        .route("/health", routing::any(()))
        // auth
        .merge(show_login::router())
        .merge(show_register::router())
        .merge(validate_register::router())
        .merge(login::router())
        .merge(register::router())
        .merge(logout::router())
        // profile
        .merge(show_profile::router())
        .merge(validate_change_email::router())
        .merge(validate_change_password::router())
        .merge(change_email::router())
        .merge(change_password::router())
        // home
        .merge(show_dashboard::router())
        // other
        .merge(get_static_file::router())
        .layer(cookie_authorization_layer)
        .layer(header_authorization_layer)
        .layer(redirects_layer)
        .layer(cache_control_layer)
        .layer(CompressionLayer::new())
        .with_state(state)
}

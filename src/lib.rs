use core::web::{cache_policy, compression, content_security_policy};
use std::sync::Arc;

use auth::password_hasher::PasswordHasher;
use axum::{extract::FromRef, middleware, Router};
use axum_extra::extract::cookie::Key;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

pub mod core;
pub mod domain;
pub mod presentation;

// old modules
pub mod accounts;
pub mod assets;
pub mod auth;
pub mod transactions;

pub const BUILD_ID: &str = env!("PLUTO_BUILD_ID");

pub const DATE_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M";
pub const DATE_TIME_FORMAT_NICE: &str = "%d %b %Y @ %H:%M";
pub const DEFAULT_PAGE_SIZE: i64 = 25;
pub const PAGE_SIZE_LIMITS: (i64, i64) = (10, 100);

#[derive(Clone)]
pub struct AppState {
    pub configuration: core::Configuration,
    pub cookie_jar_key: Key,
    pub database: PgPool,
    pub password_hasher: Arc<dyn PasswordHasher>,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_jar_key.clone()
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        // healthcheck
        .merge(presentation::healthcheck::router())
        // auth router
        .merge(auth::router())
        // dashboard
        .merge(presentation::dashboard::router())
        // assets
        .merge(assets::router())
        // acounts
        .merge(accounts::router())
        // transactions
        .merge(transactions::router())
        // static files
        .merge(presentation::static_files::router())
        // auth middlewares
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::session_providers::cookie::middleware,
        ))
        .layer(compression::default())
        .layer(middleware::from_fn(content_security_policy::middleware))
        .layer(middleware::from_fn(cache_policy::middleware))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

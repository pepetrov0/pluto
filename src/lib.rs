use std::sync::Arc;

use assets::AssetRepository;
use auth::{password_hasher::PasswordHasher, session::SessionRepository};
use axum::{extract::FromRef, middleware, Router};
use axum_extra::extract::cookie::Key;
use database::Pool;
use tower_http::trace::TraceLayer;
use user::UserRepository;

// components
pub mod assets;
pub mod compression;
pub mod config;
pub mod content_security_policy;
pub mod database;
pub mod imkvs;
pub mod templates;
pub mod user;
pub mod validation;

// features
pub mod auth;
pub mod dashboard;
pub mod healthcheck;
pub mod shutdown;
pub mod static_files;

#[derive(Clone)]
pub struct AppState {
    pub configuration: config::Configuration,
    pub cookie_jar_key: Key,
    pub database: Arc<dyn Pool>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub user_repository: Arc<dyn UserRepository>,
    pub session_repository: Arc<dyn SessionRepository>,
    pub assets_repository: Arc<dyn AssetRepository>,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_jar_key.clone()
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        // healthcheck
        .merge(healthcheck::router())
        // auth router
        .merge(auth::router())
        // dashboard
        .merge(dashboard::router())
        // static files
        .merge(static_files::router())
        // auth middlewares
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::session_providers::cookie::middleware,
        ))
        .layer(compression::default())
        .layer(middleware::from_fn(content_security_policy::middleware))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

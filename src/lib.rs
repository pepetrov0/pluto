use std::sync::Arc;

use auth::{password_hasher::PasswordHasher, session::SessionRepository};
use axum::{extract::FromRef, middleware, Router};
use axum_extra::extract::cookie::Key;
use database::Pool;
use tower_http::{services::ServeDir, trace::TraceLayer};
use user::UserRepository;

// components
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

#[derive(Clone)]
pub struct AppState {
    pub configuration: config::Configuration,
    pub cookie_jar_key: Key,
    pub database: Arc<dyn Pool>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub user_repository: Arc<dyn UserRepository>,
    pub session_repository: Arc<dyn SessionRepository>,
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
        .fallback_service( ServeDir::new("static"))
        // auth middlewares
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::session_providers::cookie::middleware,
        ))
        .layer(middleware::from_fn(content_security_policy::middleware))
        .layer(compression::default())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

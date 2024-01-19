use std::sync::Arc;

use auth::{password_hasher::PasswordHasher, session::SessionRepository};
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use database::Pool;
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
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_jar_key.clone()
    }
}

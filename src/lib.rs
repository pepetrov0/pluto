use std::sync::Arc;

use auth::password_hasher::PasswordHasher;
use database::Pool;
use user::UserRepository;

// components
pub mod config;
pub mod database;
pub mod user;

// features
pub mod auth;
pub mod healthcheck;
pub mod shutdown;
pub mod static_files;

#[derive(Clone)]
pub struct AppState {
    pub configuration: config::Configuration,
    pub database: Arc<dyn Pool>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub user_repository: Arc<dyn UserRepository>,
}

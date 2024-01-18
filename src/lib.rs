use std::sync::Arc;

use database::Database;
use user::UserRepository;

// components
pub mod config;
pub mod database;
pub mod shutdown;
pub mod user;

// features
pub mod local_auth;
pub mod static_files;
pub mod status;

#[derive(Clone)]
pub struct AppState {
    pub configuration: config::Configuration,
    pub database: Arc<dyn Database>,
    pub user_repository: Arc<dyn UserRepository>,
}

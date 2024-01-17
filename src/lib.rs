use std::sync::Arc;

use database::Database;
use user::UserRepository;

// core
pub mod config;
pub mod database;
pub mod shutdown;
pub mod static_files;

// components
pub mod user;

// api
pub mod ping_api;
pub mod register_api;

// pages
pub mod login_page;
pub mod register_email_taken_page;
pub mod register_page;

#[derive(Clone)]
pub struct AppState {
    pub configuration: config::Configuration,
    pub database: Arc<dyn Database>,
    pub user_repository: Arc<dyn UserRepository>,
}

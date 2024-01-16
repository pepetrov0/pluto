pub mod config;
pub mod database;
pub mod shutdown;
pub mod static_files;

// api
pub mod ping;

// pages
pub mod login_page;
pub mod register_page;

#[derive(Clone)]
pub struct RouterState {
    pub configuration: config::Configuration,
    pub database: database::Database,
}

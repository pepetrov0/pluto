pub mod config;
pub mod database;
pub mod ping;
pub mod shutdown;
pub mod static_files;

#[derive(Clone)]
pub struct RouterState {
    pub configuration: config::Configuration,
    pub database: database::Database,
}

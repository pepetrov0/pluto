pub mod config;
pub mod database;
pub mod ping;
pub mod shutdown;

#[derive(Clone)]
pub struct RouterState {
    pub configuration: config::Configuration,
    pub database: database::Database,
}

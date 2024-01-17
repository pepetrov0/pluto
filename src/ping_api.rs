use axum::Json;

use crate::{config::Configuration, database::Database};

#[derive(serde::Serialize)]
pub struct Ping {
    pub message: String,
    pub configuration: Configuration,
    pub database_status: DatabaseStatus,
}

#[derive(serde::Serialize)]
pub struct DatabaseStatus {
    pub open: bool,
    pub size: u32,
    pub idle: usize,
}

pub async fn handler(configuration: Configuration, database: Database) -> Json<Ping> {
    Json(Ping {
        message: "pong".to_string(),
        configuration,
        database_status: DatabaseStatus {
            open: !database.is_closed(),
            size: database.size(),
            idle: database.num_idle(),
        },
    })
}

use axum::{extract::State, Json};

use crate::{config::Configuration, AppState};

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
    pub idle: u32,
}

pub async fn handler(State(state): State<AppState>) -> Json<Ping> {
    Json(Ping {
        message: "pong".to_string(),
        configuration: state.configuration,
        database_status: DatabaseStatus {
            open: state.database.is_open(),
            size: state.database.size(),
            idle: state.database.num_idle(),
        },
    })
}

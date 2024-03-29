use axum::{extract::State, routing, Json, Router};

use crate::{core::Configuration, AppState};

#[derive(serde::Serialize)]
struct Ping {
    pub message: String,
    pub configuration: Configuration,
    pub database_status: DatabaseStatus,
}

#[derive(serde::Serialize)]
struct DatabaseStatus {
    pub open: bool,
    pub size: u32,
    pub idle: usize,
}

async fn handler(State(state): State<AppState>) -> Json<Ping> {
    Json(Ping {
        message: "pong".to_string(),
        configuration: state.configuration,
        database_status: DatabaseStatus {
            open: !state.database.is_closed(),
            size: state.database.size(),
            idle: state.database.num_idle(),
        },
    })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/ping", routing::get(handler))
}

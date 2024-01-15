use axum::{extract::State, Json};

use crate::config::Configuration;

#[derive(serde::Serialize)]
pub struct Ping {
    pub message: String,
    pub configuration: Configuration,
}

pub async fn handler(State(configuration): State<Configuration>) -> Json<Ping> {
    Json(Ping {
        message: "pong".to_string(),
        configuration,
    })
}

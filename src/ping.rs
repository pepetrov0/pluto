use axum::Json;

#[derive(serde::Serialize)]
pub struct Ping {
    pub message: String,
}

pub async fn handler() -> Json<Ping> {
    Json(Ping {
        message: "pong".to_string(),
    })
}

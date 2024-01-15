use axum::{routing, Router};

mod ping;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/ping", routing::get(ping::handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

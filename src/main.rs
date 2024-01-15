use axum::{routing, Router};
use pluto::{ping, shutdown};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let app = Router::new()
        .route("/api/ping", routing::get(ping::handler))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::signal())
        .await
        .unwrap();

    tracing::info!("shutting down..");
}

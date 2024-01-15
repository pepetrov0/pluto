use axum::{routing, Router};
use pluto::{config::Configuration, ping, shutdown};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // load in configuration
    let config = Configuration::read().expect("could not load in configuration");

    // initialize router
    let router = Router::new()
        .route("/api/ping", routing::get(ping::handler))
        .layer(TraceLayer::new_for_http())
        .with_state(config);

    // initialize listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    // serve on listener
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown::signal())
        .await
        .unwrap();

    tracing::info!("shutting down..");
}

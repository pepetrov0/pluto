use std::sync::Arc;

use argon2::Argon2;
use axum::Router;
use pluto::{config::Configuration, database, shutdown, static_files};
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

    // connect to database
    let database = Arc::new(
        database::connect(&config)
            .await
            .expect("could not connect to database"),
    );

    let state = pluto::AppState {
        configuration: config.clone(),
        database: database.clone(),
        password_hasher: Arc::new(Argon2::default()),
        user_repository: database.clone(),
    };

    // initialize router
    let router = Router::new()
        // healthcheck
        .merge(pluto::healthcheck::router())
        // static files
        .merge(static_files::router())
        // trace and state
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // initialize listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    // serve on listener
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown::signal())
        .await
        .unwrap();

    database.close().await;
    tracing::info!("shutting down..");
}

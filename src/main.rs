use std::sync::Arc;

use argon2::Argon2;
use axum_extra::extract::cookie::Key;
use pluto::{config::Configuration, database, imkvs::InMemoryKeyValueStore, shutdown};

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
        database::connect_to_postgres(&config)
            .await
            .expect("could not connect to database"),
    );

    let state = pluto::AppState {
        configuration: config.clone(),
        cookie_jar_key: config
            .cookie_jar_secret
            .map(|v| Key::from(v.as_bytes()))
            .unwrap_or_else(|| Key::generate()),
        database: database.clone(),
        password_hasher: Arc::new(Argon2::default()),
        user_repository: database.clone(),
        session_repository: Arc::new(InMemoryKeyValueStore::default()),
    };

    // initialize router
    let router = pluto::router(state);

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

use std::sync::Arc;

use argon2::Argon2;
use axum::routing;
use axum_extra::extract::cookie::Key;
use pluto::{
    config::{Configuration, SessionDriver},
    database,
    imkvs::InMemoryKeyValueStore,
    shutdown,
};

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

    // create password hasher
    let password_hasher = Arc::new(Argon2::default());

    let state = pluto::AppState {
        configuration: config.clone(),
        cookie_jar_key: Key::from(config.secret.as_bytes()),
        database: database.clone(),
        password_hasher,
        csrf_token_repository: Arc::new(InMemoryKeyValueStore::default()),
        user_repository: database.clone(),
        session_repository: match config.session_driver {
            SessionDriver::InMemory => Arc::new(InMemoryKeyValueStore::default()),
            SessionDriver::Database => database.clone(),
        },
        asset_repository: database.clone(),
        account_repository: database.clone(),
        account_ownership_repository: database.clone(),
        transaction_repository: database.clone(),
        entry_repository: database.clone(),
    };

    // initialize router
    let router = pluto::router(state).route(
        "/report",
        routing::any(|body: String| async move {
            println!("{:?}", body);
        }),
    );

    // initialize listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    // serve on listener
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown::signal())
        .await
        .unwrap();

    database.close().await;
    tracing::info!("shutting down..");
}

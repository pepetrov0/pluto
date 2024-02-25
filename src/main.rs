use std::sync::Arc;

use argon2::Argon2;
use axum_extra::extract::cookie::Key;
use pluto::{core::database, core::Configuration};

#[tokio::main]
async fn main() {
    // initialize tracing
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::TRACE)
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::WARN)
        .init();

    // load in configuration
    let config = Configuration::read().expect("could not load in configuration");

    // connect to database
    let database = database::connect_to_postgres(&config)
        .await
        .expect("could not connect to database");

    // create password hasher
    let password_hasher = Arc::new(Argon2::default());

    let state = pluto::AppState {
        configuration: config.clone(),
        cookie_jar_key: Key::from(config.secret.as_bytes()),
        database: database.clone(),
        password_hasher,
    };

    // initialize router
    let router = pluto::router(state);

    // initialize listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:46963")
        .await
        .unwrap();

    // serve on listener
    tracing::info!("listening on :46963");
    axum::serve(listener, router)
        .with_graceful_shutdown(pluto::core::shutdown::signal())
        .await
        .unwrap();

    database.close().await;
    tracing::info!("shutting down..");
}

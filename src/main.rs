use std::sync::Arc;

use argon2::Argon2;
use axum::{middleware, Router, routing};
use axum_extra::extract::cookie::Key;
use pluto::{
    auth::{self, principal::AuthPrincipal}, config::Configuration, content_security_policy, database, imkvs::InMemoryKeyValueStore,
    shutdown, static_files,
};
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
    let router = Router::new()
        // healthcheck
        .merge(pluto::healthcheck::router())
        // auth router
        .merge(auth::router())
        // static files
        .merge(static_files::router())
        // testing
        .route("/me", routing::get(|user: Option<AuthPrincipal>| async move {
            format!("{:?}", user)
        }))
        // auth middlewares
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::session_providers::cookie::middleware,
        ))
        // trace and state
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(content_security_policy::middleware))
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

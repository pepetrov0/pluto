use pluto::{
    domain::{
        database::{AnyDatabase, Database},
        keys, Configuration,
    },
    web,
};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    let log_filter = EnvFilter::from_default_env().add_directive("pluto=trace".parse().unwrap());

    #[cfg(not(debug_assertions))]
    let log_filter = EnvFilter::from_default_env().add_directive("pluto=info".parse().unwrap());

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(log_filter)
        .init();

    let config = Configuration::try_load().unwrap_or_default();
    let database = AnyDatabase::connect_with_configuration(&config)
        .await
        .unwrap();
    let router = web::router(config.clone(), database.clone(), keys::cookie_key(&config));
    let listener = TcpListener::bind("0.0.0.0:46963").await.unwrap();

    tracing::info!("listening on 0.0.0.0:46963");
    axum::serve(listener, router)
        .with_graceful_shutdown(pluto::domain::shutdown::signal())
        .await
        .unwrap();
    tracing::info!("shutting down..");
    database.close().await;
}

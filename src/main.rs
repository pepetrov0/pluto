use pluto::{domain::configuration::Configuration, web};
use tokio::net::TcpListener;
use tracing::Level;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(Level::DEBUG)
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::INFO)
        .init();

    let _config = Configuration::try_load().unwrap_or_default();
    let router = web::router();
    let listener = TcpListener::bind("0.0.0.0:46963").await.unwrap();

    tracing::info!("listening on 0.0.0.0:46963");
    axum::serve(listener, router).await.unwrap();
    tracing::info!("shutting down..");
}

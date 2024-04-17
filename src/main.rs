use pluto::{domain::configuration::Configuration, web};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config = Configuration::try_load().unwrap_or_default();
    println!("cfg: {config:?}");
    let router = web::router();
    let listener = TcpListener::bind("0.0.0.0:46963").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

use axum::{routing, Router};
use cached::proc_macro::cached;
use rust_embed::RustEmbed;
use sha2::{Digest, Sha256};

use super::core::State;

mod action;
mod responder;

#[derive(RustEmbed)]
#[folder = "static"]
pub struct Statics;

/// Returns the hash of a static file
#[cached(key = "String", convert = r#"{ format!("{file}") }"#)]
pub fn hash(file: &str) -> String {
    tracing::trace!("computing hash.. {file}");
    match Statics::get(file) {
        Some(file) => hex::encode(file.metadata.sha256_hash()),
        None => {
            let mut hash = Sha256::new();
            hash.update(file.as_bytes());
            hex::encode(hash.finalize())
        }
    }
}

/// Returns the URL of a static file
pub fn url(file: &str) -> String {
    format!("/static/{file}?hash={}", &self::hash(file)[..10])
}

#[tracing::instrument]
pub fn router() -> Router<State> {
    tracing::debug!("constructing router (static_files)..");
    Router::new().route("/static/:path", routing::get(action::invoke))
}

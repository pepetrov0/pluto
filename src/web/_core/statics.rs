//! Implements the core of the static file management.

use cached::proc_macro::cached;
use rust_embed::RustEmbed;
use sha2::{Digest, Sha256};

/// A structure of embeded static files
#[derive(RustEmbed)]
#[folder = "static"]
pub struct Statics;

/// Returns the hash of a static file
#[cached(key = "String", convert = r#"{ format!("{file}") }"#)]
fn hash(file: &str) -> String {
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

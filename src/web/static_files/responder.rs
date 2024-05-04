use std::time::Duration;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{CacheControl, ContentType},
    TypedHeader,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static"]
struct Statics;

const CACHE_MAX_AGE: u64 = 31_536_000; // in seconds, equals ~1 year

pub async fn invoke(path: &str) -> Response {
    match Statics::get(path) {
        Some(file) => {
            let content_type = mime_guess::from_path(path).first_or_octet_stream();
            let cache_control = CacheControl::new()
                .with_immutable()
                .with_public()
                .with_max_age(Duration::from_secs(CACHE_MAX_AGE));

            let data = file.data.to_vec();
            let content_type = TypedHeader(ContentType::from(content_type));
            let cache_control = TypedHeader(cache_control);

            (content_type, cache_control, data).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

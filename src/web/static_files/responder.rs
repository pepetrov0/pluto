use std::{str::FromStr, time::Duration};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{CacheControl, ContentType},
    TypedHeader,
};

const CACHE_MAX_AGE: u64 = 31_536_000; // in seconds, equals ~1 year

pub async fn invoke(path: &str) -> Response {
    match super::Statics::get(path) {
        Some(file) => {
            let content_type = ContentType::from_str(file.metadata.mimetype())
                .unwrap_or_else(|_| ContentType::octet_stream());
            let cache_control = CacheControl::new()
                .with_immutable()
                .with_public()
                .with_max_age(Duration::from_secs(CACHE_MAX_AGE));

            let content_type = TypedHeader(content_type);
            let cache_control = TypedHeader(cache_control);
            let data = file.data.to_vec();
            (content_type, cache_control, data).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

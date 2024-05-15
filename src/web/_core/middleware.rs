//! This module implements common middleware layers.

use axum::{
    extract::Request,
    http::header,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{headers::CacheControl, TypedHeader};

/// A middleware layer that adds cache control header to all responses that do not have one.
pub async fn cache_control_layer(req: Request, next: Next) -> Response {
    let response = next.run(req).await;

    match response.headers().contains_key(header::CACHE_CONTROL) {
        true => response,
        false => (TypedHeader(CacheControl::new().with_no_store()), response).into_response(),
    }
}

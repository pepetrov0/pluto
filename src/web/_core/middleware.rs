//! This module implements common middleware layers.

use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{headers::CacheControl, TypedHeader};

use super::Auth;

/// A middleware layer that adds cache control header to all responses that do not have one.
pub async fn cache_control_layer(req: Request, next: Next) -> Response {
    let response = next.run(req).await;

    match response.headers().contains_key(header::CACHE_CONTROL) {
        true => response,
        false => (TypedHeader(CacheControl::new().with_no_store()), response).into_response(),
    }
}

/// A middleware layer that tries to extract the authorization principle from the request
/// and attach it as an extension.
pub async fn auth_layer(
    State(state): State<super::State>,
    mut req: Request,
    next: Next,
) -> Response {
    if let Some(auth) = Auth::try_from_request(&state, &mut req).await {
        req.extensions_mut().insert(auth);
    }

    next.run(req).await
}

//! This module implements common middleware layers.

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    RequestExt,
};
use axum_extra::{headers::CacheControl, TypedHeader};

use super::{Auth, CreateAuth, DeleteAuth, Hx};

/// A middleware layer that adds cache control header to all responses that do not have one.
#[tracing::instrument(skip(req, next))]
pub async fn cache_control_layer(req: Request, next: Next) -> Response {
    let response = next.run(req).await;

    match response.headers().contains_key(header::CACHE_CONTROL) {
        true => response,
        false => (TypedHeader(CacheControl::new().with_no_store()), response).into_response(),
    }
}

/// A middleware layer that tries to extract the authorization principle from the header
/// and attach it as an extension.
#[tracing::instrument(skip(state, req, next))]
pub async fn header_authorization_layer(
    State(state): State<super::GlobalState>,
    mut req: Request,
    next: Next,
) -> Response {
    // extract auth from request only if it is missing form the request.
    if req.extensions().get::<Auth>().is_none() {
        if let Some(auth) = Auth::try_from_request_header(&state, &mut req).await {
            req.extensions_mut().insert(auth);
        }
    }

    next.run(req).await
}

/// A middleware layer that tries to extract the authorization principle from the cookies
/// and attach it as an extension.
#[tracing::instrument(skip(state, req, next))]
pub async fn cookie_authorization_layer(
    State(state): State<super::GlobalState>,
    mut req: Request,
    next: Next,
) -> Response {
    // extract auth from request only if it is missing form the request.
    if req.extensions().get::<Auth>().is_none() {
        if let Some(auth) = Auth::try_from_request_cookies(&state, &mut req).await {
            req.extensions_mut().insert(auth);
        }
    }

    let response = next.run(req).await;

    // cookie jars
    let create_jar = CreateAuth::from_response(&response).map(|v| v.to_response_parts(&state));

    // delete auth
    let delete_jar = DeleteAuth::from_response(&response).map(|v| v.to_response_parts(&state));

    (delete_jar, create_jar, response).into_response()
}

#[tracing::instrument(skip(req, next))]
pub async fn redirects_layer(mut req: Request, next: Next) -> Response {
    const HTMX_LOCATION_HEADER: &str = "HX-Location";

    // try and extract htmx headers
    let hx: Hx = req.extract_parts().await.unwrap();

    // run the rest of the chain
    let mut response = next.run(req).await;

    // if a htmx request header and status code is SEE_OTHER,
    // set status and HX-Location
    if hx.request && response.status() == StatusCode::SEE_OTHER {
        // set status
        *response.status_mut() = StatusCode::NO_CONTENT;

        // set headers
        if let Some(location) = response.headers_mut().remove(header::LOCATION) {
            response
                .headers_mut()
                .insert(HTMX_LOCATION_HEADER, location);
        }
    }

    response
}

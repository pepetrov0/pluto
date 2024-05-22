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
    State(state): State<super::GlobalState>,
    mut req: Request,
    next: Next,
) -> Response {
    // extract auth from request
    if let Some(auth) = Auth::try_from_request(&state, &mut req).await {
        req.extensions_mut().insert(auth);
    }

    let response = next.run(req).await;

    // cookie jars
    let create_jar = CreateAuth::from_response(&response).map(|v| v.to_response_parts(&state));

    // delete auth
    let delete_jar = DeleteAuth::from_response(&response).map(|v| v.to_response_parts(&state));

    (delete_jar, create_jar, response).into_response()
}

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

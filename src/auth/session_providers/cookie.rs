//! Implements a cookie session provider

use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::Response,
    RequestExt,
};
use axum_extra::extract::PrivateCookieJar;

use crate::{auth::session::Session, AppState};

const DEFAULT_SESSION_COOKIE_NAME: &'static str = "x-pluto-session";

/// Cookie session extraction middleware
pub async fn middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    // try restore the cookie session into the request
    restore(&state, &mut req).await;

    // call the chain
    next.run(req).await
}

async fn restore(state: &AppState, req: &mut Request<Body>) {
    // restore cookie session
    if req.extensions().get::<Session>().is_some() {
        return;
    }

    let session_cookie_name = state
        .configuration
        .session_cookie_name
        .as_ref()
        .map(|v| v.as_str())
        .unwrap_or_else(|| DEFAULT_SESSION_COOKIE_NAME);

    let cookie_jar = req
        .extract_parts_with_state::<PrivateCookieJar, _>(state)
        .await
        .expect("Infallible");

    let session_id = match cookie_jar.get(session_cookie_name) {
        Some(v) => v.value().to_owned(),
        None => return,
    };

    if let Some(session) = state.session_repository.find_session(&session_id).await {
        req.extensions_mut().insert(session);
    }
}

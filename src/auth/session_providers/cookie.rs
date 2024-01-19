//! Implements a cookie session provider

use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
    Extension,
};
use axum_extra::extract::{
    cookie::{Cookie, Key, SameSite},
    PrivateCookieJar,
};

use crate::{auth::session::Session, AppState};

const DEFAULT_SESSION_COOKIE_NAME: &'static str = "x-pluto-session";

/// An extension to set a session cookie
#[derive(Debug, Clone)]
pub struct SetCookieSession(pub Session);

/// Cookie session extraction middleware
pub async fn middleware(
    State(state): State<AppState>,
    jar: PrivateCookieJar<Key>,
    session: Option<Extension<Session>>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let session_cookie_name = state
        .configuration
        .session_cookie_name
        .clone()
        .unwrap_or_else(|| DEFAULT_SESSION_COOKIE_NAME.to_owned());

    // try restore the cookie session into the request
    if session.is_none() {
        if let Some(session) = restore(&session_cookie_name, &state, jar).await {
            req.extensions_mut().insert(session);
        }
    }

    // call the chain
    let response = next.run(req).await;

    match response.extensions().get::<SetCookieSession>().cloned() {
        Some(session) => store(session_cookie_name, state, response, session).await,
        None => response,
    }
}

async fn restore(
    session_cookie_name: &str,
    state: &AppState,
    jar: PrivateCookieJar<Key>,
) -> Option<Session> {
    let session_id = jar.get(session_cookie_name)?.value().to_owned();
    state.session_repository.find_session(&session_id).await
}

async fn store(
    session_cookie_name: String,
    state: AppState,
    res: Response,
    SetCookieSession(session): SetCookieSession,
) -> Response {
    let cookie = Cookie::build((session_cookie_name, session.id))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Strict)
        .permanent()
        .build();

    let jar = PrivateCookieJar::new(state.cookie_jar_key.clone()).add(cookie);
    (jar, res).into_response()
}

impl IntoResponseParts for SetCookieSession {
    type Error = <Extension<SetCookieSession> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        Extension(self).into_response_parts(res)
    }
}

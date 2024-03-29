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

use crate::{
    core::database::ReadonlyRepository,
    domain::{self, sessions::Session},
    AppState,
};

const DEFAULT_SESSION_COOKIE_NAME: &str = "x-pluto-session";

#[derive(Debug, Clone)]
pub struct SetCookieSession(pub Session);

#[derive(Debug, Clone)]
pub struct RemoveCookieSession;

pub async fn middleware(
    State(state): State<AppState>,
    mut jar: PrivateCookieJar<Key>,
    session: Option<Session>,
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
        if let Some(cookie) = jar.get(&session_cookie_name) {
            let session = cookie.value();
            let session = match ReadonlyRepository::from_pool(&state.database).await {
                Some(mut repository) => domain::sessions::find(&mut repository, session)
                    .await
                    .ok()
                    .flatten(),
                None => None,
            };
            if let Some(session) = session {
                req.extensions_mut().insert::<Session>(session);
            }
        }
    }

    // call the chain
    let response = next.run(req).await;

    if let Some(SetCookieSession(session)) = response.extensions().get::<SetCookieSession>() {
        let cookie = Cookie::build((session_cookie_name.clone(), session.id.clone()))
            .http_only(true)
            .path("/")
            .same_site(SameSite::Strict)
            .permanent()
            .build();
        jar = jar.add(cookie);
    }

    if response.extensions().get::<RemoveCookieSession>().is_some() {
        let cookie = Cookie::build(session_cookie_name)
            .http_only(true)
            .path("/")
            .same_site(SameSite::Strict)
            .permanent()
            .build();
        jar = jar.remove(cookie);
    }

    (jar, response).into_response()
}

impl IntoResponseParts for SetCookieSession {
    type Error = <Extension<SetCookieSession> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        Extension(self).into_response_parts(res)
    }
}

impl IntoResponseParts for RemoveCookieSession {
    type Error = <Extension<RemoveCookieSession> as IntoResponseParts>::Error;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        Extension(self).into_response_parts(res)
    }
}

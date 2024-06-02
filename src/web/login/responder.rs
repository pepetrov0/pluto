use axum::response::{IntoResponse, Redirect, Response};

use crate::{
    domain::{authentication::AuthenticationError, Session},
    web::{
        _components::{organisms, pages},
        _core::{CreateAuth, Hx, Locale},
    },
};

pub async fn invoke(
    locale: Locale,
    hx: Hx,
    result: Result<Session, AuthenticationError>,
) -> Response {
    match result {
        Ok(session) => (CreateAuth(session), Redirect::to("/")).into_response(),
        Err(error) if hx.request => {
            organisms::login_form(locale.as_str(), Some(error)).into_response()
        }
        Err(error) => pages::login(locale.as_str(), Some(error)).into_response(),
    }
}

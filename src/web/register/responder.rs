use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie;

use crate::{
    domain::{registration::RegistrationError, sessions::Session},
    web::{
        _components::pages,
        _core::{CreateAuth, Hx, Locale, Redirect},
    },
};

pub async fn invoke(
    locale: Locale,
    hx: Hx,
    key: cookie::Key,
    _args: super::Arguments,
    result: Result<Session, RegistrationError>,
) -> Response {
    match result {
        Ok(s) => (CreateAuth(key, s.id), Redirect::see_other(hx, "/")).into_response(),
        Err(_) => pages::register(locale.as_str()).into_response(),
    }
}

use crate::web::{_components::pages, _core::Locale};
use axum::response::{IntoResponse, Redirect, Response};

pub async fn invoke(locale: Locale, is_authorized: bool) -> Response {
    if is_authorized {
        return Redirect::to("/").into_response();
    }

    pages::login(locale.as_str(), None).into_response()
}

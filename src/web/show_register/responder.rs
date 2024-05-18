use crate::web::_components::pages;
use axum::response::{IntoResponse, Redirect, Response};

pub async fn invoke(locale: &str, is_authorized: bool) -> Response {
    if is_authorized {
        return Redirect::to("/").into_response();
    }

    pages::register(locale).into_response()
}

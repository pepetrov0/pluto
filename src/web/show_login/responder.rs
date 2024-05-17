use crate::web::_components::{organisms, templates};
use axum::response::{IntoResponse, Redirect, Response};

pub async fn invoke(locale: &str, is_authorized: bool) -> Response {
    if is_authorized {
        return Redirect::to("/").into_response();
    }

    templates::page(locale, "login.title", false, organisms::login_form(locale)).into_response()
}

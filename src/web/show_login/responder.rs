use axum::response::{IntoResponse, Redirect, Response};
use crate::web::_components::{self, login};

pub async fn invoke(locale: &str, is_authorized: bool) -> Response {
    if is_authorized {
        return Redirect::to("/").into_response();
    }

    _components::page(locale, "login.title", false, login(locale)).into_response()
}

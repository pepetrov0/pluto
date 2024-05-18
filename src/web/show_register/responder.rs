use crate::web::{
    _components::pages,
    _core::{Hx, Redirect},
};
use axum::response::{IntoResponse, Response};

pub async fn invoke(locale: &str, hx: Hx, is_authorized: bool) -> Response {
    if is_authorized {
        return Redirect::see_other(hx, "/").into_response();
    }

    pages::register(locale).into_response()
}

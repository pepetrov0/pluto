use crate::web::{
    _components::pages,
    _core::{Hx, Locale, Redirect},
};
use axum::response::{IntoResponse, Response};

pub async fn invoke(locale: Locale, hx: Hx, is_authorized: bool) -> Response {
    if is_authorized {
        return Redirect::see_other(hx, "/").into_response();
    }

    pages::login(locale.as_str()).into_response()
}

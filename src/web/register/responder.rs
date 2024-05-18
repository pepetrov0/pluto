use axum::response::{IntoResponse, Response};

use crate::web::{
    _components::pages,
    _core::{Hx, Locale, Redirect},
};

pub async fn invoke(locale: Locale, hx: Hx) -> Response {
    if true {
        return Redirect::see_other(hx, "/login").into_response();
    }

    pages::register(locale.as_str()).into_response()
}

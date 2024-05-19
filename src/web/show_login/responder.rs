use crate::web::{
    _components::{organisms, pages},
    _core::{Hx, Locale, Redirect},
};
use axum::response::{IntoResponse, Response};

pub async fn invoke(locale: Locale, hx: Hx, is_authorized: bool) -> Response {
    if is_authorized {
        return Redirect::see_other(hx, "/").into_response();
    }

    match hx.request && !hx.boosted {
        true => organisms::login_form(locale.as_str()).into_response(),
        false => pages::login(locale.as_str()).into_response(),
    }
}

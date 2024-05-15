use axum::response::{IntoResponse, Response};
use maud::html;

use crate::web::{_components::page, _core::Locale};

pub fn invoke(locale: Locale) -> Response {
    let content = html! { "Hello World!" };
    page(locale.as_str(), "dashboard.title", true, content).into_response()
}

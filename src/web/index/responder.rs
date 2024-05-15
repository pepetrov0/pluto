use axum::response::{IntoResponse, Response};
use maud::html;

use crate::web::{
    _components::page,
    _core::{Auth, Locale},
};

pub fn invoke(locale: Locale, auth: Auth) -> Response {
    let content = html! { "Hello, " (auth.0.email) " 👋" };
    page(locale.as_str(), "dashboard.title", true, content).into_response()
}

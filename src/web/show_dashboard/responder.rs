use axum::response::{IntoResponse, Response};
use maud::html;

use crate::web::{
    _components,
    _core::{Auth, Locale},
};

pub fn invoke(locale: Locale, auth: Auth) -> Response {
    let content = html! { span .w-full { "Hello, " (auth.user.email) " 👋" } };
    _components::templates::page(locale.as_str(), "dashboard.title", content).into_response()
}

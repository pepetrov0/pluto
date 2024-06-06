use axum::response::{IntoResponse, Response};
use maud::html;

use crate::web::{
    _components::{templates, Icon},
    _core::{Auth, Locale},
};

pub fn invoke(locale: Locale, auth: Auth) -> Response {
    let content = html! { span .w-full { "Hello, " (auth.user.email) " 👋" } };
    templates::page(locale.as_str(), Icon::Newspaper, "dashboard.title", content).into_response()
}

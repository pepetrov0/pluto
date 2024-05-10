use axum::response::{IntoResponse, Response};
use maud::html;

use crate::web::_components::page;

pub fn invoke() -> Response {
    let content = html! { "Hello World!" };
    page("index", content).into_response()
}

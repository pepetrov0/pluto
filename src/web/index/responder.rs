use axum::response::{IntoResponse, Response};

use crate::web::components::page;

pub fn invoke() -> Response {
    page("index").into_response()
}

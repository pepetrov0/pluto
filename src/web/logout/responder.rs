use axum::response::{IntoResponse, Redirect, Response};

use crate::web::_core::DeleteAuth;

pub async fn invoke() -> Response {
    (DeleteAuth, Redirect::to("/")).into_response()
}

use axum::response::{IntoResponse, Response};

use crate::web::_core::{DeleteAuth, Hx, Redirect};

pub async fn invoke(hx: Hx) -> Response {
    (DeleteAuth, Redirect::see_other(hx, "/")).into_response()
}

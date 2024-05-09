use axum::response::Response;

pub async fn invoke() -> Response {
    super::responder::invoke()
}

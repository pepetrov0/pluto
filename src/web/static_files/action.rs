use axum::{extract::Path, response::Response};

pub async fn invoke(Path(path): Path<String>) -> Response {
    super::responder::invoke(path.as_str()).await
}

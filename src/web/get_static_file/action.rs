use axum::{extract::Path, response::Response};

#[tracing::instrument]
pub async fn invoke(Path(path): Path<String>) -> Response {
    super::responder::invoke(path.as_str()).await
}

use axum::{body::Body, extract::Request, http::StatusCode};
use tower::ServiceExt;

use crate::domain::database::AnyDatabase;

#[tokio::test]
async fn health() {
    let database = AnyDatabase::in_memory().await.unwrap();
    let router = crate::web::router(database);

    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

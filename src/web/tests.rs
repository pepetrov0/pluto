use axum::{body::Body, extract::Request, http::StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn health() {
    let router = crate::web::router();

    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}


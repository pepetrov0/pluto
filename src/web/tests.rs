use axum::{body::Body, extract::Request, http::StatusCode};
use axum_extra::extract::cookie;
use tower::ServiceExt;

use crate::domain::{database::AnyDatabase, Configuration};

#[tokio::test]
async fn health() {
    let database = AnyDatabase::in_memory().await.unwrap();
    let router = crate::web::router(Configuration::default(), database, cookie::Key::generate());

    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

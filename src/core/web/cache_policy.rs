use axum::{
    body::Body,
    http::{header, Request},
    middleware::Next,
    response::Response,
};

pub async fn middleware(req: Request<Body>, next: Next) -> Response {
    let mut response = next.run(req).await;

    if !response.headers().contains_key(header::CACHE_CONTROL) {
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            "no-store".try_into().expect("should not happen"),
        );
    }

    response
}

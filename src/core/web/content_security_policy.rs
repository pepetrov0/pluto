//! Implements content security policies

use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

/// A function middleware to add Content-Security-Policy to every response
pub async fn middleware(req: Request<Body>, next: Next) -> Response {
    let resp = next.run(req).await;

    let header = (
        "Content-Security-Policy",
        "default-src 'self' https://fonts.googleapis.com https://fonts.gstatic.com; img-src 'self' data:",
    );
    ([header], resp).into_response()
}

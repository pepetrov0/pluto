use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn middleware(req: Request<Body>, next: Next) -> Response {
    let resp = next.run(req).await;

    let header = (
        "Content-Security-Policy",
        "default-src 'self' https://fonts.googleapis.com https://fonts.gstatic.com; img-src 'self' data:",
    );
    ([header], resp).into_response()
}

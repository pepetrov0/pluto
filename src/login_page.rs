use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "login-page.html")]
struct LoginPage;

pub async fn handler() -> impl IntoResponse {
    LoginPage
}

use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "register-page.html")]
struct RegisterPage;

pub async fn handler() -> impl IntoResponse {
    RegisterPage
}

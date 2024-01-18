use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "register-email-taken-page.html")]
struct RegisterEmailTakenPage;

pub async fn handler() -> impl IntoResponse {
    RegisterEmailTakenPage
}

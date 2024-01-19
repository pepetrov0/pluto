//! Enables the users to register locally

use askama::Template;
use axum::{routing, Router};

use crate::{templates::HtmlTemplate, AppState, compression};

#[derive(Template)]
#[template(path = "auth/local/register.html")]
struct RegisterPage;

async fn page() -> HtmlTemplate<RegisterPage> {
    HtmlTemplate(RegisterPage)
}

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/register",
        routing::get(page).layer(compression::default()),
    )
}

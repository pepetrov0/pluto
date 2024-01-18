//! Enables the users to register locally

use askama::Template;
use axum::{routing, Router};

use crate::AppState;

#[derive(Template)]
#[template(path = "auth/local/register.html")]
struct RegisterPage;

async fn page() -> RegisterPage {
    RegisterPage
}

pub fn router() -> Router<AppState> {
    Router::new().route("/register", routing::get(page))
}

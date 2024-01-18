use axum::{routing, Router};

use crate::AppState;

mod login_page;
mod register_api;
mod register_email_taken_page;
mod register_page;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", routing::get(login_page::handler))
        .route("/register", routing::get(register_page::handler))
        .route("/register", routing::post(register_api::handler))
        .route(
            "/register-email-taken",
            routing::get(register_email_taken_page::handler),
        )
}

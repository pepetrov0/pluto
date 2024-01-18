use axum::{extract::State, response::Redirect, Form};

use crate::AppState;

#[derive(serde::Deserialize)]
pub struct CreateUserForm {
    pub email: String,
    pub password: String,
}

pub async fn handler(State(state): State<AppState>, Form(input): Form<CreateUserForm>) -> Redirect {
    state
        .user_repository
        .create_user(input.email, input.password)
        .await
        .ok()
        .map(|_| Redirect::to("/login"))
        .unwrap_or_else(|| Redirect::to("/register-email-taken"))
}

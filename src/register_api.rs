use axum::{response::Redirect, Form};

use crate::{database::Database, user::UserRepository};

#[derive(serde::Deserialize)]
pub struct CreateUserForm {
    pub email: String,
    pub password: String,
}

pub async fn handler(database: Database, Form(input): Form<CreateUserForm>) -> Redirect {
    database
        .create_user(input.email, input.password)
        .await
        .ok()
        .map(|_| Redirect::to("/login"))
        .unwrap_or_else(|| Redirect::to("/register-email-taken"))
}

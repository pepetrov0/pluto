use axum::{response::Response, Form};
use secrecy::Secret;

use crate::{
    domain::change_password::validate_change_password,
    web::{
        _components::organisms::ChangePasswordFormData,
        _core::{Auth, Locale},
    },
};

#[tracing::instrument]
pub async fn invoke(
    locale: Locale,
    auth: Auth,
    Form(data): Form<ChangePasswordFormData>,
) -> Response {
    let error = validate_change_password(
        &auth.user,
        &Secret::from(data.new_password.clone()),
        &Secret::from(data.confirm_new_password.clone()),
    )
    .err();
    super::responder::invoke(locale, data, error).await
}

use axum::{response::Response, Form};

use crate::{
    domain::change_password::validate_change_password,
    web::{
        _components::organisms::ChangePasswordFormData,
        _core::{Auth, Locale},
    },
};

pub async fn invoke(
    locale: Locale,
    auth: Auth,
    Form(data): Form<ChangePasswordFormData>,
) -> Response {
    let error =
        validate_change_password(&auth.user, &data.new_password, &data.confirm_new_password).err();
    super::responder::invoke(locale, data, error).await
}

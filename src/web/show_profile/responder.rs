use axum::response::{IntoResponse, Response};

use crate::{
    domain::users::User,
    web::{
        _components::{organisms::ChangeEmailFormData, pages},
        _core::Locale,
    },
};

pub fn invoke(locale: Locale, user: User) -> Response {
    pages::profile(
        locale.as_str(),
        user.clone(),
        Some(ChangeEmailFormData {
            new_email: user.email.clone(),
            ..Default::default()
        }),
        None,
    )
    .into_response()
}

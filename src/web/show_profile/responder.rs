use axum::response::{IntoResponse, Response};

use crate::{
    domain::User,
    web::{
        _components::{organisms::ChangeEmailFormData, pages},
        _core::Locale,
    },
};

pub fn invoke(locale: Locale, user: User) -> Response {
    pages::profile(
        locale.as_str(),
        &user,
        Some(ChangeEmailFormData {
            new_email: user.email.clone(),
            ..Default::default()
        }),
        None,
        None,
        None,
    )
    .into_response()
}

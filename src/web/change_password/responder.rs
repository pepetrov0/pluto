use axum::response::{IntoResponse, Redirect, Response};

use crate::{
    domain::{change_password::ChangePasswordError, users::User},
    web::{
        _components::{
            organisms::{change_password_form, ChangePasswordFormData},
            pages,
        },
        _core::{Hx, Locale},
    },
};

pub async fn invoke(
    locale: Locale,
    hx: Hx,
    user: &User,
    data: ChangePasswordFormData,
    error: Option<ChangePasswordError>,
) -> Response {
    match error {
        Some(error) if hx.request => {
            change_password_form(locale.as_str(), Some(data), Some(error)).into_response()
        }
        Some(error) => pages::profile(locale.as_str(), user, None, None, Some(data), Some(error))
            .into_response(),
        None => Redirect::to("/profile").into_response(),
    }
}

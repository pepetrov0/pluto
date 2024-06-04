use axum::response::{IntoResponse, Redirect, Response};

use crate::{
    domain::{change_password::ChangePasswordError, User},
    web::{
        _components::{
            organisms::{change_password, ChangePasswordData},
            pages,
        },
        _core::{Hx, Locale},
    },
};

pub async fn invoke(
    locale: Locale,
    hx: Hx,
    user: &User,
    data: ChangePasswordData,
    error: Option<ChangePasswordError>,
) -> Response {
    match error {
        Some(error) if hx.request => {
            change_password(locale.as_str(), Some(data), Some(error)).into_response()
        }
        Some(error) => pages::profile(
            locale.as_str(),
            user,
            None,
            None,
            Some(data),
            Some(error),
            None,
        )
        .into_response(),
        None => Redirect::to("/profile").into_response(),
    }
}

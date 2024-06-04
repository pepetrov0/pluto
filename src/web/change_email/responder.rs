use axum::response::{IntoResponse, Redirect, Response};

use crate::{
    domain::{change_email::ChangeEmailError, User},
    web::{
        _components::{
            organisms::{change_email, ChangeEmailData},
            pages,
        },
        _core::{Hx, Locale},
    },
};

pub async fn invoke(
    locale: Locale,
    hx: Hx,
    user: &User,
    data: ChangeEmailData,
    error: Option<ChangeEmailError>,
) -> Response {
    match error {
        Some(error) if hx.request => {
            change_email(locale.as_str(), Some(data), Some(error)).into_response()
        }
        Some(error) => pages::profile(
            locale.as_str(),
            user,
            Some(data),
            Some(error),
            None,
            None,
            None,
        )
        .into_response(),
        None => Redirect::to("/profile").into_response(),
    }
}

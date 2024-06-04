use axum::response::{IntoResponse, Redirect, Response};

use crate::{
    domain::{delete_user::DeleteUserError, User},
    web::{
        _components::{organisms, pages},
        _core::{DeleteAuth, Hx, Locale},
    },
};

pub async fn invoke(
    locale: Locale,
    hx: Hx,
    user: &User,
    error: Option<DeleteUserError>,
) -> Response {
    match error {
        Some(err) if hx.request => {
            organisms::danger_zone(locale.as_str(), Some(err)).into_response()
        }
        Some(err) => {
            pages::profile(locale.as_str(), user, None, None, None, None, Some(err)).into_response()
        }
        None => (DeleteAuth, Redirect::to("/")).into_response(),
    }
}

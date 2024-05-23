use axum::response::{IntoResponse, Response};

use crate::{
    domain::users::User,
    web::{_components::pages, _core::Locale},
};

pub fn invoke(locale: Locale, user: User) -> Response {
    pages::profile(locale.as_str(), user).into_response()
}

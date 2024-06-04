//! The profile pages.

use maud::{html, Markup};

use crate::{
    domain::{change_email::ChangeEmailError, change_password::ChangePasswordError, delete_user::DeleteUserError, User},
    web::_components::{
        organisms::{
            change_email, change_password, danger_zone, profile_details, ChangeEmailData,
            ChangePasswordData,
        },
        templates,
    },
};

// A profile page
pub fn profile(
    locale: &str,
    user: &User,
    change_email_data: Option<ChangeEmailData>,
    change_email_error: Option<ChangeEmailError>,
    change_password_data: Option<ChangePasswordData>,
    change_password_error: Option<ChangePasswordError>,
    delete_user_error: Option<DeleteUserError>,
) -> Markup {
    let content = html! {
        (profile_details(locale, &user))

        @if user.password.is_some() {
            (change_email(locale, change_email_data, change_email_error))
            (change_password(locale, change_password_data, change_password_error))
            (danger_zone(locale, delete_user_error))
        }
    };
    templates::page(locale, "profile.title", content)
}

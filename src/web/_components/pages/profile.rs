//! The profile pages.

use maud::{html, Markup};

use crate::{
    domain::{change_email::ChangeEmailError, change_password::ChangePasswordError, users::User},
    web::_components::{
        organisms::{
            change_email_form, change_password_form, profile_details_card, ChangeEmailFormData,
            ChangePasswordFormData,
        },
        templates,
    },
};

// A profile page
pub fn profile(
    locale: &str,
    user: &User,
    change_email_data: Option<ChangeEmailFormData>,
    change_email_error: Option<ChangeEmailError>,
    change_password_data: Option<ChangePasswordFormData>,
    change_password_error: Option<ChangePasswordError>,
) -> Markup {
    let content = html! {
        (profile_details_card(locale, &user))

        @if user.has_password {
            (change_email_form(locale, change_email_data, change_email_error))
            (change_password_form(locale, change_password_data, change_password_error))
        }
    };
    templates::page(locale, "profile.title", content)
}

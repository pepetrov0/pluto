//! The profile pages.

use maud::{html, Markup};

use crate::{
    domain::{change_email::ChangeEmailError, users::User},
    web::_components::{
        organisms::{change_email_form, profile_details_card, ChangeEmailFormData},
        templates,
    },
};

// A profile page
pub fn profile(
    locale: &str,
    user: &User,
    change_email_data: Option<ChangeEmailFormData>,
    change_email_error: Option<ChangeEmailError>,
) -> Markup {
    let content = html! {
        (profile_details_card(locale, &user))

        @if user.has_password {
            (change_email_form(locale, change_email_data, change_email_error))
        }
    };
    templates::page(locale, "profile.title", content)
}

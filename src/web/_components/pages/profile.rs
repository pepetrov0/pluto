//! The profile pages.

use maud::{html, Markup};

use crate::{
    domain::users::User,
    web::_components::{
        organisms::{change_email_form, profile_details_card, ChangeEmailFormData},
        templates,
    },
};

// A profile page
pub fn profile(
    locale: &str,
    user: User,
    change_email_form_data: Option<ChangeEmailFormData>,
) -> Markup {
    let content = html! {
        (profile_details_card(locale, user))
        (change_email_form(locale, change_email_form_data))
    };
    templates::page(locale, "profile.title", content)
}

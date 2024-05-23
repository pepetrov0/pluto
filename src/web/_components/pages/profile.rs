//! The profile pages.

use maud::{html, Markup};

use crate::{
    domain::users::User,
    web::_components::{organisms::profile_details_card, templates},
};

// A profile page
pub fn profile(locale: &str, user: User) -> Markup {
    let content = html! { (profile_details_card(locale, user)) };
    templates::page(locale, "profile.title", content)
}

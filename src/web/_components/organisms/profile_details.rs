//! Implements a profile card component.

use maud::{html, Markup};
use rust_i18n::t;

use crate::{domain::User, web::_components::atoms::Icon};

pub fn profile_details(locale: &str, user: &User) -> Markup {
    const STYLES: &str = "card grid gap-2 grid-cols-1 sm:grid-cols-2";
    const TITLE_STYLES: &str = "sm:col-span-2 flex flex-row gap-2 items-center mb-4";
    const FIELDS_STYLES: &str = "flex flex-col";
    const FIELD_LABEL_STYLES: &str = "flex flex-row gap-1 items-center";

    html! {
        section .(STYLES) {
            h2 .(TITLE_STYLES) {
                span ."icon-sm" { (Icon::Identification) }
                span { (t!("profile-details.title", locale = locale)) }
            }

            div .(FIELDS_STYLES) {
                strong .(FIELD_LABEL_STYLES) {
                    span ."icon-xs" { (Icon::AtSymbol) }
                    span { (t!("profile-details.labels.email", locale = locale)) }
                }
                span { (user.email) }
            }
        }
    }
}

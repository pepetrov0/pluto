//! A danger zone card.

use maud::{html, Markup};
use rust_i18n::t;

use crate::web::_components::atoms::Icon;

///

/// A danger zone organism that provides a form for the user to delete their account.
pub fn danger_zone(locale: &str) -> Markup {
    const STYLES: &str = "card flex flex-col gap-4 border-2 border-red-500";
    const TITLE_STYLES: &str = "text-red-500 flex flex-row gap-2 items-center mb-2";
    // const ERROR_LABEL_STYLES: &str = "text-sm text-red-500";
    const FIELD_CONTAINER_STYLES: &str = "w-full flex flex-col gap-1";
    const FIELD_LABEL_STYLES: &str = "flex flex-row gap-1 items-center";
    const ACTIONS_STYLES: &str = "flex flex-row items-center justify-end mt-2";
    const ACTION_STYLES: &str = "sm:w-fit danger";

    html! {
        form #delete-account-form .(STYLES)
            action="/profile/delete"
            method="POST"
            hx-boost="true"
            hx-disabled-elt="#delete-account-form input"
            hx-indicator="#delete-account-form input"
        {
            h2 .(TITLE_STYLES) {
                span ."icon-sm" { (Icon::ExclamationTriangle) }
                span { (t!("danger-zone.title", locale = locale)) }
            }

            p {
                (t!("danger-zone.description", locale = locale))
            }

            div .(FIELD_CONTAINER_STYLES) {
                span .(FIELD_LABEL_STYLES) {
                    span ."icon-xs" { (Icon::Key) }
                    label for="danger-zone-password" { (t!("danger-zone.labels.password", locale = locale)) };
                }
                input #danger-zone-password type="password" name="password"
                    placeholder=(t!("danger-zone.placeholders.password", locale = locale));
            }

            // @if let Some(error) = error {
            //     span .(ERROR_LABEL_STYLES) { (t!(error, locale = locale)) };
            // }

            div .(ACTIONS_STYLES) {
                input .(ACTION_STYLES)
                    type="submit" value=(t!("danger-zone.labels.delete", locale = locale))
                    hx-post="/profile/delete"
                    hx-target="#delete-account-form"
                    hx-swap="outerHTML";
            }
        }
    }
}

//! Implementation of a change email form.

use maud::{html, Markup};
use rust_i18n::t;

use crate::web::_components::atoms::Icon;

/// Represents the data in a change email form.
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct ChangeEmailFormData {
    pub new_email: String,
    pub current_password: String,
}

/// A form that allows the user to change their email.
pub fn change_email_form(locale: &str, data: Option<ChangeEmailFormData>) -> Markup {
    const STYLES: &str = "card grid gap-2 grid-cols-1 sm:grid-cols-2";
    const TITLE_STYLES: &str = "sm:col-span-2 flex flex-row gap-2 items-center mb-4";
    // const ERROR_LABEL_STYLES: &str = "text-sm text-red-500";
    const FIELD_CONTAINER_STYLES: &str = "w-full flex flex-col gap-1";
    const FIELD_LABEL_STYLES: &str = "flex flex-row gap-2 items-center";
    const ACTIONS_STYLES: &str = "sm:col-span-2 flex flex-row items-center justify-end mt-4";
    const ACTION_STYLES: &str = "sm:w-fit";

    let data = data.unwrap_or_default();

    html! {
        form .(STYLES) action="/profile/change-email" method="POST"
            hx-boost="true" hx-disabled-elt="[type='submit']" hx-indicator="[type='submit']" {
            h2 .(TITLE_STYLES) {
                span ."icon-sm" { (Icon::AtSymbol) }
                span { (t!("change-email.title", locale = locale)) }
            }

            div .(FIELD_CONTAINER_STYLES) {
                span .(FIELD_LABEL_STYLES) {
                    span ."icon-xs" { (Icon::AtSymbol) }
                    label for="new-email" { (t!("change-email.labels.new-email", locale = locale)) };
                }
                input #new-email type="email" name="new_email" value=(data.new_email) minlength="5"
                    placeholder=(t!("change-email.placeholders.new-email", locale = locale));
            }

            div .(FIELD_CONTAINER_STYLES) {
                span .(FIELD_LABEL_STYLES) {
                    span ."icon-xs" { (Icon::Key) }
                    label for="current-password" { (t!("change-email.labels.current-password", locale = locale)) };
                }
                input #current-password type="password" name="current_password"
                    placeholder=(t!("change-email.placeholders.current-password", locale = locale));
            }

            div .(ACTIONS_STYLES) {
                input .(ACTION_STYLES) type="submit" value=(t!("change-email.labels.save", locale = locale));
            }
        }
    }
}

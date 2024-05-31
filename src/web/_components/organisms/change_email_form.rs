//! Implementation of a change email form.

use maud::{html, Markup};
use rust_i18n::t;

use crate::{domain::change_email::ChangeEmailError, web::_components::atoms::Icon};

/// Represents the data in a change email form.
#[derive(Default, Clone, serde::Deserialize)]
pub struct ChangeEmailFormData {
    pub new_email: String,
    pub current_password: String,
}

/// A form that allows the user to change their email.
pub fn change_email_form(
    locale: &str,
    data: Option<ChangeEmailFormData>,
    error: Option<ChangeEmailError>,
) -> Markup {
    const STYLES: &str = "card grid gap-4 grid-cols-1 sm:grid-cols-2";
    const TITLE_STYLES: &str = "sm:col-span-2 flex flex-row gap-2 items-center mb-2";
    const ERROR_LABEL_STYLES: &str = "text-sm text-red-500";
    const FIELD_CONTAINER_STYLES: &str = "w-full flex flex-col gap-1";
    const FIELD_LABEL_STYLES: &str = "flex flex-row gap-1 items-center";
    const ACTIONS_STYLES: &str = "sm:col-span-2 flex flex-row items-center justify-end mt-2";
    const ACTION_STYLES: &str = "sm:w-fit";

    let data = data.unwrap_or_default();

    // extract email error
    let new_email_error = match error {
        Some(ChangeEmailError::EmailTaken) => Some("change-email.errors.email-taken"),
        Some(ChangeEmailError::EmailInvalid) => Some("change-email.errors.email-invalid"),
        _ => None,
    };

    // extract current password error
    let current_password_error = match error {
        Some(ChangeEmailError::InvalidCredentials) => {
            Some("change-email.errors.invalid-credentials")
        }
        _ => None,
    };

    html! {
        form #change-email-form .(STYLES) action="/profile/change-email" method="POST"
            hx-boost="true" hx-disabled-elt="#change-email-form input" hx-indicator="#change-email-form input" {
            h2 .(TITLE_STYLES) {
                span ."icon-sm" { (Icon::AtSymbol) }
                span { (t!("change-email.title", locale = locale)) }
            }

            div .(FIELD_CONTAINER_STYLES) {
                span .(FIELD_LABEL_STYLES) {
                    span ."icon-xs" { (Icon::AtSymbol) }
                    label for="new-email" { (t!("change-email.labels.new-email", locale = locale)) };
                }
                input #new-email
                    .error[new_email_error.is_some()]
                    type="email" name="new_email" minlength="5" value=(data.new_email)
                    placeholder=(t!("change-email.placeholders.new-email", locale = locale))
                    hx-post="/profile/change-email/validate"
                    hx-target="#change-email-form"
                    hx-swap="outerHTML"
                    hx-trigger="change";
                @if let Some(copy) = new_email_error {
                    span .(ERROR_LABEL_STYLES) { (t!(copy, locale = locale)) };
                }
            }

            div .(FIELD_CONTAINER_STYLES) {
                span .(FIELD_LABEL_STYLES) {
                    span ."icon-xs" { (Icon::Key) }
                    label for="current-password" { (t!("change-email.labels.current-password", locale = locale)) };
                }
                input #current-password
                    .error[current_password_error.is_some()]
                    type="password"
                    name="current_password"
                    placeholder=(t!("change-email.placeholders.current-password", locale = locale));
                @if let Some(copy) = current_password_error {
                    span .(ERROR_LABEL_STYLES) { (t!(copy, locale = locale)) };
                }
            }

            @if let Some(ChangeEmailError::Failure) = error {
                span ."sm:col-span-2 text-center" .(ERROR_LABEL_STYLES) {
                    (t!("change-email.errors.something-went-wrong", locale = locale))
                };
            }

            div .(ACTIONS_STYLES) {
                input .(ACTION_STYLES)
                    type="submit" value=(t!("change-email.labels.save", locale = locale))
                    hx-post="/profile/change-email"
                    hx-target="#change-email-form"
                    hx-swap="outerHTML";
            }
        }
    }
}

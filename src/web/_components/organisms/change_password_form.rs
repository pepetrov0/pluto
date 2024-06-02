//! Implementation of a change password form.

use maud::{html, Markup};
use rust_i18n::t;
use secrecy::{ExposeSecret, Secret};

use crate::{domain::change_password::ChangePasswordError, web::_components::atoms::Icon};

/// Represents the data in a change password form.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ChangePasswordFormData {
    pub new_password: Secret<String>,
    pub confirm_new_password: Secret<String>,
    pub current_password: Secret<String>,
}

/// A form that allows the user to change their password.
pub fn change_password_form(
    locale: &str,
    data: Option<ChangePasswordFormData>,
    error: Option<ChangePasswordError>,
) -> Markup {
    const STYLES: &str = "card grid gap-4 grid-cols-1 sm:grid-cols-2";
    const TITLE_STYLES: &str = "sm:col-span-2 flex flex-row gap-2 items-center mb-2";
    const ERROR_LABEL_STYLES: &str = "text-sm text-red-500";
    const FIELD_CONTAINER_STYLES: &str = "w-full flex flex-col gap-1";
    const FIELD_LABEL_STYLES: &str = "flex flex-row gap-1 items-center";
    const ACTIONS_STYLES: &str = "sm:col-span-2 flex flex-row items-center justify-end mt-2";
    const ACTION_STYLES: &str = "sm:w-fit";

    let data = data.unwrap_or_default();

    // extract new password error
    let new_password_error = match error {
        Some(ChangePasswordError::WeakPassword) => Some("change-password.errors.weak-password"),
        _ => None,
    };

    // extract confirm new password error
    let confirm_new_password_error = match error {
        Some(ChangePasswordError::PasswordsMismatch) => {
            Some("change-password.errors.passwords-mismatch")
        }
        _ => None,
    };

    // extract global error
    let global_error = match error {
        Some(ChangePasswordError::InvalidCredentials) => {
            Some("change-password.errors.invalid-credentials")
        }
        Some(ChangePasswordError::Failure) => Some("change-password.errors.something-went-wrong"),
        _ => None,
    };

    html! {
        form #change-password-form .(STYLES) action="/profile/change-password" method="POST"
            hx-boost="true" hx-disabled-elt="#change-password-form input" hx-indicator="#change-password-form input" {
            h2 .(TITLE_STYLES) {
                span ."icon-sm" { (Icon::Key) }
                span { (t!("change-password.title", locale = locale)) }
            }

            div .(FIELD_CONTAINER_STYLES) {
                span .(FIELD_LABEL_STYLES) {
                    span ."icon-xs" { (Icon::Key) }
                    label for="new-password" { (t!("change-password.labels.new-password", locale = locale)) };
                }
                input #new-password
                    type="password" name="new_password" minlength="5"
                    value=(data.new_password.expose_secret().as_str())
                    placeholder=(t!("change-password.placeholders.new-password", locale = locale))
                    hx-post="/profile/change-password/validate"
                    hx-target="#change-password-form"
                    hx-swap="outerHTML"
                    hx-trigger="change";
                @if let Some(copy) = new_password_error {
                    span .(ERROR_LABEL_STYLES) { (t!(copy, locale = locale)) };
                }
            }

            div .(FIELD_CONTAINER_STYLES) {
                span .(FIELD_LABEL_STYLES) {
                    span ."icon-xs" { (Icon::Key) }
                    label for="confirm-new-password" { (t!("change-password.labels.confirm-new-password", locale = locale)) };
                }
                input #confirm-new-password
                    type="password" name="confirm_new_password" minlength="5"
                    value=(data.confirm_new_password.expose_secret().as_str())
                    placeholder=(t!("change-password.placeholders.confirm-new-password", locale = locale))
                    hx-post="/profile/change-password/validate"
                    hx-target="#change-password-form"
                    hx-swap="outerHTML"
                    hx-trigger="change";
                @if let Some(copy) = confirm_new_password_error {
                    span .(ERROR_LABEL_STYLES) { (t!(copy, locale = locale)) };
                }
            }

            div .(FIELD_CONTAINER_STYLES) ."sm:col-span-2" {
                span .(FIELD_LABEL_STYLES) {
                    span ."icon-xs" { (Icon::Key) }
                    label for="current-password" { (t!("change-password.labels.current-password", locale = locale)) };
                }
                input #current-password
                    type="password"
                    name="current_password"
                    placeholder=(t!("change-password.placeholders.current-password", locale = locale));
            }

            @if let Some(copy) = global_error {
                span ."sm:col-span-2 text-center" .(ERROR_LABEL_STYLES) {
                    (t!(copy, locale = locale))
                };
            }

            div .(ACTIONS_STYLES) {
                input .(ACTION_STYLES)
                    type="submit" value=(t!("change-password.labels.save", locale = locale))
                    hx-post="/profile/change-password"
                    hx-target="#change-password-form"
                    hx-swap="outerHTML";
            }
        }
    }
}

impl Default for ChangePasswordFormData {
    fn default() -> Self {
        Self {
            new_password: Secret::from(String::default()),
            confirm_new_password: Secret::from(String::default()),
            current_password: Secret::from(String::default()),
        }
    }
}

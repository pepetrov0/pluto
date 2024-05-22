//! Implementation of a register form.

use maud::{html, Markup};
use rust_i18n::t;

use crate::domain::registration::RegistrationError;

/// Represents the data in a register form.
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct RegisterFormData {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

/// Returns a register form.
pub fn register_form(
    locale: &str,
    data: Option<RegisterFormData>,
    error: Option<RegistrationError>,
) -> Markup {
    const STYLES: &str = "card mt-4 w-full max-w-md flex flex-col items-center gap-4";
    const ERROR_LABEL_STYLES: &str = "text-sm text-red-500";
    const FIELD_CONTAINER_STYLES: &str = "w-full flex flex-col gap-1";

    let data = data.unwrap_or_default();

    // extract email error
    let email_error = match error {
        Some(RegistrationError::EmailTaken) => Some("register.errors.email-taken"),
        Some(RegistrationError::EmailInvalid) => Some("register.errors.email-invalid"),
        _ => None,
    };

    // extract password error
    let password_error = match error {
        Some(RegistrationError::WeakPassword) => Some("register.errors.weak-password"),
        _ => None,
    };

    // extract confirm password error
    let confirm_password_error = match error {
        Some(RegistrationError::PasswordsMismatch) => Some("register.errors.passwords-mismatch"),
        _ => None,
    };

    html! {
        form #register-form .(STYLES) method="POST" hx-boost="true" hx-disabled-elt="[type='submit']" hx-indicator="[type='submit']" {
            h1 .mb-4 { (t!("register.title", locale = locale)) }

            div .(FIELD_CONTAINER_STYLES) {
                label for="email" { (t!("register.email.label", locale = locale)) };
                input #email type="email" name="email" minlength="3" value=(data.email)
                    placeholder=(t!("register.email.placeholder", locale = locale));
                @if let Some(copy) = email_error {
                    span .(ERROR_LABEL_STYLES) { (t!(copy, locale = locale)) };
                }
            }

            div .(FIELD_CONTAINER_STYLES) {
                label for="password" { (t!("register.password.label", locale = locale)) };
                input #password type="password" name="password" minlength="8" value=(data.password)
                    placeholder=(t!("register.password.placeholder", locale = locale));
                @if let Some(copy) = password_error {
                    span .(ERROR_LABEL_STYLES) { (t!(copy, locale = locale)) };
                }
            }

            div .(FIELD_CONTAINER_STYLES) {
                label for="confirm-password" { (t!("register.confirm-password.label", locale = locale)) };
                input #confirm-password type="password" name="confirm_password" minlength="8" value=(data.confirm_password)
                    placeholder=(t!("register.confirm-password.placeholder", locale = locale));
                @if let Some(copy) = confirm_password_error {
                    span .(ERROR_LABEL_STYLES) { (t!(copy, locale = locale)) };
                }
            }

            @if let Some(RegistrationError::Failure) = error {
                span .(ERROR_LABEL_STYLES) { (t!("register.errors.something-went-wrong", locale = locale)) };
            }

            input .mt-4 type="submit" value=(t!("register.title", locale = locale));

            a href="/login" {
                (t!("register.already-have-an-account", locale = locale))
            }
        }
    }
}

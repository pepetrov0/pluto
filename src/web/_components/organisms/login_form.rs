//! Implementation of a login form.

use maud::{html, Markup};
use rust_i18n::t;

use crate::domain::authentication::AuthenticationError;

/// Represents the data in a login form.
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct LoginFormData {
    pub email: String,
    pub password: String,
}

/// Returns a login form.
pub fn login_form(locale: &str, error: Option<AuthenticationError>) -> Markup {
    const STYLES: &str = "card mt-4 w-full max-w-md flex flex-col items-center gap-4";
    const ERROR_LABEL_STYLES: &str = "text-sm text-red-500";
    const FIELD_CONTAINER_STYLES: &str = "w-full flex flex-col gap-1";

    let error = match error {
        Some(AuthenticationError::Failure) => Some("login.errors.something-went-wrong"),
        Some(AuthenticationError::InvalidCredentials) => Some("login.errors.invalid-credentials"),
        None => None,
    };

    html! {
        form .(STYLES) method="POST" hx-boost="true" hx-disabled-elt="[type='submit']" hx-indicator="[type='submit']" {
            h1 .mb-4 { (t!("login.title", locale = locale)) }

            div .(FIELD_CONTAINER_STYLES) {
                label for="email" { (t!("login.labels.email", locale = locale)) };
                input #email type="text" name="email" placeholder=(t!("login.placeholders.email", locale = locale));
            }

            div .(FIELD_CONTAINER_STYLES) {
                label for="password" { (t!("login.labels.password", locale = locale)) };
                input #password type="password" name="password" placeholder=(t!("login.placeholders.password", locale = locale));
            }

            @if let Some(error) = error {
                span .(ERROR_LABEL_STYLES) { (t!(error, locale = locale)) };
            }

            input .mt-4 type="submit" value=(t!("login.title", locale = locale));

            a href="/register" hx-disabled-elt="this" hx-indicator="this" {
                (t!("login.labels.register", locale = locale))
            }
        }
    }
}

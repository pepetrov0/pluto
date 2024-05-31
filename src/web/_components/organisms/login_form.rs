//! Implementation of a login form.

use maud::{html, Markup};
use rust_i18n::t;

use crate::{domain::authentication::AuthenticationError, web::_components::atoms::Icon};

/// Represents the data in a login form.
#[derive(Default, Clone, serde::Deserialize)]
pub struct LoginFormData {
    pub email: String,
    pub password: String,
}

/// Returns a login form.
pub fn login_form(locale: &str, error: Option<AuthenticationError>) -> Markup {
    const STYLES: &str = "card mt-4 w-full max-w-md flex flex-col items-center gap-4";
    const ERROR_LABEL_STYLES: &str = "text-sm text-red-500";
    const FIELD_CONTAINER_STYLES: &str = "w-full flex flex-col gap-1";
    const FIELD_LABEL_STYLES: &str = "flex flex-row gap-1 items-center";

    let error = match error {
        Some(AuthenticationError::Failure) => Some("login.errors.something-went-wrong"),
        Some(AuthenticationError::InvalidCredentials) => Some("login.errors.invalid-credentials"),
        None => None,
    };

    html! {
        form #login-form .(STYLES) method="POST"
            hx-boost="true" hx-disabled-elt="#login-form input" hx-indicator="#login-form input" {
            h1 .mb-4 { (t!("login.title", locale = locale)) }

            div .(FIELD_CONTAINER_STYLES) {
                span .(FIELD_LABEL_STYLES) {
                    span ."icon-xs" { (Icon::AtSymbol) }
                    label for="email" { (t!("login.labels.email", locale = locale)) };
                }
                input #email type="text" name="email"
                    placeholder=(t!("login.placeholders.email", locale = locale));
            }

            div .(FIELD_CONTAINER_STYLES) {
                span .(FIELD_LABEL_STYLES) {
                    span ."icon-xs" { (Icon::Key) }
                    label for="password" { (t!("login.labels.password", locale = locale)) };
                }
                input #password type="password" name="password"
                    placeholder=(t!("login.placeholders.password", locale = locale));
            }

            @if let Some(error) = error {
                span .(ERROR_LABEL_STYLES) { (t!(error, locale = locale)) };
            }

            input .mt-4 type="submit" value=(t!("login.title", locale = locale))
                hx-post="/login" hx-target="#login-form" hx-swap="outerHTML";

            a href="/register" hx-disabled-elt="this" hx-indicator="this" {
                (t!("login.labels.register", locale = locale))
            }
        }
    }
}

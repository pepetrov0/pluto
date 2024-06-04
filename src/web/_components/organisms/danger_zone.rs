//! A danger zone card.

use maud::{html, Markup};
use rust_i18n::t;
use secrecy::Secret;

use crate::{domain::delete_user::DeleteUserError, web::_components::atoms::Icon};

/// Data received from a danger zone form.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DangerZoneData {
    pub password: Secret<String>,
}

/// A danger zone organism that provides a form for the user to delete their account.
pub fn danger_zone(locale: &str, error: Option<DeleteUserError>) -> Markup {
    const STYLES: &str = "card flex flex-col gap-4 border-2 border-red-500";
    const TITLE_STYLES: &str = "text-red-500 flex flex-row gap-2 items-center mb-2";
    const ERROR_LABEL_STYLES: &str = "text-sm text-red-500";
    const FIELD_CONTAINER_STYLES: &str = "w-full flex flex-col gap-1";
    const FIELD_LABEL_STYLES: &str = "flex flex-row gap-1 items-center";
    const ACTIONS_STYLES: &str = "flex flex-row items-center justify-end mt-2";
    const ACTION_STYLES: &str = "sm:w-fit danger";

    let password_error = match error {
        Some(DeleteUserError::InvalidCredentials) => Some("danger-zone.errors.invalid-credentials"),
        _ => None,
    };

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
                    label for="danger-zone-password" { (t!("danger-zone.labels.password", locale = locale)) }
                }
                input #danger-zone-password type="password" name="password"
                    placeholder=(t!("danger-zone.placeholders.password", locale = locale));
                @if let Some(copy) = password_error {
                    span .(ERROR_LABEL_STYLES) { (t!(copy, locale = locale)) }
                }
            }

            @if let Some(DeleteUserError::Failure) = error {
                span .(ERROR_LABEL_STYLES) { (t!("danger-zone.errors.something-went-wrong", locale = locale)) }
            }

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

//! Implementation of a register form.

use maud::{html, Markup};
use rust_i18n::t;

/// Returns a register form.
pub fn register_form(locale: &str) -> Markup {
    const STYLES: &str = "card mt-4 w-full max-w-md flex flex-col items-center gap-4";
    const FIELD_CONTAINER_STYLES: &str = "w-full flex flex-col gap-1";

    html! {
        form .(STYLES) {
            h1 .mb-4 { (t!("register.title", locale = locale)) }

            div .(FIELD_CONTAINER_STYLES) {
                label for="email" { (t!("register.email.label", locale = locale)) };
                input #email type="email" name="email" minlength="3" placeholder=(t!("register.email.placeholder", locale = locale));
            }

            div .(FIELD_CONTAINER_STYLES) {
                label for="password" { (t!("register.password.label", locale = locale)) };
                input #password type="password" name="password" minlength="8" placeholder=(t!("register.password.placeholder", locale = locale));
            }

            div .(FIELD_CONTAINER_STYLES) {
                label for="confirm-password" { (t!("register.confirm-password.label", locale = locale)) };
                input #confirm-password type="password" name="confirm-password" minlength="8" placeholder=(t!("register.confirm-password.placeholder", locale = locale));
            }

            input .mt-4 type="submit" value=(t!("register.title", locale = locale));

            a href="/login" {
                (t!("register.already-have-an-account", locale = locale))
            }
        }
    }
}

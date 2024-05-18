//! Implementation of a login form.

use maud::{html, Markup};
use rust_i18n::t;

/// Returns a login form.
pub fn login_form(locale: &str) -> Markup {
    const STYLES: &str = "card mt-4 w-full max-w-md flex flex-col items-center gap-4";
    const FIELD_CONTAINER_STYLES: &str = "w-full flex flex-col gap-1";

    html! {
        form .(STYLES) method="POST" {
            h1 .mb-4 { (t!("login.title", locale = locale)) }

            div .(FIELD_CONTAINER_STYLES) {
                label for="email" { (t!("login.email.label", locale = locale)) };
                input #email type="email" name="email" placeholder=(t!("login.email.placeholder", locale = locale));
            }

            div .(FIELD_CONTAINER_STYLES) {
                label for="password" { (t!("login.password.label", locale = locale)) };
                input #password type="password" name="password" placeholder=(t!("login.password.placeholder", locale = locale));
            }

            input .mt-4 type="submit" value=(t!("login.title", locale = locale));

            a href="/register" {
                (t!("login.new-here", locale = locale))
            }
        }
    }
}

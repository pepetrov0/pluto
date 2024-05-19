//! The login page.

use maud::Markup;

use crate::web::_components::{organisms, templates};

/// Constructs the login page.
pub fn login(locale: &str) -> Markup {
    templates::page(locale, "login.title", false, organisms::login_form(locale))
}
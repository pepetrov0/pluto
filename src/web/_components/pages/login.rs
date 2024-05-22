//! The login page.

use maud::Markup;

use crate::{domain::authentication::AuthenticationError, web::_components::{organisms, templates}};

/// Constructs the login page.
pub fn login(locale: &str, error: Option<AuthenticationError>) -> Markup {
    templates::page(locale, "login.title", false, organisms::login_form(locale, error))
}

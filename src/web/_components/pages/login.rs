//! The login page.

use maud::Markup;

use crate::{
    domain::authentication::AuthenticationError,
    web::_components::{organisms, templates},
};

/// Constructs the login page.
pub fn login(locale: &str, error: Option<AuthenticationError>) -> Markup {
    templates::blank_page(locale, "login.title", organisms::login(locale, error))
}

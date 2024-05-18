//! The register page.

use maud::Markup;

use crate::web::_components::{organisms, templates};

/// Constructs the register page.
pub fn register(locale: &str) -> Markup {
    templates::page(
        locale,
        "register.title",
        false,
        organisms::register_form(locale),
    )
}

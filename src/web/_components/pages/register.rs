//! The register page.

use maud::Markup;

use crate::{
    domain::registration::RegistrationError,
    web::_components::{
        organisms::{self, RegisterData},
        templates,
    },
};

/// Constructs the register page.
pub fn register(
    locale: &str,
    data: Option<RegisterData>,
    error: Option<RegistrationError>,
) -> Markup {
    templates::blank_page(
        locale,
        "register.title",
        organisms::register(locale, data, error),
    )
}

//! The register page.

use maud::Markup;

use crate::{
    domain::registration::RegistrationError,
    web::_components::{
        organisms::{self, RegisterFormData},
        templates,
    },
};

/// Constructs the register page.
pub fn register(
    locale: &str,
    data: Option<RegisterFormData>,
    error: Option<RegistrationError>,
) -> Markup {
    templates::blank_page(
        locale,
        "register.title",
        organisms::register_form(locale, data, error),
    )
}

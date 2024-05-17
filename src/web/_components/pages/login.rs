use maud::Markup;

use crate::web::_components::{organisms, templates};

pub fn login(locale: &str) -> Markup {
    templates::page(locale, "login.title", false, organisms::login_form(locale))
}

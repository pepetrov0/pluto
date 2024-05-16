use maud::{html, Markup, DOCTYPE};

use crate::web::_components::{atoms, organisms};

/// A generic page component
pub fn page(locale: &str, title: &str, has_navigation: bool, content: Markup) -> Markup {
    const BODY_STYLES: &str = "bg-stone-200 text-black dark:bg-stone-900 dark:text-white";
    const BODY_WITH_NAVIGATION_STYLES: &str = "pl-12";
    const CONTENT_STYLES: &str = "flex flex-col items-center mx-auto gap-4 p-2 max-w-3xl";

    html! {
        (DOCTYPE)
        html lang=(locale) {
            (atoms::head(locale, title))
            body hx-boost="true" .(BODY_STYLES) .(BODY_WITH_NAVIGATION_STYLES)[has_navigation] {
                @if has_navigation {
                    (organisms::navigation(locale))
                }
                main .(CONTENT_STYLES) {
                    (content)
                }
            }
        }
    }
}

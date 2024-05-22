//! A generic page template.

use maud::{html, Markup, DOCTYPE};
use rust_i18n::t;

use crate::web::{_components::organisms, _core::static_file_url};

/// Constructs a generic page.
pub fn page(locale: &str, title: &str, has_navigation: bool, content: Markup) -> Markup {
    const BODY_STYLES: &str = "bg-stone-200 text-black dark:bg-stone-900 dark:text-white";
    const BODY_WITH_NAVIGATION_STYLES: &str = "pl-12";
    const CONTENT_STYLES: &str = "flex flex-col items-center mx-auto gap-4 p-2 max-w-3xl";

    html! {
        (DOCTYPE)
        html lang=(locale) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="description" content="pluto: a personal finance manager";

                link rel="apple-touch-icon" sizes="180x180" href=(static_file_url("apple-touch-icon.png"));
                link rel="icon" type="image/png" sizes="32x32" href=(static_file_url("favicon-32x32.png"));
                link rel="icon" type="image/png" sizes="16x16" href=(static_file_url("favicon-16x16.png"));
                link rel="manifest" href=(static_file_url("site.webmanifest"));
                link rel="stylesheet" href=(static_file_url("styles.css"));
                script src=(static_file_url("htmx.min.js")) {}

                title { (t!(title, locale = locale)) }
            }
            body .(BODY_STYLES) .(BODY_WITH_NAVIGATION_STYLES)[has_navigation] {
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

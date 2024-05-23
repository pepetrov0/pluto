//! A generic page template.

use maud::{html, Markup, DOCTYPE};
use rust_i18n::t;

use crate::web::{_components::organisms, _core::static_file_url};

const BODY_STYLES: &str = "bg-stone-200 text-black dark:bg-stone-900 dark:text-white";
const CONTENT_STYLES: &str = "flex flex-col items-center mx-auto gap-4 p-4 max-w-3xl [&>*]:w-full";

fn head(locale: &str, title: &str) -> Markup {
    html! {
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
            script defer src=(static_file_url("alpine.min.js")) {}

            title { (t!(title, locale = locale)) }
        }
    }
}

/// Constructs a blank page.
pub fn blank_page(locale: &str, title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang=(locale) {
            (head(locale, title))
            body .(BODY_STYLES) {
                main .(CONTENT_STYLES) {
                    (content)
                }
            }
        }
    }
}

/// Constructs a contentful page.
pub fn page(locale: &str, title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang=(locale) {
            (head(locale, title))
            body .pl-12 .(BODY_STYLES) {
                (organisms::navigation(locale))
                main .(CONTENT_STYLES) {
                    h1 { (t!(title, locale = locale)) }
                    (content)
                }
            }
        }
    }
}

use maud::{html, Markup, DOCTYPE};

use crate::web::static_file_url;

/// The header for every document
fn header(title: &str) -> Markup {
    html! {
        (DOCTYPE)

        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        meta name="description" content="pluto: a personal finance manager";

        link rel="apple-touch-icon" sizes="180x180" href=(static_file_url("apple-touch-icon.png"));
        link rel="icon" type="image/png" sizes="32x32" href=(static_file_url("favicon-32x32.png"));
        link rel="icon" type="image/png" sizes="16x16" href=(static_file_url("favicon-16x16.png"));
        link rel="manifest" href=(static_file_url("site.webmanifest"));

        style { (include_str!("../../../target/styles.css")) }
        title { (title) }
    }
}

/// A generic page component
pub fn page(title: &str, content: Markup) -> Markup {
    const BODY_STYLES: &str =
        "min-w-dvw min-h-dvh max-w-dvw max-h-dvh bg-gray-200 dark:bg-gray-700";

    html! {
        (header(title))
        body .(BODY_STYLES) {
            main {
                (content)
            }
        }
    }
}

use maud::{html, Markup, DOCTYPE};
use crate::web::static_file_url;

/// The header for every document
pub fn header(title: &str) -> Markup {
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
        script src=(static_file_url("htmx.min.js")) {}
        title { (title) }
    }
}
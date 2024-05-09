#![allow(unused)]

use maud::{html, Markup, DOCTYPE};

fn header(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        style { (include_str!("../../../target/styles.css")) }
        title { (title) }
    }
}

pub fn page(title: &str) -> Markup {
    const STYLES: &str = "min-w-dvw min-h-dvh max-w-dvw max-h-dvh bg-gray-200 dark:bg-gray-700";

    html! {
        (header(title))
        body .(STYLES) { }
    }
}

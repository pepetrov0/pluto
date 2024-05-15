use maud::{html, Markup};

/// A generic page component
pub fn page(title: &str, has_navigation: bool, content: Markup) -> Markup {
    const BODY_STYLES: &str = "bg-root-background text-root-background-contrast dark:bg-root-background-dark dark:text-root-background-contrast-dark";
    const BODY_WITH_NAVIGATION_STYLES: &str = "pl-12";
    const CONTENT_STYLES: &str = "max-w-3xl mx-auto p-2";

    html! {
        (super::header::header(title))
        body hx-boost="true" .(BODY_STYLES) .(BODY_WITH_NAVIGATION_STYLES)[has_navigation] {
            @if has_navigation {
                (super::navigation::navigation())
            }
            main .(CONTENT_STYLES) {
                (content)
            }
        }
    }
}

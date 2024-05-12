use maud::{html, Markup};

/// A generic page component
pub fn page(title: &str, navigation: bool, content: Markup) -> Markup {
    const BODY_STYLES: &str = "bg-root-background text-root-background-contrast dark:bg-root-background-dark dark:text-root-background-contrast-dark";
    const CONTENT_STYLES: &str = "max-w-3xl mx-auto p-2";

    html! {
        (super::header::header(title))
        body .(BODY_STYLES) {
            @if navigation {
                (super::navigation::navigation())
            }
            main .(CONTENT_STYLES) {
                (content)
            }
        }
    }
}

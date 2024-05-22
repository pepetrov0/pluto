//! Implementation of the navigation of the application.

use maud::{html, Markup};
use rust_i18n::t;

use crate::web::_components::atoms::Icon;

/// Used internally to construct the header of the navigation,
/// which contains the toggle button and the application title as link to the home page.
fn header() -> Markup {
    const STYLES: &str = "flex flex-row";

    // toggle styles
    const TOGGLE_CONTAINER_STYLES: &str = concat!(
        "w-12 h-12 p-3 xl:hidden ",
        "transition-colors hover:bg-black hover:text-white hover:dark:bg-white hover:dark:text-black"
    );
    const TOGGLE_STYLES: &str = "absolute left-0 top-0 w-12 h-12 opacity-0 cursor-pointer z-[55]";

    // home link styles
    const HOME_LINK_STYLES: &str = concat!(
        "p-0 no-underline ",
        "h-12 flex-grow items-center justify-center hidden xl:flex group-has-[#nav-toggle:checked]/nav:flex ",
        "font-bold hover:cursor-pointer hover:bg-black hover:text-white hover:dark:bg-white hover:dark:text-black"
    );

    html! {
        div .(STYLES) {
            div .(TOGGLE_CONTAINER_STYLES) {
                input #nav-toggle type="checkbox" aria-label="menu" x-model="open" .(TOGGLE_STYLES);
                span ."inline group-has-[#nav-toggle:checked]/nav:hidden" {
                    (Icon::Menu)
                }
                span ."hidden group-has-[#nav-toggle:checked]/nav:inline" {
                    (Icon::X)
                }
            }
            a href="/" .(HOME_LINK_STYLES) {
                span { (crate::NAME) }
            }
        }
    }
}

/// An internal component which is a row in the navigation.
/// Contains a link and an icon.
fn item(locale: &str, icon: Icon, text: &str, url: &str) -> Markup {
    const STYLES: &str = concat!(
        "p-0 no-underline ",
        "flex flex-row z-50 ",
        "hover:cursor-pointer hover:bg-black hover:text-white hover:dark:bg-white hover:dark:text-black"
    );
    const ICON_STYLES: &str = "w-12 h-12 p-3";
    const TEXT_STYLES: &str =
        "flex-grow items-center hidden xl:flex group-has-[#nav-toggle:checked]/nav:flex";

    html! {
        a .(STYLES) href=(url) {
            div .(ICON_STYLES) {
                (icon)
            }
            div .(TEXT_STYLES) {
                span { (t!(text, locale = locale)) }
            }
        }
    }
}

/// Constructs the navigation of the application.
pub fn navigation(locale: &str) -> Markup {
    const STYLES: &str = concat!(
        "fixed left-0 top-0 h-dvh xl:w-full has-[#nav-toggle:checked]:w-full max-w-[min(16rem,90dvw)] overflow-y-auto ",
        "flex flex-col group/nav ",
        "bg-white text-black dark:bg-stone-800 dark:text-white"
    );

    html! {
        nav .(STYLES) x-data="{ open: false }" "@click.outside"="open = false" {
            (header())
            (item(locale, Icon::Newspaper, "dashboard.title", "/"))
        }
    }
}

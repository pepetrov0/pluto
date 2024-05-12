use maud::{html, Markup};

pub fn navigation() -> Markup {
    const STYLES: &str = concat!(
        "fixed left-0 top-0 h-dvh w-[min(16rem,90dvw)] ",
        "bg-background text-background-contrast dark:bg-background-dark dark:text-background-contrast-dark "
    );

    html! {
        nav .(STYLES) {

        }
    }
}

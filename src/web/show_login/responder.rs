use axum::response::{IntoResponse, Redirect, Response};
use maud::html;
use rust_i18n::t;

use crate::web::_components;

pub async fn invoke(locale: &str, is_authorized: bool) -> Response {
    if is_authorized {
        return Redirect::to("/").into_response();
    }

    const STYLES: &str = "card mt-4 w-full max-w-md flex flex-col items-center gap-4";
    const FIELD_CONTAINER_STYLES: &str = "w-full flex flex-col gap-1";
    let content = html! {
        form .(STYLES) {
            h1 .mb-4 { (t!("login.title", locale = locale)) }

            div .(FIELD_CONTAINER_STYLES) {
                label for="email" { (t!("login.email.label", locale = locale)) };
                input #email type="email" name="email" placeholder=(t!("login.email.placeholder", locale = locale));
            }    

            div .(FIELD_CONTAINER_STYLES) {
                label for="password" { (t!("login.password.label", locale = locale)) };
                input #password type="password" name="password" placeholder=(t!("login.password.placeholder", locale = locale));
            }        

            input .mt-4 type="submit" value=(t!("login.title", locale = locale));
        }
        a href="/register" {
            (t!("login.new-here", locale = locale))
        }
    };
    _components::page(locale, "login.title", false, content).into_response()
}

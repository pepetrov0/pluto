//! Organisms

mod login_form;
mod navigation;
mod register_form;

pub use login_form::login_form;
pub(in crate::web::_components) use navigation::navigation;
pub use register_form::register_form;

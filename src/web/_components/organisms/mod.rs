//! Organisms

mod login_form;
mod navigation;
mod register_form;
mod profile_details_card;

pub use login_form::*;
pub(in crate::web::_components) use navigation::navigation;
pub use profile_details_card::*;
pub use register_form::*;

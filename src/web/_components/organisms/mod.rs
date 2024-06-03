//! Organisms

mod change_email;
mod change_password;
mod danger_zone;
mod login;
mod navigation;
mod profile_details;
mod register;

pub use change_email::*;
pub use change_password::*;
pub use danger_zone::*;
pub use login::*;
pub(in crate::web::_components) use navigation::navigation;
pub use profile_details::*;
pub use register::*;

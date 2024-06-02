//! This module implements the domain/business logic of the application.
//!
//! As per wikipedia:
//! The domain can modify state, interacting with storage and/or
//! manipulating data as needed. It contains the business logic.
//!

pub mod authentication;
pub mod authorization;
pub mod change_email;
pub mod change_password;
mod configuration;
pub mod database;
mod identifier;
pub mod keys;
pub mod logout;
pub mod passwords;
pub mod registration;
mod sessions;
pub mod shutdown;
mod users;
pub mod validations;

pub use configuration::Configuration;
pub use identifier::Id;
pub use sessions::Session;
pub use users::User;

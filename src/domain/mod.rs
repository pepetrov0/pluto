//! This module implements the domain/business logic of the application.
//!
//! As per wikipedia:
//! The domain can modify state, interacting with storage and/or
//! manipulating data as needed. It contains the business logic.
//!

pub mod authentication;
mod configuration;
pub mod database;
mod identifier;
pub mod keys;
pub mod passwords;
pub mod registration;
pub mod sessions;
pub mod shutdown;
pub mod users;
pub mod validations;

pub use configuration::Configuration;
pub use identifier::Id;

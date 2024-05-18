//! pluto is a personal finance management application
//!
//! # STRUCTURE
//! pluto follows Action-Domain-Response pattern where the domain logic is 
//! separated from the the representation logic and thus the core is 
//! separated into two modules: domain and web.
//!
//! The domain logic lives in the `domain` module and is further divided
//! by features and/or layer.
//!
//! The presentation logic lives in the `web` module and is further divided 
//! into features - which in turn are divided into action and responder pair.
//!

#![deny(unused)]

pub mod domain;
pub mod web;

pub const NAME: &str = env!("CARGO_PKG_NAME");

// This intializes the locales.
rust_i18n::i18n!("locales");

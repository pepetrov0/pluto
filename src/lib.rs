//! pluto is a personal finance management application

#![deny(unused)]

pub mod domain;
pub mod web;

const NAME: &str = env!("CARGO_PKG_NAME");

// init i18n
rust_i18n::i18n!("locales");

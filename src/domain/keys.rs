//! This module deals with deriving keys from configuration.

use argon2::Argon2;
use axum_extra::extract::cookie;

use super::configuration::Configuration;

/// Generate a cookie key based on the configuration.
pub fn cookie_key(configuration: &Configuration) -> cookie::Key {
    match configuration.secret.as_ref() {
        Some(secret) => {
            let mut key_material = [0u8; 32];
            match Argon2::default()
                .hash_password_into(secret.as_bytes(), secret.as_bytes(), &mut key_material)
                .is_ok()
            {
                true => cookie::Key::derive_from(&key_material),
                false => cookie::Key::generate(),
            }
        }
        None => cookie::Key::generate(),
    }
}

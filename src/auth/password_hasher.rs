//! Enables password hashing

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash,
};

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: String) -> Option<String>;
    fn verify(&self, password: String, hash: String) -> bool;
}

impl PasswordHasher for Argon2<'_> {
    fn hash(&self, password: String) -> Option<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2::PasswordHasher::hash_password(self, password.as_bytes(), &salt)
            .ok()?
            .to_string();
        Some(hash)
    }

    fn verify(&self, password: String, hash: String) -> bool {
        let hash = match PasswordHash::new(&hash) {
            Ok(v) => v,
            Err(_) => return false,
        };
        argon2::PasswordVerifier::verify_password(self, password.as_bytes(), &hash).is_ok()
    }
}

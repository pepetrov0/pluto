//! Implements password hashing

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash,
};

/// Provides creation and verification of password hashes
pub trait PasswordHasher: Send + Sync {
    /// Hashes a password
    fn hash(&self, password: &[u8]) -> Option<String>;
    /// Verifies that a password and a hash match
    fn verify(&self, password: &[u8], hash: &str) -> bool;
}

impl PasswordHasher for Argon2<'_> {
    #[tracing::instrument(skip(password))]
    fn hash(&self, password: &[u8]) -> Option<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2::PasswordHasher::hash_password(self, password, &salt)
            .ok()?
            .to_string();
        Some(hash)
    }

    #[tracing::instrument(skip(password))]
    fn verify(&self, password: &[u8], hash: &str) -> bool {
        let hash = match PasswordHash::new(hash) {
            Ok(v) => v,
            Err(_) => return false,
        };
        argon2::PasswordVerifier::verify_password(self, password, &hash).is_ok()
    }
}

//! Implements password hashing and verification.

use argon2::{
    password_hash::{self, rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

#[derive(Debug, Clone)]
pub struct PasswordError;

/// Hashes a password for storing in a database.
pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    argon
        .hash_password(password.as_bytes(), &salt)
        .map(|v| v.to_string())
        .map_err(PasswordError::from)
}

/// Verifies that a password and a hash match.
pub fn verify_password(password: &str, hash: &str) -> Result<(), PasswordError> {
    let argon = Argon2::default();
    let hash = PasswordHash::new(hash).map_err(PasswordError::from)?;
    argon
        .verify_password(password.as_bytes(), &hash)
        .map_err(PasswordError::from)
}

impl std::error::Error for PasswordError {}
impl std::fmt::Display for PasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<password_hash::Error> for PasswordError {
    fn from(_: password_hash::Error) -> Self {
        Self
    }
}

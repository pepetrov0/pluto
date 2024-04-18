//! A module providing a concept of a user.

use axum::async_trait;

/// An entity representing a user.
#[derive(Debug, Clone, sqlx::FromRow, Eq, PartialEq)]
pub struct User {
    /// Identifier of the user.
    pub id: i32,
    /// Email.
    pub email: String,
    /// Argon hash of the password.
    pub password: Option<String>,
}

/// A trait describing a repository of users.
#[async_trait]
pub trait Users {
    /// Finds a user by identifier.
    async fn find_user_by_id(&mut self, id: i32) -> Option<Option<User>>;

    /// Finds a user by email.
    async fn find_user_by_email(&mut self, email: &str) -> Option<Option<User>>;

    /// Create a user.
    async fn create_user(&mut self, email: &str, password: Option<&str>) -> Option<User>;
}


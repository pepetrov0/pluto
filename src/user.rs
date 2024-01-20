//! Implements a user component

use axum::async_trait;
use sqlx::FromRow;

/// Represents a user
#[derive(Debug, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
}

/// Represents a user with password
#[derive(Debug, FromRow)]
pub struct UserWithPassword {
    pub id: String,
    pub email: String,
    pub password: Option<String>
}

/// A user repository
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Creates a user
    async fn create_user(&self, email: String, password: Option<String>) -> Option<User>;

    /// Finds a user by id or by email
    async fn find_user(&self, id_or_email: &str) -> Option<User>;

    /// Retrieve a user bundled with their password
    async fn find_user_with_password(&self, id_or_email: &str) -> Option<UserWithPassword>;
}

#[async_trait]
impl UserRepository for sqlx::PgPool {
    #[tracing::instrument]
    async fn create_user(&self, email: String, password: Option<String>) -> Option<User> {
        sqlx::query_as::<_, User>(
            "insert into users (id, email, password) values ($1, $2, $3) returning id, email",
        )
        .bind(nanoid::nanoid!())
        .bind(email)
        .bind(password)
        .fetch_one(self)
        .await
        .ok()
    }

    #[tracing::instrument]
    async fn find_user(&self, id_or_email: &str) -> Option<User> {
        sqlx::query_as::<_, User>("select id, email from users where id=$1 or email=$1")
            .bind(id_or_email)
            .fetch_one(self)
            .await
            .ok()
    }

    #[tracing::instrument]
    async fn find_user_with_password(&self, id_or_email: &str) -> Option<UserWithPassword> {
        sqlx::query_as::<_, UserWithPassword>("select id, email, password from users where id=$1 or email=$1")
            .bind(id_or_email)
            .fetch_one(self)
            .await
            .ok()
    }
}

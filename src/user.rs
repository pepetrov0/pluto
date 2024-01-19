//! Implements a user component

use axum::async_trait;
use sqlx::FromRow;

/// Represents a user
#[derive(FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
}

/// A user repository
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Creates a user
    async fn create_user(&self, email: String, password: Option<String>) -> Option<User>;

    /// Finds a user by id or by email
    async fn find_user(&self, id_or_email: &str) -> Option<User>;
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
}

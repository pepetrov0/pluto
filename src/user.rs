//! Implements a user component

use axum::async_trait;
use chrono_tz::Tz;
use sqlx::FromRow;

/// Represents a user
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub timezone: String,
}

/// Represents a user with password
#[derive(Debug, FromRow)]
pub struct UserWithPassword {
    pub id: String,
    pub email: String,
    pub password: Option<String>,
    pub timezone: String,
}

/// A user repository
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Creates a user
    async fn create_user(
        &self,
        email: String,
        password: Option<String>,
        timezone: Tz,
    ) -> Option<User>;

    /// Finds a user by id or by email
    async fn find_user(&self, id_or_email: &str) -> Option<User>;

    /// Retrieve a user bundled with their password
    async fn find_user_with_password(&self, id_or_email: &str) -> Option<UserWithPassword>;

    /// Lists all users
    async fn list_users(&self) -> Option<Vec<User>>;

    /// Lists all users by IDs
    async fn list_users_by_ids(&self, ids: Vec<String>) -> Option<Vec<User>>;
}

#[async_trait]
impl UserRepository for sqlx::PgPool {
    #[tracing::instrument]
    async fn create_user(
        &self,
        email: String,
        password: Option<String>,
        timezone: Tz,
    ) -> Option<User> {
        sqlx::query_as::<_, User>(
            "insert into users (id, email, password, timezone) values ($1, $2, $3, $4) returning id, email, timezone",
        )
        .bind(nanoid::nanoid!())
        .bind(email)
        .bind(password)
        .bind(timezone.name())
        .fetch_one(self)
        .await
        .ok()
    }

    #[tracing::instrument]
    async fn find_user(&self, id_or_email: &str) -> Option<User> {
        sqlx::query_as::<_, User>("select id, email, timezone from users where id=$1 or email=$1")
            .bind(id_or_email)
            .fetch_one(self)
            .await
            .ok()
    }

    #[tracing::instrument]
    async fn find_user_with_password(&self, id_or_email: &str) -> Option<UserWithPassword> {
        sqlx::query_as::<_, UserWithPassword>(
            "select id, email, password, timezone from users where id=$1 or email=$1",
        )
        .bind(id_or_email)
        .fetch_one(self)
        .await
        .ok()
    }

    #[tracing::instrument]
    async fn list_users(&self) -> Option<Vec<User>> {
        sqlx::query_as::<_, User>("select id, email, timezone from users order by email")
            .fetch_all(self)
            .await
            .ok()
    }

    #[tracing::instrument]
    async fn list_users_by_ids(&self, ids: Vec<String>) -> Option<Vec<User>> {
        if ids.is_empty() {
            return Some(vec![]);
        }

        sqlx::query_as::<_, User>(
            "select id, email, timezone from users where id=ANY($1) order by email",
        )
        .bind(&ids[..])
        .fetch_all(self)
        .await
        .ok()
    }
}

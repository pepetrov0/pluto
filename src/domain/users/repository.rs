use axum::async_trait;
use chrono_tz::Tz;
use sqlx::Postgres;

use crate::database::{AsReadonlyExecutor, AsWriteExecutor};

use super::{User, UserWithPassword};

/// A user readonly repository
#[async_trait]
pub trait UserReadonlyRepository {
    /// Finds a user by id or by email
    async fn find_user(&mut self, id_or_email: &str) -> Option<User>;

    /// Retrieve a user bundled with their password
    async fn find_user_with_password(&mut self, id_or_email: &str) -> Option<UserWithPassword>;

    /// Lists all users
    async fn list_users(&mut self) -> Option<Vec<User>>;

    /// Lists all users by IDs
    async fn list_users_by_ids_or_emails(&mut self, ids: &[String]) -> Option<Vec<User>>;
}

/// A user write repository
#[async_trait]
pub trait UserWriteRepository {
    /// Creates a user
    async fn create_user(
        &mut self,
        email: &str,
        password: Option<String>,
        timezone: Tz,
        favorite_asset: &str,
        favorite_account: &str,
    ) -> Option<User>;
}

#[async_trait]
impl<T> UserReadonlyRepository for T
where
    T: AsReadonlyExecutor<Postgres> + Send + std::fmt::Debug,
{
    #[tracing::instrument]
    async fn find_user(&mut self, id_or_email: &str) -> Option<User> {
        sqlx::query_as::<_, User>("select id, email, timezone, favorite_asset, favorite_account from users where id=$1 or email=$1")
            .bind(id_or_email)
            .fetch_one(self.as_executor())
            .await
            .map_err(|v| tracing::warn!("{:#?}", v))
            .ok()
    }

    #[tracing::instrument]
    async fn find_user_with_password(&mut self, id_or_email: &str) -> Option<UserWithPassword> {
        sqlx::query_as::<_, UserWithPassword>(
            "select id, email, password, timezone, favorite_asset, favorite_account from users where id=$1 or email=$1",
        )
        .bind(id_or_email)
        .fetch_one(self.as_executor())
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }

    #[tracing::instrument]
    async fn list_users(&mut self) -> Option<Vec<User>> {
        sqlx::query_as::<_, User>(
            "select id, email, timezone, favorite_asset, favorite_account from users",
        )
        .fetch_all(self.as_executor())
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }

    #[tracing::instrument]
    async fn list_users_by_ids_or_emails(&mut self, ids_or_emails: &[String]) -> Option<Vec<User>> {
        if ids_or_emails.is_empty() {
            return Some(vec![]);
        }

        sqlx::query_as::<_, User>(
            "select id, email, timezone, favorite_asset, favorite_account from users where id=ANY($1) or email=ANY($1)",
        )
        .bind(ids_or_emails)
        .fetch_all(self.as_executor())
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }
}

#[async_trait]
impl<T> UserWriteRepository for T
where
    T: AsWriteExecutor<Postgres> + Send + std::fmt::Debug,
{
    #[tracing::instrument]
    async fn create_user(
        &mut self,
        email: &str,
        password: Option<String>,
        timezone: Tz,
        favorite_asset: &str,
        favorite_account: &str,
    ) -> Option<User> {
        sqlx::query_as::<_, User>(
            "insert into users (id, email, password, timezone, favorite_asset, favorite_account) values ($1, $2, $3, $4, $5, $6) returning id, email, timezone, favorite_asset, favorite_account",
        )
        .bind(nanoid::nanoid!())
        .bind(email)
        .bind(password)
        .bind(timezone.name())
        .bind(favorite_asset)
        .bind(favorite_account)
        .fetch_one(self.as_executor())
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }
}

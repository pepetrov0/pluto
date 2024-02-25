use axum::async_trait;
use chrono_tz::Tz;

use crate::core::database::{
    IntoRepositoryResult, ReadonlyDatabaseRepository, RepositoryResult, WriteDatabaseRepository,
};

use super::{User, UserWithPassword};

#[async_trait]
pub trait UserReadonlyRepository {
    async fn find_user(&mut self, id_or_email: &str) -> RepositoryResult<Option<User>>;

    async fn find_user_with_password(
        &mut self,
        id_or_email: &str,
    ) -> RepositoryResult<Option<UserWithPassword>>;

    async fn list_users(&mut self) -> RepositoryResult<Vec<User>>;

    async fn list_users_by_ids_or_emails(&mut self, ids: &[String]) -> RepositoryResult<Vec<User>>;
}

#[async_trait]
pub trait UserWriteRepository {
    async fn create_user(
        &mut self,
        email: &str,
        password: Option<String>,
        timezone: Tz,
        favorite_asset: &str,
        favorite_account: &str,
    ) -> RepositoryResult<User>;
}

#[async_trait]
impl<T> UserReadonlyRepository for T
where
    T: ReadonlyDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument(err)]
    async fn find_user(&mut self, id_or_email: &str) -> RepositoryResult<Option<User>> {
        sqlx::query_as::<_, User>("select id, email, timezone, favorite_asset, favorite_account from users where id=$1 or email=$1")
            .bind(id_or_email)
            .fetch_optional(self.acquire().await?)
            .await
            .into_repository_result()
    }

    #[tracing::instrument(err)]
    async fn find_user_with_password(
        &mut self,
        id_or_email: &str,
    ) -> RepositoryResult<Option<UserWithPassword>> {
        sqlx::query_as::<_, UserWithPassword>(
            "select id, email, password, timezone, favorite_asset, favorite_account from users where id=$1 or email=$1",
        )
        .bind(id_or_email)
        .fetch_optional(self.acquire().await?)
        .await
        .into_repository_result()
    }

    #[tracing::instrument(err)]
    async fn list_users(&mut self) -> RepositoryResult<Vec<User>> {
        sqlx::query_as::<_, User>(
            "select id, email, timezone, favorite_asset, favorite_account from users",
        )
        .fetch_all(self.acquire().await?)
        .await
        .into_repository_result()
    }

    #[tracing::instrument(err)]
    async fn list_users_by_ids_or_emails(
        &mut self,
        ids_or_emails: &[String],
    ) -> RepositoryResult<Vec<User>> {
        if ids_or_emails.is_empty() {
            return Ok(vec![]);
        }

        sqlx::query_as::<_, User>(
            "select id, email, timezone, favorite_asset, favorite_account from users where id=ANY($1) or email=ANY($1)",
        )
        .bind(ids_or_emails)
        .fetch_all(self.acquire().await?)
        .await
        .into_repository_result()
    }
}

#[async_trait]
impl<T> UserWriteRepository for T
where
    T: WriteDatabaseRepository + Send + std::fmt::Debug,
{
    #[tracing::instrument(err)]
    async fn create_user(
        &mut self,
        email: &str,
        password: Option<String>,
        timezone: Tz,
        favorite_asset: &str,
        favorite_account: &str,
    ) -> RepositoryResult<User> {
        sqlx::query_as::<_, User>(
            "insert into users (id, email, password, timezone, favorite_asset, favorite_account) values ($1, $2, $3, $4, $5, $6) returning id, email, timezone, favorite_asset, favorite_account",
        )
        .bind(nanoid::nanoid!())
        .bind(email)
        .bind(password)
        .bind(timezone.name())
        .bind(favorite_asset)
        .bind(favorite_account)
        .fetch_one(self.acquire().await?)
        .await
        .into_repository_result()
    }
}

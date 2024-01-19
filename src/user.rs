use axum::async_trait;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(
        &self,
        email: String,
        password: Option<String>,
    ) -> Result<User, sqlx::Error>;

    async fn find_user_by_email(&self, email: &str) -> Result<User, sqlx::Error>;
}

#[async_trait]
impl UserRepository for sqlx::PgPool {
    #[tracing::instrument(err)]
    async fn create_user(
        &self,
        email: String,
        password: Option<String>,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "insert into users (id, email, password) values ($1, $2, $3) returning id, email",
        )
        .bind(nanoid::nanoid!())
        .bind(email)
        .bind(password)
        .fetch_one(self)
        .await
    }

    #[tracing::instrument(err)]
    async fn find_user_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "select id, email from users where email=$1",
        )
        .bind(email)
        .fetch_one(self)
        .await
    }
}

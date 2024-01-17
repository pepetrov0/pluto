use axum::async_trait;
use sqlx::FromRow;

use crate::database::Database;

#[derive(FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, email: String, password: String) -> Result<User, sqlx::Error>;
}

#[async_trait]
impl UserRepository for Database {
    async fn create_user(&self, email: String, password: String) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "insert into users (id, email, password) values ($1, $2, crypt($3, gen_salt('bf'))) returning (id, email)",
        ).bind(nanoid::nanoid!()).bind(email).bind(password).fetch_one(self).await
    }
}
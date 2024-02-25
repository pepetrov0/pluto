use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub timezone: String,
    pub favorite_asset: String,
    pub favorite_account: String,
}

#[derive(Debug, FromRow)]
pub struct UserWithPassword {
    pub id: String,
    pub email: String,
    pub password: Option<String>,
    pub timezone: String,
    pub favorite_asset: String,
    pub favorite_account: String,
}

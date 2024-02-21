use sqlx::FromRow;

/// Represents a user
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub timezone: String,
    pub favorite_asset: String,
    pub favorite_account: String,
}

/// Represents a user with password
#[derive(Debug, FromRow)]
pub struct UserWithPassword {
    pub id: String,
    pub email: String,
    pub password: Option<String>,
    pub timezone: String,
    pub favorite_asset: String,
    pub favorite_account: String,
}

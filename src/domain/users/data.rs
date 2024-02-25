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

impl From<UserWithPassword> for User {
    fn from(value: UserWithPassword) -> Self {
        Self {
            id: value.id,
            email: value.email,
            timezone: value.timezone,
            favorite_asset: value.favorite_asset,
            favorite_account: value.favorite_account,
        }
    }
}

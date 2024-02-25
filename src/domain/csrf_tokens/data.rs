use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct CsrfToken {
    pub id: String,
    pub usr: String,
    pub usage: String,
}

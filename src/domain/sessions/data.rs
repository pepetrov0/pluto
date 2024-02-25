use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Session {
    pub id: String,
    pub usr: String,
}

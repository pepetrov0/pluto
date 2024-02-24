use sqlx::FromRow;

/// Represents a CSRF token
#[derive(Debug, Clone, FromRow)]
pub struct CsrfToken {
    pub id: String,
    pub usr: String,
    pub usage: String,
}

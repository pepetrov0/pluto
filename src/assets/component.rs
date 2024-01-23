//! Implements the asset component

use axum::async_trait;
use sqlx::{prelude::FromRow, PgPool, Type};

/// Represents an asset type
#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(type_name = "asset_type", rename_all = "snake_case")]
pub enum AssetType {
    Currency,
}

/// Represents an asset
#[derive(Debug, Clone, FromRow)]
pub struct Asset {
    pub id: String,
    pub ticker: String,
    pub symbol: Option<String>,
    pub label: String,
    pub precision: i16,
    pub atype: AssetType,
}

/// An asset repository
#[async_trait]
pub trait AssetRepository: Send + Sync {
    // lists all assets
    async fn list_assets(&self) -> Option<Vec<Asset>>;
}

#[async_trait]
impl AssetRepository for PgPool {
    async fn list_assets(&self) -> Option<Vec<Asset>> {
        sqlx::query_as::<_, Asset>(
            "select id, ticker, symbol, label, precision, atype from assets order by label",
        )
        .fetch_all(self)
        .await
        .ok()
    }
}

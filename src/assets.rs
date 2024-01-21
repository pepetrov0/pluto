//! Implements the asset component

use axum::async_trait;
use sqlx::{prelude::FromRow, Type, PgPool};

/// Represents an asset type
#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "asset_type", rename_all = "snake_case")]
pub enum AssetType {
    Currency,
}

/// Represents an asset
#[derive(Debug, Clone, FromRow)]
pub struct Asset {
    pub id: String,
    pub symbol: String,
    pub label: String,
    pub precision: u8,
    pub atype: AssetType
}

/// An asset repository
#[async_trait]
pub trait AssetRepository: Send + Sync {
}

#[async_trait]
impl AssetRepository for PgPool {

}
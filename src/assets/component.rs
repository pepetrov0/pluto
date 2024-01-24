//! Implements the asset component

use axum::async_trait;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool, Type};

/// Represents an asset type
#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "asset_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    Currency,
}

/// Represents an asset
#[derive(Debug, Clone, FromRow)]
pub struct Asset {
    pub id: String,
    pub ticker: String,
    pub symbol: String,
    pub label: String,
    pub precision: i16,
    pub atype: AssetType,
}

/// An asset repository
#[async_trait]
pub trait AssetRepository: Send + Sync {
    /// lists all assets
    async fn list_assets(&self) -> Option<Vec<Asset>>;

    /// lists all assets filtered by id
    async fn list_assets_by_ids(&self, ids: Vec<String>) -> Option<Vec<Asset>>;

    /// find an asset by ticker
    async fn find_asset_by_ticker(&self, ticker: &str) -> Option<Asset>;

    /// create an asset
    async fn create_asset(
        &self,
        ticker: String,
        symbol: String,
        label: String,
        precision: i16,
        atype: AssetType,
    ) -> Option<Asset>;
}

#[async_trait]
impl AssetRepository for PgPool {
    #[tracing::instrument]
    async fn list_assets(&self) -> Option<Vec<Asset>> {
        sqlx::query_as::<_, Asset>(
            "select id, ticker, symbol, label, precision, atype from assets order by label",
        )
        .fetch_all(self)
        .await
        .ok()
    }

    #[tracing::instrument]
    async fn list_assets_by_ids(&self, ids: Vec<String>) -> Option<Vec<Asset>> {
        if ids.is_empty() {
            return Some(vec![]);
        }

        sqlx::query_as::<_, Asset>(
            "select id, ticker, symbol, label, precision, atype from assets where id=ANY($1) order by label",
        )
        .bind(&ids[..])
        .fetch_all(self)
        .await
        .ok()
    }

    #[tracing::instrument]
    async fn find_asset_by_ticker(&self, ticker: &str) -> Option<Asset> {
        sqlx::query_as::<_, Asset>(
            "select id, ticker, symbol, label, precision, atype from assets where ticker=$1",
        )
        .bind(ticker)
        .fetch_one(self)
        .await
        .ok()
    }

    #[tracing::instrument]
    async fn create_asset(
        &self,
        ticker: String,
        symbol: String,
        label: String,
        precision: i16,
        atype: AssetType,
    ) -> Option<Asset> {
        sqlx::query_as::<_, Asset>(
            "insert into assets (id, ticker, symbol, label, precision, atype) values ($1, $2, $3, $4, $5, $6) returning id, ticker, symbol, label, precision, atype",
        )
        .bind(nanoid!())
        .bind(ticker)
        .bind(symbol)
        .bind(label)
        .bind(precision)
        .bind(atype)
        .fetch_one(self)
        .await
        .ok()
    }
}

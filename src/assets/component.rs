//! Implements the asset component

use axum::async_trait;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres, Type};

use crate::database::{ReadonlyDatabaseRepository, WriteDatabaseRepository};

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
    pub symbol: Option<String>,
    pub label: String,
    pub precision: i16,
    pub atype: AssetType,
}

/// An asset readonly repository
#[async_trait]
pub trait AssetReadonlyRepository {
    /// lists all assets
    async fn list_assets(&mut self) -> Option<Vec<Asset>>;

    /// lists all assets filtered by id
    async fn list_assets_by_ids(&mut self, ids: Vec<String>) -> Option<Vec<Asset>>;

    /// find an asset by id or ticker
    async fn find_asset(&mut self, id_or_ticker: &str) -> Option<Asset>;
}

/// An asset write repository
#[async_trait]
pub trait AssetWriteRepository {
    /// create an asset
    async fn create_asset(
        &mut self,
        ticker: String,
        symbol: Option<String>,
        label: String,
        precision: i16,
        atype: AssetType,
    ) -> Option<Asset>;
}

#[async_trait]
impl<T> AssetReadonlyRepository for T
where
    T: ReadonlyDatabaseRepository<Postgres> + std::fmt::Debug + Send,
{
    #[tracing::instrument]
    async fn list_assets(&mut self) -> Option<Vec<Asset>> {
        sqlx::query_as::<_, Asset>(
            "select id, ticker, symbol, label, precision, atype from assets order by label",
        )
        .fetch_all(self.acquire().await?)
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }

    #[tracing::instrument]
    async fn list_assets_by_ids(&mut self, ids: Vec<String>) -> Option<Vec<Asset>> {
        if ids.is_empty() {
            return Some(vec![]);
        }

        sqlx::query_as::<_, Asset>(
            "select id, ticker, symbol, label, precision, atype from assets where id=ANY($1) order by label",
        )
        .bind(&ids[..])
        .fetch_all(self.acquire().await?)
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }

    #[tracing::instrument]
    async fn find_asset(&mut self, id_or_ticker: &str) -> Option<Asset> {
        sqlx::query_as::<_, Asset>(
            "select id, ticker, symbol, label, precision, atype from assets where id=$1 or ticker=$1",
        )
        .bind(id_or_ticker)
        .fetch_one(self.acquire().await?)
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }
}

#[async_trait]
impl<T> AssetWriteRepository for T
where
    T: WriteDatabaseRepository<Postgres> + std::fmt::Debug + Send,
{
    #[tracing::instrument]
    async fn create_asset(
        &mut self,
        ticker: String,
        symbol: Option<String>,
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
        .fetch_one(self.acquire().await?)
        .await
        .map_err(|v| tracing::warn!("{:#?}", v))
        .ok()
    }
}

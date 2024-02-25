use axum::async_trait;

use crate::database::{ReadonlyDatabaseRepository, WriteDatabaseRepository};

use super::data::{Asset, AssetType};

/// An asset readonly repository
#[async_trait]
pub trait AssetReadonlyRepository {
    /// lists all assets
    async fn list_assets(&mut self) -> Option<Vec<Asset>>;

    /// lists all assets filtered by id
    async fn list_assets_by_ids_or_tickers(
        &mut self,
        ids_or_tickers: &[String],
    ) -> Option<Vec<Asset>>;

    /// find an asset by id or ticker
    async fn find_asset(&mut self, id_or_ticker: &str) -> Option<Asset>;
}

/// An asset write repository
#[async_trait]
pub trait AssetWriteRepository {
    /// create an asset
    async fn create_asset(
        &mut self,
        ticker: &str,
        symbol: Option<&str>,
        label: &str,
        precision: i16,
        atype: AssetType,
    ) -> Option<Asset>;
}

#[async_trait]
impl<T> AssetReadonlyRepository for T
where
    T: ReadonlyDatabaseRepository + std::fmt::Debug + Send,
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
    async fn list_assets_by_ids_or_tickers(
        &mut self,
        ids_or_tickers: &[String],
    ) -> Option<Vec<Asset>> {
        if ids_or_tickers.is_empty() {
            return Some(vec![]);
        }

        sqlx::query_as::<_, Asset>(
            "select id, ticker, symbol, label, precision, atype from assets where id=ANY($1) or ticker=ANY($1) order by label",
        )
        .bind(ids_or_tickers)
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
    T: WriteDatabaseRepository + std::fmt::Debug + Send,
{
    #[tracing::instrument]
    async fn create_asset(
        &mut self,
        ticker: &str,
        symbol: Option<&str>,
        label: &str,
        precision: i16,
        atype: AssetType,
    ) -> Option<Asset> {
        sqlx::query_as::<_, Asset>(
            "insert into assets (id, ticker, symbol, label, precision, atype) values ($1, $2, $3, $4, $5, $6) returning id, ticker, symbol, label, precision, atype",
        )
        .bind(nanoid::nanoid!())
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

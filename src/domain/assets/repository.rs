use axum::async_trait;

use crate::core::database::{
    IntoRepositoryResult, ReadonlyDatabaseRepository, RepositoryResult, WriteDatabaseRepository,
};

use super::data::{Asset, AssetType};

#[async_trait]
pub trait AssetReadonlyRepository {
    async fn list_assets(&mut self) -> RepositoryResult<Vec<Asset>>;

    async fn list_assets_by_ids_or_tickers(
        &mut self,
        ids_or_tickers: &[String],
    ) -> RepositoryResult<Vec<Asset>>;

    async fn find_asset(&mut self, id_or_ticker: &str) -> RepositoryResult<Option<Asset>>;
}

#[async_trait]
pub trait AssetWriteRepository {
    async fn create_asset(
        &mut self,
        ticker: &str,
        symbol: Option<&str>,
        label: &str,
        precision: i16,
        atype: AssetType,
    ) -> RepositoryResult<Asset>;
}

#[async_trait]
impl<T> AssetReadonlyRepository for T
where
    T: ReadonlyDatabaseRepository + std::fmt::Debug + Send,
{
    #[tracing::instrument(err)]
    async fn list_assets(&mut self) -> RepositoryResult<Vec<Asset>> {
        sqlx::query_as::<_, Asset>(
            "select id, ticker, symbol, label, precision, atype from assets order by label",
        )
        .fetch_all(self.acquire().await?)
        .await
        .into_repository_result()
    }

    #[tracing::instrument(err)]
    async fn list_assets_by_ids_or_tickers(
        &mut self,
        ids_or_tickers: &[String],
    ) -> RepositoryResult<Vec<Asset>> {
        if ids_or_tickers.is_empty() {
            return Ok(vec![]);
        }

        sqlx::query_as::<_, Asset>(
            "select id, ticker, symbol, label, precision, atype from assets where id=ANY($1) or ticker=ANY($1) order by label",
        )
        .bind(ids_or_tickers)
        .fetch_all(self.acquire().await?)
        .await
        .into_repository_result()
    }

    #[tracing::instrument(err)]
    async fn find_asset(&mut self, id_or_ticker: &str) -> RepositoryResult<Option<Asset>> {
        sqlx::query_as::<_, Asset>(
            "select id, ticker, symbol, label, precision, atype from assets where id=$1 or ticker=$1",
        )
        .bind(id_or_ticker)
        .fetch_optional(self.acquire().await?)
        .await
        .into_repository_result()
    }
}

#[async_trait]
impl<T> AssetWriteRepository for T
where
    T: WriteDatabaseRepository + std::fmt::Debug + Send,
{
    #[tracing::instrument(err)]
    async fn create_asset(
        &mut self,
        ticker: &str,
        symbol: Option<&str>,
        label: &str,
        precision: i16,
        atype: AssetType,
    ) -> RepositoryResult<Asset> {
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
        .into_repository_result()
    }
}

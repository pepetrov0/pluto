use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "asset_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    Currency,
}

#[derive(Debug, Clone, FromRow)]
pub struct Asset {
    pub id: String,
    pub ticker: String,
    pub symbol: Option<String>,
    pub label: String,
    pub precision: i16,
    pub atype: AssetType,
}

use axum::async_trait;

use crate::domain::{
    database::{
        sessions::{Session, Sessions},
        Result,
    },
    identifier::Id,
};

use super::AnyTransaction;

#[async_trait]
impl Sessions for AnyTransaction {
    #[tracing::instrument(skip(self))]
    async fn find_session_by_id(&mut self, id: Id) -> Result<Option<Session>> {
        match self {
            AnyTransaction::Sqlite(v) => v.find_session_by_id(id).await,
            AnyTransaction::Pg(v) => v.find_session_by_id(id).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn find_all_sessions_by_user_id(&mut self, user_id: Id) -> Result<Vec<Session>> {
        match self {
            AnyTransaction::Sqlite(v) => v.find_all_sessions_by_user_id(user_id).await,
            AnyTransaction::Pg(v) => v.find_all_sessions_by_user_id(user_id).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn create_session(&mut self, user_id: Id, agent: &str) -> Result<Session> {
        match self {
            AnyTransaction::Sqlite(v) => v.create_session(user_id, agent).await,
            AnyTransaction::Pg(v) => v.create_session(user_id, agent).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn delete_session_by_id(&mut self, id: Id) -> Result<()> {
        match self {
            AnyTransaction::Sqlite(v) => v.delete_session_by_id(id).await,
            AnyTransaction::Pg(v) => v.delete_session_by_id(id).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn delete_all_sessions_by_user_id(&mut self, user_id: Id) -> Result<()> {
        match self {
            AnyTransaction::Sqlite(v) => v.delete_all_sessions_by_user_id(user_id).await,
            AnyTransaction::Pg(v) => v.delete_all_sessions_by_user_id(user_id).await,
        }
    }
}

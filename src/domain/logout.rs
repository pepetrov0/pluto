//! Implements the logic around a user logout.

use tracing::instrument;

use super::{database::AnyTransaction, sessions::SessionsRepository, Session};

/// Logs out a session.
#[instrument(skip(tx))]
pub async fn logout(tx: &mut AnyTransaction, session: &Session) {
    let _ = tx.delete_session_by_id(session.id).await;
}

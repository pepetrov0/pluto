//! Implements the logic around a user logout.

use super::{database::AnyTransaction, sessions::SessionsRepository, Session};

/// Logs out a session.
pub async fn logout(tx: &mut AnyTransaction, session: &Session) {
    let _ = tx.delete_session_by_id(session.id).await;
}

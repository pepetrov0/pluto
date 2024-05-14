use crate::domain::database::{sessions::Sessions, users::Users, Database};

use super::AnyDatabase;

#[tokio::test]
async fn sqlite() {
    let sqlite = AnyDatabase::connect("sqlite::memory:")
        .await
        .expect("connect to in-memory sqlite");

    run_suite(sqlite).await;
}

#[tokio::test]
async fn postgres() {
    let postgres = AnyDatabase::connect("postgresql://local:local@localhost:5432/local")
        .await
        .expect("connect to postgres");

    run_suite(postgres).await;
}

async fn run_suite(database: AnyDatabase) {
    test_users(&database).await;
    test_sessions(&database).await;
}

async fn test_users(database: &AnyDatabase) {
    let mut transaction = database.begin().await.expect("begin transaction");

    // Phase 1: try finding non-existent user

    assert!(transaction
        .find_user_by_id(56.into())
        .await
        .expect("should not error")
        .is_none());

    // Phase 2: try creating user and finding them

    let user = transaction
        .create_user("test@local.host", None)
        .await
        .expect("should not error");
    let by_id = transaction
        .find_user_by_id(user.id)
        .await
        .expect("should not error")
        .expect("should find the user by id");
    let by_email = transaction
        .find_user_by_email(user.email.as_str())
        .await
        .expect("should not error")
        .expect("should find the user by email");

    assert_eq!(user, by_id);
    assert_eq!(user, by_email);
}

async fn test_sessions(database: &AnyDatabase) {
    let mut transaction = database.begin().await.expect("begin transaction");

    // Phase 1: create users
    let user_1 = transaction
        .create_user("user-1@local.host", None)
        .await
        .expect("should not error");
    let user_2 = transaction
        .create_user("user-2@local.host", None)
        .await
        .expect("should not error");

    // Phase 2: try finding non existent session

    assert!(transaction
        .find_session_by_id(56.into())
        .await
        .expect("should not error")
        .is_none());

    // Phase 3: try finding sessions for user - should return no sessions

    assert!(transaction
        .find_all_sessions_by_user_id(user_1.id)
        .await
        .expect("should not error")
        .is_empty());

    // Phase 4: try creating a session and find it

    let session = transaction
        .create_session(user_1.id, "<agent>")
        .await
        .expect("should not error");

    let by_id = transaction
        .find_session_by_id(session.id)
        .await
        .expect("should not error")
        .expect("should find the session by id");
    let by_user_id = transaction
        .find_all_sessions_by_user_id(user_1.id)
        .await
        .expect("should not error");

    assert_eq!(by_user_id.len(), 1);
    assert_eq!(by_id, session);
    assert_eq!(by_user_id[0], session);

    // Phase 5: try deleting a session and checking for it's existence

    let session = transaction
        .create_session(user_1.id, "<agent>")
        .await
        .expect("should not error");
    transaction
        .delete_session_by_id(session.id)
        .await
        .expect("should not error");

    assert!(transaction
        .find_session_by_id(session.id)
        .await
        .expect("should not error")
        .is_none());

    // Phase 6: try deleting sessions by user ID and check for their existence

    for _ in 0..5 {
        transaction
            .create_session(user_2.id, "<agent>")
            .await
            .expect("should not error");
    }

    transaction
        .delete_all_sessions_by_user_id(user_2.id)
        .await
        .expect("should not error");

    assert!(transaction
        .find_all_sessions_by_user_id(user_2.id)
        .await
        .expect("should not error")
        .is_empty());
}

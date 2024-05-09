use crate::domain::database::{sqlite::SqliteDatabase, users::Users, Database};

#[tokio::test]
async fn find_non_existent_user() {
    let database = SqliteDatabase::connect("sqlite::memory:")
        .await
        .expect("connect to in-memory sqlite");

    let mut transaction = database.begin().await.expect("begin transaction");

    assert!(transaction
        .find_user_by_id(56.into())
        .await
        .expect("should not error")
        .is_none());
}

#[tokio::test]
async fn create_a_user() {
    let database = SqliteDatabase::connect("sqlite::memory:")
        .await
        .expect("connect to in-memory sqlite");

    let mut transaction = database.begin().await.expect("begin transaction");

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

use std::path::PathBuf;

use sqlx::{Connection, SqliteConnection, sqlite::SqliteConnectOptions};

pub mod engine;

pub async fn get_handle(db_path: &PathBuf) -> SqliteConnection {
    let db_options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    let mut db = SqliteConnection::connect_with(&db_options).await.unwrap();

    // let _ = sqlx::query!(
    //     "CREATE TABLE IF NOT EXISTS rounds (
    //             round INTEGER,
    //             team TEXT,
    //             service TEXT,
    //             success INTEGER,
    //             message TEXT,
    //             timestamp REAL
    //         )
    //     "
    // )
    // .execute(&mut db)
    // .await;

    db
}

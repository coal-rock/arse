use std::path::PathBuf;

use axum::routing::connect;
use sqlx::{
    Pool, Sqlite, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

pub mod engine;
pub mod users;

pub async fn get_pool(db_path: &PathBuf) -> SqlitePool {
    let db_options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(db_options)
        .await
        .unwrap()
}

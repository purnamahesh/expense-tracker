use std::{error::Error, path::PathBuf};

use sqlx::{Pool, Sqlite, SqlitePool, sqlite::SqliteConnectOptions};

use crate::path::get_project_db_file_path;

pub async fn initialize_db(
    custom_file_path: Option<PathBuf>,
    create_if_missing: bool,
) -> Result<Pool<Sqlite>, Box<dyn Error>> {
    let filename = custom_file_path.unwrap_or(get_project_db_file_path()?.into());

    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(create_if_missing);

    let conn = SqlitePool::connect_with(options).await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS expense (
                id TEXT PRIMARY KEY,
                amount REAL,
                category TEXT,
                datetime INTEGER,
                tags TEXT,
                description TEXT 
            ); "#,
    )
    .execute(&conn)
    .await?
    .rows_affected();

    Ok(conn)
}

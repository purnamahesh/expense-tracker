use std::{error::Error, path::Path, process::exit};

use sqlx::{Pool, Sqlite, SqlitePool, sqlite::SqliteConnectOptions};

use directories::ProjectDirs;

pub async fn initialize_db() -> Result<Pool<Sqlite>, Box<dyn Error>> {
    if let Some(p) = ProjectDirs::from("", "", "expense-tracker") {
        // self.config_dir = Some(p.config_dir().to_path_buf());
        let data_dir = p.data_dir().to_path_buf();

        let path = match data_dir.to_str() {
            Some(path_str) => path_str,
            None => {
                eprintln!("utf-8: Failed to get data dir");
                exit(1);
            }
        };
        // let conn_str = format!("sqlite://{}/{}", path, "data.db");
        let filename = format!("{}/{}", path, "data.db");

        let options = SqliteConnectOptions::new()
            .filename(filename)
            .create_if_missing(true);

        let conn = SqlitePool::connect_with(options).await?;

        sqlx::query(r#"CREATE TABLE IF NOT EXISTS expense (
                    id TEXT PRIMARY KEY,
                    amount REAL,
                    category TEXT,
                    datetime INTEGER,
                    tags TEXT,
                    description TEXT 
                ); "#).execute(&conn)
            .await?
            .rows_affected();

        Ok(conn)
    } else {
        return Err("Non utf-8 character in home path".into());
    }
}

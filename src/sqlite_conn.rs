use std::{error::Error, path::PathBuf, process::exit};

use sqlx::{migrate::MigrateDatabase, sqlite::Sqlite};

pub async fn create_db_if_not_exists(data_dir: Option<&PathBuf>) -> Result<(), Box<dyn Error>> {
    let path = match data_dir {
        Some(pb) => if let Some(path_str) = pb.to_str() {
            path_str
        } else {
            eprintln!("Failed to get data dir");
            exit(1);
        },
        None => {
            eprintln!("Failed to get data dir");
            exit(1);
        }
    };

    let conn_str = format!("sqlite://{}/{}", path, "data.db");

    if ! Sqlite::database_exists(&conn_str).await? {
        Sqlite::create_database(&conn_str).await?;
    }

    // let conn =SqlitePool::connect("sqlite:///Users/mmv6113/RustProjects/lite/data.db").await?;
    
    Ok(())
}
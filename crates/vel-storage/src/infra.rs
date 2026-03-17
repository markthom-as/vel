use sqlx::sqlite::SqliteConnectOptions;
use std::str::FromStr;

use crate::db::StorageError;

pub(crate) fn sqlite_connect_options(db_path: &str) -> Result<SqliteConnectOptions, StorageError> {
    let url = if db_path == ":memory:" {
        "sqlite::memory:".to_string()
    } else if db_path.starts_with("sqlite:") {
        db_path.to_string()
    } else {
        format!("sqlite://{db_path}")
    };

    let options = SqliteConnectOptions::from_str(&url)?.create_if_missing(true);

    Ok(options)
}

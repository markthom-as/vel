use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, SqlitePool};
use std::{fs, path::Path, str::FromStr};
use time::OffsetDateTime;
use uuid::Uuid;

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

pub(crate) async fn connect_pool(db_path: &str) -> Result<SqlitePool, StorageError> {
    if db_path != ":memory:" {
        if let Some(parent) = Path::new(db_path).parent() {
            fs::create_dir_all(parent)?;
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(sqlite_connect_options(db_path)?)
        .await?;

    Ok(pool)
}

pub(crate) async fn run_migrations(
    pool: &SqlitePool,
    migrator: &Migrator,
) -> Result<(), StorageError> {
    migrator.run(pool).await?;
    Ok(())
}

pub(crate) async fn healthcheck(pool: &SqlitePool) -> Result<(), StorageError> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}

pub(crate) async fn schema_version(pool: &SqlitePool) -> Result<u32, StorageError> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM _sqlx_migrations")
        .fetch_one(pool)
        .await?;
    Ok(row.0 as u32)
}

pub(crate) async fn emit_event(
    pool: &SqlitePool,
    event_type: &str,
    subject_type: &str,
    subject_id: Option<&str>,
    payload_json: &str,
) -> Result<(), StorageError> {
    let event_id = format!("evt_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"
        INSERT INTO events (event_id, event_type, subject_type, subject_id, payload_json, created_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&event_id)
    .bind(event_type)
    .bind(subject_type)
    .bind(subject_id)
    .bind(payload_json)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}

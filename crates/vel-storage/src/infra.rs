use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::{
    migrate::{MigrateError, Migrator},
    sqlite::SqlitePoolOptions,
    Row, SqlitePool,
};
use std::collections::HashSet;
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
    let options = if db_path != ":memory:" {
        options.journal_mode(SqliteJournalMode::Wal)
    } else {
        options
    };
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
    match migrator.run(pool).await {
        Ok(()) => {}
        Err(MigrateError::ExecuteMigration(error, version))
            if error
                .to_string()
                .contains("duplicate column name: client_kind") =>
        {
            ensure_swarm_client_columns(pool).await?;
            mark_migration_applied(pool, migrator, version).await?;
            migrator.run(pool).await?;
        }
        Err(error) => return Err(error.into()),
    }
    Ok(())
}

async fn ensure_swarm_client_columns(pool: &SqlitePool) -> Result<(), StorageError> {
    let rows = sqlx::query("PRAGMA table_info(cluster_workers)")
        .fetch_all(pool)
        .await?;
    let existing: HashSet<String> = rows
        .into_iter()
        .map(|row| row.try_get::<String, _>("name"))
        .collect::<Result<_, _>>()?;

    for (column, sql_type) in [
        ("client_kind", "TEXT"),
        ("client_version", "TEXT"),
        ("protocol_version", "TEXT"),
        ("build_id", "TEXT"),
        ("ping_ms", "INTEGER"),
        ("sync_status", "TEXT"),
        ("last_upstream_sync_at", "INTEGER"),
        ("last_downstream_sync_at", "INTEGER"),
        ("last_sync_error", "TEXT"),
    ] {
        if existing.contains(column) {
            continue;
        }

        sqlx::query(&format!(
            "ALTER TABLE cluster_workers ADD COLUMN {column} {sql_type}"
        ))
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn mark_migration_applied(
    pool: &SqlitePool,
    migrator: &Migrator,
    version: i64,
) -> Result<(), StorageError> {
    let already_applied =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM _sqlx_migrations WHERE version = ?1")
            .bind(version)
            .fetch_one(pool)
            .await?;

    if already_applied > 0 {
        return Ok(());
    }

    let migration = migrator
        .migrations
        .iter()
        .find(|migration| migration.version == version)
        .ok_or_else(|| StorageError::Migration(MigrateError::VersionMissing(version)))?;

    sqlx::query(
        r#"
        INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time)
        VALUES (?1, ?2, TRUE, ?3, -1)
        "#,
    )
    .bind(migration.version)
    .bind(&*migration.description)
    .bind(&*migration.checksum)
    .execute(pool)
    .await?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn wal_mode_enabled_for_file_db() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let db_path = std::env::temp_dir()
            .join(format!("vel-wal-test-{nanos}.sqlite"))
            .to_string_lossy()
            .to_string();

        let pool = connect_pool(&db_path).await.expect("pool opens");
        let row: (String,) = sqlx::query_as("PRAGMA journal_mode")
            .fetch_one(&pool)
            .await
            .expect("pragma query");
        assert_eq!(row.0, "wal", "WAL mode should be active for file-based DB");

        // cleanup WAL sidecar files
        std::fs::remove_file(&db_path).ok();
        std::fs::remove_file(format!("{}-wal", db_path)).ok();
        std::fs::remove_file(format!("{}-shm", db_path)).ok();
    }

    #[tokio::test]
    async fn wal_mode_skipped_for_memory_db() {
        // In-memory databases cannot use WAL — must connect without error
        let pool = connect_pool(":memory:").await;
        assert!(
            pool.is_ok(),
            "in-memory connect must succeed (no WAL applied)"
        );
    }
}

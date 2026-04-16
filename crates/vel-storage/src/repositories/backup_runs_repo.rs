use serde_json::Value as JsonValue;
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;

use crate::{
    db::{BackupRunRecord, StorageError},
    mapping::timestamp_to_datetime,
};

const LOCAL_BACKUP_TARGET_ID: &str = "stgt_local_backup_runtime";
const LOCAL_BACKUP_TARGET_KIND: &str = "local_filesystem";
const LOCAL_BACKUP_TARGET_ROLE: &str = "backup_only";
const LOCAL_BACKUP_TARGET_LABEL: &str = "Local Backup Runtime";
const LOCAL_BACKUP_EXPORT_TARGET_ID: &str = "stgt_local_backup_export_runtime";
const LOCAL_BACKUP_EXPORT_TARGET_KIND: &str = "local_filesystem";
const LOCAL_BACKUP_EXPORT_TARGET_ROLE: &str = "knowledge_export";
const LOCAL_BACKUP_EXPORT_TARGET_LABEL: &str = "Local Backup Export Runtime";

pub(crate) async fn persist_backup_run(
    pool: &SqlitePool,
    backup_id: &str,
    output_root: &str,
    state: &str,
    manifest_json: &JsonValue,
    started_at: OffsetDateTime,
    completed_at: Option<OffsetDateTime>,
    verified_at: Option<OffsetDateTime>,
    last_error: Option<&str>,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    ensure_local_backup_target(&mut tx, output_root, started_at).await?;

    sqlx::query(
        r#"
        INSERT INTO backup_manifests (
            backup_manifest_id,
            storage_target_id,
            scope,
            state,
            started_at,
            completed_at,
            verified_at,
            summary_json,
            last_error
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(backup_manifest_id) DO UPDATE SET
            state = excluded.state,
            completed_at = excluded.completed_at,
            verified_at = excluded.verified_at,
            summary_json = excluded.summary_json,
            last_error = excluded.last_error
        "#,
    )
    .bind(backup_id)
    .bind(LOCAL_BACKUP_TARGET_ID)
    .bind("full")
    .bind(state)
    .bind(started_at.unix_timestamp())
    .bind(completed_at.map(|value| value.unix_timestamp()))
    .bind(verified_at.map(|value| value.unix_timestamp()))
    .bind(serde_json::to_string(manifest_json)?)
    .bind(last_error)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE storage_targets
        SET root_uri = ?,
            updated_at = ?,
            last_success_at = COALESCE(?, last_success_at),
            last_error = ?
        WHERE storage_target_id = ?
        "#,
    )
    .bind(output_root)
    .bind(started_at.unix_timestamp())
    .bind(completed_at.map(|value| value.unix_timestamp()))
    .bind(last_error)
    .bind(LOCAL_BACKUP_TARGET_ID)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

pub(crate) async fn persist_backup_export_run(
    pool: &SqlitePool,
    export_id: &str,
    target_root: &str,
    state: &str,
    manifest_json: &JsonValue,
    started_at: OffsetDateTime,
    completed_at: Option<OffsetDateTime>,
    verified_at: Option<OffsetDateTime>,
    last_error: Option<&str>,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    ensure_local_backup_export_target(&mut tx, target_root, started_at).await?;

    sqlx::query(
        r#"
        INSERT INTO backup_manifests (
            backup_manifest_id,
            storage_target_id,
            scope,
            state,
            started_at,
            completed_at,
            verified_at,
            summary_json,
            last_error
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(backup_manifest_id) DO UPDATE SET
            state = excluded.state,
            completed_at = excluded.completed_at,
            verified_at = excluded.verified_at,
            summary_json = excluded.summary_json,
            last_error = excluded.last_error
        "#,
    )
    .bind(export_id)
    .bind(LOCAL_BACKUP_EXPORT_TARGET_ID)
    .bind("knowledge_export")
    .bind(state)
    .bind(started_at.unix_timestamp())
    .bind(completed_at.map(|value| value.unix_timestamp()))
    .bind(verified_at.map(|value| value.unix_timestamp()))
    .bind(serde_json::to_string(manifest_json)?)
    .bind(last_error)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE storage_targets
        SET root_uri = ?,
            updated_at = ?,
            last_success_at = COALESCE(?, last_success_at),
            last_error = ?
        WHERE storage_target_id = ?
        "#,
    )
    .bind(target_root)
    .bind(started_at.unix_timestamp())
    .bind(completed_at.map(|value| value.unix_timestamp()))
    .bind(last_error)
    .bind(LOCAL_BACKUP_EXPORT_TARGET_ID)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

pub(crate) async fn get_backup_run(
    pool: &SqlitePool,
    backup_id: &str,
) -> Result<Option<BackupRunRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            backup_manifest_id,
            state,
            summary_json,
            started_at,
            completed_at,
            verified_at,
            last_error
        FROM backup_manifests
        WHERE backup_manifest_id = ?
        "#,
    )
    .bind(backup_id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_backup_run_row).transpose()
}

pub(crate) async fn get_backup_export_run(
    pool: &SqlitePool,
    export_id: &str,
) -> Result<Option<BackupRunRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            backup_manifest_id,
            state,
            summary_json,
            started_at,
            completed_at,
            verified_at,
            last_error
        FROM backup_manifests
        WHERE backup_manifest_id = ?
          AND storage_target_id = ?
        "#,
    )
    .bind(export_id)
    .bind(LOCAL_BACKUP_EXPORT_TARGET_ID)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_backup_run_row).transpose()
}

pub(crate) async fn list_backup_runs(
    pool: &SqlitePool,
    limit: u32,
) -> Result<Vec<BackupRunRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT
            backup_manifest_id,
            state,
            summary_json,
            started_at,
            completed_at,
            verified_at,
            last_error
        FROM backup_manifests
        WHERE storage_target_id = ?
        ORDER BY COALESCE(completed_at, started_at) DESC, started_at DESC, rowid DESC
        LIMIT ?
        "#,
    )
    .bind(LOCAL_BACKUP_TARGET_ID)
    .bind(limit as i64)
    .fetch_all(pool)
    .await?;

    rows.iter().map(map_backup_run_row).collect()
}

pub(crate) async fn list_backup_export_runs(
    pool: &SqlitePool,
    limit: u32,
) -> Result<Vec<BackupRunRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT
            backup_manifest_id,
            state,
            summary_json,
            started_at,
            completed_at,
            verified_at,
            last_error
        FROM backup_manifests
        WHERE storage_target_id = ?
        ORDER BY COALESCE(completed_at, started_at) DESC, started_at DESC, rowid DESC
        LIMIT ?
        "#,
    )
    .bind(LOCAL_BACKUP_EXPORT_TARGET_ID)
    .bind(limit as i64)
    .fetch_all(pool)
    .await?;

    rows.iter().map(map_backup_run_row).collect()
}

pub(crate) async fn get_last_successful_backup_run(
    pool: &SqlitePool,
) -> Result<Option<BackupRunRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            backup_manifest_id,
            state,
            summary_json,
            started_at,
            completed_at,
            verified_at,
            last_error
        FROM backup_manifests
        WHERE storage_target_id = ?
          AND state IN ('completed', 'verified')
        ORDER BY COALESCE(verified_at, completed_at, started_at) DESC, started_at DESC, rowid DESC
        LIMIT 1
        "#,
    )
    .bind(LOCAL_BACKUP_TARGET_ID)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_backup_run_row).transpose()
}

pub(crate) async fn get_last_successful_backup_export_run(
    pool: &SqlitePool,
) -> Result<Option<BackupRunRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            backup_manifest_id,
            state,
            summary_json,
            started_at,
            completed_at,
            verified_at,
            last_error
        FROM backup_manifests
        WHERE storage_target_id = ?
          AND state IN ('completed', 'verified')
        ORDER BY COALESCE(verified_at, completed_at, started_at) DESC, started_at DESC, rowid DESC
        LIMIT 1
        "#,
    )
    .bind(LOCAL_BACKUP_EXPORT_TARGET_ID)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_backup_run_row).transpose()
}

pub(crate) async fn create_sqlite_snapshot(
    pool: &SqlitePool,
    destination: &str,
) -> Result<(), StorageError> {
    let escaped = destination.replace('\'', "''");
    let statement = format!("VACUUM INTO '{escaped}'");
    sqlx::query(&statement).execute(pool).await?;
    Ok(())
}

async fn ensure_local_backup_target(
    tx: &mut Transaction<'_, Sqlite>,
    output_root: &str,
    now: OffsetDateTime,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO storage_targets (
            storage_target_id,
            kind,
            role,
            label,
            root_uri,
            metadata_json,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(storage_target_id) DO UPDATE SET
            root_uri = excluded.root_uri,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(LOCAL_BACKUP_TARGET_ID)
    .bind(LOCAL_BACKUP_TARGET_KIND)
    .bind(LOCAL_BACKUP_TARGET_ROLE)
    .bind(LOCAL_BACKUP_TARGET_LABEL)
    .bind(output_root)
    .bind(r#"{"managed_by":"phase9_backup_runtime"}"#)
    .bind(now.unix_timestamp())
    .bind(now.unix_timestamp())
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn ensure_local_backup_export_target(
    tx: &mut Transaction<'_, Sqlite>,
    target_root: &str,
    now: OffsetDateTime,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        INSERT INTO storage_targets (
            storage_target_id,
            kind,
            role,
            label,
            root_uri,
            metadata_json,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(storage_target_id) DO UPDATE SET
            root_uri = excluded.root_uri,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(LOCAL_BACKUP_EXPORT_TARGET_ID)
    .bind(LOCAL_BACKUP_EXPORT_TARGET_KIND)
    .bind(LOCAL_BACKUP_EXPORT_TARGET_ROLE)
    .bind(LOCAL_BACKUP_EXPORT_TARGET_LABEL)
    .bind(target_root)
    .bind(r#"{"managed_by":"phase9_backup_export_runtime"}"#)
    .bind(now.unix_timestamp())
    .bind(now.unix_timestamp())
    .execute(&mut **tx)
    .await?;

    Ok(())
}

fn map_backup_run_row(row: &sqlx::sqlite::SqliteRow) -> Result<BackupRunRecord, StorageError> {
    let manifest_json: JsonValue =
        serde_json::from_str(&row.try_get::<String, _>("summary_json")?)?;
    let output_root = manifest_json
        .get("output_root")
        .and_then(JsonValue::as_str)
        .or_else(|| manifest_json.get("target_root").and_then(JsonValue::as_str))
        .ok_or_else(|| {
            StorageError::DataCorrupted(
                "backup manifest missing output_root or target_root".to_string(),
            )
        })?;

    Ok(BackupRunRecord {
        backup_id: row.try_get("backup_manifest_id")?,
        output_root: output_root.to_string(),
        state: row.try_get("state")?,
        manifest_json,
        started_at: timestamp_to_datetime(row.try_get("started_at")?)?,
        completed_at: row
            .try_get::<Option<i64>, _>("completed_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        verified_at: row
            .try_get::<Option<i64>, _>("verified_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        last_error: row.try_get("last_error")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn backup_runs_persist_history_and_last_success() {
        let pool = test_pool().await;
        let first_started = OffsetDateTime::now_utc();
        let second_started = first_started + time::Duration::minutes(1);

        persist_backup_run(
            &pool,
            "bkp_first",
            "/tmp/backups/bkp_first",
            "completed",
            &serde_json::json!({
                "backup_id": "bkp_first",
                "output_root": "/tmp/backups/bkp_first"
            }),
            first_started,
            Some(first_started),
            None,
            None,
        )
        .await
        .unwrap();

        persist_backup_run(
            &pool,
            "bkp_second",
            "/tmp/backups/bkp_second",
            "verified",
            &serde_json::json!({
                "backup_id": "bkp_second",
                "output_root": "/tmp/backups/bkp_second"
            }),
            second_started,
            Some(second_started),
            Some(second_started),
            None,
        )
        .await
        .unwrap();

        let history = list_backup_runs(&pool, 10).await.unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].backup_id, "bkp_second");

        let last_success = get_last_successful_backup_run(&pool)
            .await
            .unwrap()
            .expect("last success");
        assert_eq!(last_success.backup_id, "bkp_second");

        let target_row = sqlx::query(
            "SELECT root_uri, last_success_at, last_error FROM storage_targets WHERE storage_target_id = ?",
        )
        .bind(LOCAL_BACKUP_TARGET_ID)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(
            target_row.try_get::<String, _>("root_uri").unwrap(),
            "/tmp/backups/bkp_second"
        );
        assert!(target_row
            .try_get::<Option<i64>, _>("last_success_at")
            .unwrap()
            .is_some());
        assert!(target_row
            .try_get::<Option<String>, _>("last_error")
            .unwrap()
            .is_none());
    }

    #[tokio::test]
    async fn backup_export_runs_use_separate_target_and_last_success() {
        let pool = test_pool().await;
        let started = OffsetDateTime::now_utc();

        persist_backup_run(
            &pool,
            "bkp_pack",
            "/tmp/backups/bkp_pack",
            "verified",
            &serde_json::json!({
                "backup_id": "bkp_pack",
                "output_root": "/tmp/backups/bkp_pack"
            }),
            started,
            Some(started),
            Some(started),
            None,
        )
        .await
        .unwrap();

        persist_backup_export_run(
            &pool,
            "bex_first",
            "/tmp/nas/google",
            "verified",
            &serde_json::json!({
                "export_id": "bex_first",
                "target_root": "/tmp/nas/google"
            }),
            started,
            Some(started),
            Some(started),
            None,
        )
        .await
        .unwrap();

        let export = get_backup_export_run(&pool, "bex_first")
            .await
            .unwrap()
            .expect("export run");
        assert_eq!(export.backup_id, "bex_first");
        assert_eq!(export.output_root, "/tmp/nas/google");

        let export_history = list_backup_export_runs(&pool, 10).await.unwrap();
        assert_eq!(export_history.len(), 1);
        assert_eq!(export_history[0].backup_id, "bex_first");

        let last_export = get_last_successful_backup_export_run(&pool)
            .await
            .unwrap()
            .expect("last export");
        assert_eq!(last_export.backup_id, "bex_first");

        let last_backup = get_last_successful_backup_run(&pool)
            .await
            .unwrap()
            .expect("last backup");
        assert_eq!(last_backup.backup_id, "bkp_pack");
    }
}

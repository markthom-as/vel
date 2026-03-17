use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    db::{StorageError, UncertaintyRecord, UncertaintyRecordInsert},
    mapping::parse_json_value,
};

pub(crate) async fn insert_uncertainty_record(
    pool: &SqlitePool,
    input: UncertaintyRecordInsert,
) -> Result<String, StorageError> {
    let mut tx = pool.begin().await?;
    let id = insert_uncertainty_record_in_tx(&mut tx, &input).await?;
    tx.commit().await?;
    Ok(id)
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn insert_uncertainty_record_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &UncertaintyRecordInsert,
) -> Result<String, StorageError> {
    let id = format!("unc_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let reasons_json = serde_json::to_string(&input.reasons_json)
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    let missing_evidence_json = input
        .missing_evidence_json
        .as_ref()
        .map(serde_json::to_string)
        .transpose()
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    sqlx::query(
        r#"
        INSERT INTO uncertainty_records (
            id,
            subject_type,
            subject_id,
            decision_kind,
            confidence_band,
            confidence_score,
            reasons_json,
            missing_evidence_json,
            resolution_mode,
            status,
            created_at,
            resolved_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'open', ?, NULL)
        "#,
    )
    .bind(&id)
    .bind(&input.subject_type)
    .bind(&input.subject_id)
    .bind(&input.decision_kind)
    .bind(&input.confidence_band)
    .bind(input.confidence_score)
    .bind(reasons_json)
    .bind(missing_evidence_json)
    .bind(&input.resolution_mode)
    .bind(now)
    .execute(&mut **tx)
    .await?;
    Ok(id)
}

pub(crate) async fn list_uncertainty_records(
    pool: &SqlitePool,
    status: Option<&str>,
    limit: u32,
) -> Result<Vec<UncertaintyRecord>, StorageError> {
    let limit = i64::from(limit.max(1));
    let rows = sqlx::query(
        r#"
        SELECT
            id,
            subject_type,
            subject_id,
            decision_kind,
            confidence_band,
            confidence_score,
            reasons_json,
            missing_evidence_json,
            resolution_mode,
            status,
            created_at,
            resolved_at
        FROM uncertainty_records
        WHERE (? IS NULL OR status = ?)
        ORDER BY created_at DESC
        LIMIT ?
        "#,
    )
    .bind(status)
    .bind(status)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| map_uncertainty_row(&row))
        .collect()
}

pub(crate) async fn get_uncertainty_record(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<UncertaintyRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            subject_type,
            subject_id,
            decision_kind,
            confidence_band,
            confidence_score,
            reasons_json,
            missing_evidence_json,
            resolution_mode,
            status,
            created_at,
            resolved_at
        FROM uncertainty_records
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    row.map(|row| map_uncertainty_row(&row)).transpose()
}

pub(crate) async fn find_open_uncertainty_record(
    pool: &SqlitePool,
    subject_type: &str,
    subject_id: Option<&str>,
    decision_kind: &str,
) -> Result<Option<UncertaintyRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            subject_type,
            subject_id,
            decision_kind,
            confidence_band,
            confidence_score,
            reasons_json,
            missing_evidence_json,
            resolution_mode,
            status,
            created_at,
            resolved_at
        FROM uncertainty_records
        WHERE subject_type = ?
          AND decision_kind = ?
          AND status = 'open'
          AND ((? IS NULL AND subject_id IS NULL) OR subject_id = ?)
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(subject_type)
    .bind(decision_kind)
    .bind(subject_id)
    .bind(subject_id)
    .fetch_optional(pool)
    .await?;
    row.map(|row| map_uncertainty_row(&row)).transpose()
}

pub(crate) async fn find_recent_uncertainty_record(
    pool: &SqlitePool,
    subject_type: &str,
    subject_id: Option<&str>,
    decision_kind: &str,
    status: &str,
    since_ts: i64,
) -> Result<Option<UncertaintyRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            subject_type,
            subject_id,
            decision_kind,
            confidence_band,
            confidence_score,
            reasons_json,
            missing_evidence_json,
            resolution_mode,
            status,
            created_at,
            resolved_at
        FROM uncertainty_records
        WHERE subject_type = ?
          AND decision_kind = ?
          AND status = ?
          AND ((? IS NULL AND subject_id IS NULL) OR subject_id = ?)
          AND COALESCE(resolved_at, created_at) >= ?
        ORDER BY COALESCE(resolved_at, created_at) DESC
        LIMIT 1
        "#,
    )
    .bind(subject_type)
    .bind(decision_kind)
    .bind(status)
    .bind(subject_id)
    .bind(subject_id)
    .bind(since_ts)
    .fetch_optional(pool)
    .await?;
    row.map(|row| map_uncertainty_row(&row)).transpose()
}

pub(crate) async fn resolve_uncertainty_record(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<UncertaintyRecord>, StorageError> {
    let mut tx = pool.begin().await?;
    let resolved = resolve_uncertainty_record_in_tx(&mut tx, id).await?;
    tx.commit().await?;
    Ok(resolved)
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn resolve_uncertainty_record_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
) -> Result<Option<UncertaintyRecord>, StorageError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let updated = sqlx::query(
        r#"
        UPDATE uncertainty_records
        SET status = 'resolved',
            resolved_at = COALESCE(resolved_at, ?)
        WHERE id = ?
        "#,
    )
    .bind(now)
    .bind(id)
    .execute(&mut **tx)
    .await?;
    if updated.rows_affected() == 0 {
        return Ok(None);
    }
    get_uncertainty_record_in_tx(tx, id).await
}

async fn get_uncertainty_record_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
) -> Result<Option<UncertaintyRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            subject_type,
            subject_id,
            decision_kind,
            confidence_band,
            confidence_score,
            reasons_json,
            missing_evidence_json,
            resolution_mode,
            status,
            created_at,
            resolved_at
        FROM uncertainty_records
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(&mut **tx)
    .await?;
    row.map(|row| map_uncertainty_row(&row)).transpose()
}

fn map_uncertainty_row(row: &sqlx::sqlite::SqliteRow) -> Result<UncertaintyRecord, StorageError> {
    let reasons_json = row.try_get::<String, _>("reasons_json")?;
    let missing_evidence_json = row.try_get::<Option<String>, _>("missing_evidence_json")?;
    Ok(UncertaintyRecord {
        id: row.try_get("id")?,
        subject_type: row.try_get("subject_type")?,
        subject_id: row.try_get("subject_id")?,
        decision_kind: row.try_get("decision_kind")?,
        confidence_band: row.try_get("confidence_band")?,
        confidence_score: row.try_get("confidence_score")?,
        reasons_json: parse_json_value(&reasons_json)?,
        missing_evidence_json: missing_evidence_json
            .as_deref()
            .map(parse_json_value)
            .transpose()?,
        resolution_mode: row.try_get("resolution_mode")?,
        status: row.try_get("status")?,
        created_at: row.try_get("created_at")?,
        resolved_at: row.try_get("resolved_at")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    fn sample_uncertainty_input(subject_id: Option<&str>) -> UncertaintyRecordInsert {
        UncertaintyRecordInsert {
            subject_type: "suggestion_candidate".to_string(),
            subject_id: subject_id.map(str::to_string),
            decision_kind: "suggestion_generation".to_string(),
            confidence_band: "low".to_string(),
            confidence_score: Some(0.42),
            reasons_json: json!({
                "summary": "Barely enough evidence for a commute-buffer change."
            }),
            missing_evidence_json: Some(json!({
                "current_count": 2,
                "threshold": 2,
                "more_events_needed": 1
            })),
            resolution_mode: "defer".to_string(),
        }
    }

    #[tokio::test]
    async fn insert_find_and_resolve_uncertainty_records_round_trip() {
        let pool = test_pool().await;

        let id = insert_uncertainty_record(
            &pool,
            sample_uncertainty_input(Some("increase_commute_buffer")),
        )
        .await
        .unwrap();

        let open = list_uncertainty_records(&pool, Some("open"), 10)
            .await
            .unwrap();
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].id, id);

        let current = get_uncertainty_record(&pool, &id).await.unwrap().unwrap();
        assert_eq!(current.status, "open");

        let found_open = find_open_uncertainty_record(
            &pool,
            "suggestion_candidate",
            Some("increase_commute_buffer"),
            "suggestion_generation",
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(found_open.id, id);

        let resolved = resolve_uncertainty_record(&pool, &id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(resolved.status, "resolved");
        let resolved_at = resolved.resolved_at.expect("resolved_at should be set");

        let found_recent_resolved = find_recent_uncertainty_record(
            &pool,
            "suggestion_candidate",
            Some("increase_commute_buffer"),
            "suggestion_generation",
            "resolved",
            resolved_at - 1,
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(found_recent_resolved.id, id);
    }

    #[tokio::test]
    async fn uncertainty_record_tx_helpers_roll_back_with_transaction() {
        let pool = test_pool().await;

        let id = {
            let mut tx = pool.begin().await.unwrap();
            let id = insert_uncertainty_record_in_tx(&mut tx, &sample_uncertainty_input(None))
                .await
                .unwrap();
            tx.commit().await.unwrap();
            id
        };

        {
            let mut tx = pool.begin().await.unwrap();
            let resolved = resolve_uncertainty_record_in_tx(&mut tx, &id)
                .await
                .unwrap()
                .unwrap();
            assert_eq!(resolved.status, "resolved");
            tx.rollback().await.unwrap();
        }

        let after = get_uncertainty_record(&pool, &id).await.unwrap().unwrap();
        assert_eq!(after.status, "open");
        assert!(after.resolved_at.is_none());
    }
}

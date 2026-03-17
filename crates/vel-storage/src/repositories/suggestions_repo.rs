use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    db::{
        StorageError, SuggestionEvidenceInsert, SuggestionEvidenceRecord, SuggestionInsertV2,
        SuggestionRecord,
    },
    mapping::parse_json_value,
};

pub(crate) async fn insert_suggestion_v2(
    pool: &SqlitePool,
    input: SuggestionInsertV2,
) -> Result<String, StorageError> {
    let mut tx = pool.begin().await?;
    let id = insert_suggestion_v2_in_tx(&mut tx, &input).await?;
    tx.commit().await?;
    Ok(id)
}

pub(crate) async fn insert_suggestion_v2_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &SuggestionInsertV2,
) -> Result<String, StorageError> {
    let id = format!("sug_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let payload_str = serde_json::to_string(&input.payload_json)
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    let decision_context_str = input
        .decision_context_json
        .as_ref()
        .map(|v| serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()));

    sqlx::query(
        r#"
        INSERT INTO suggestions (
            id,
            suggestion_type,
            state,
            title,
            summary,
            priority,
            confidence,
            dedupe_key,
            payload_json,
            decision_context_json,
            created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&input.suggestion_type)
    .bind(&input.state)
    .bind(&input.title)
    .bind(&input.summary)
    .bind(input.priority as i64)
    .bind(input.confidence)
    .bind(&input.dedupe_key)
    .bind(payload_str)
    .bind(decision_context_str)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(id)
}

pub(crate) async fn list_suggestions(
    pool: &SqlitePool,
    state_filter: Option<&str>,
    limit: u32,
) -> Result<Vec<SuggestionRecord>, StorageError> {
    let limit = limit.min(100) as i64;
    let rows = sqlx::query(
        r#"
        SELECT
            id,
            suggestion_type,
            state,
            title,
            summary,
            priority,
            confidence,
            dedupe_key,
            payload_json,
            decision_context_json,
            created_at,
            resolved_at,
            (SELECT COUNT(*) FROM suggestion_evidence WHERE suggestion_id = s.id) as evidence_count
        FROM suggestions s
        WHERE (? IS NULL OR state = ?)
        ORDER BY priority DESC, created_at DESC
        LIMIT ?
        "#,
    )
    .bind(state_filter)
    .bind(state_filter)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| map_suggestion_row(&row))
        .collect()
}

pub(crate) async fn get_suggestion_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<SuggestionRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            suggestion_type,
            state,
            title,
            summary,
            priority,
            confidence,
            dedupe_key,
            payload_json,
            decision_context_json,
            created_at,
            resolved_at,
            (SELECT COUNT(*) FROM suggestion_evidence WHERE suggestion_id = s.id) as evidence_count
        FROM suggestions s
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    row.map(|row| map_suggestion_row(&row)).transpose()
}

pub(crate) async fn find_recent_suggestion_by_dedupe_key(
    pool: &SqlitePool,
    dedupe_key: &str,
) -> Result<Option<SuggestionRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            suggestion_type,
            state,
            title,
            summary,
            priority,
            confidence,
            dedupe_key,
            payload_json,
            decision_context_json,
            created_at,
            resolved_at,
            (SELECT COUNT(*) FROM suggestion_evidence WHERE suggestion_id = s.id) as evidence_count
        FROM suggestions s
        WHERE dedupe_key = ?
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(dedupe_key)
    .fetch_optional(pool)
    .await?;

    row.map(|row| map_suggestion_row(&row)).transpose()
}

pub(crate) async fn update_suggestion_state(
    pool: &SqlitePool,
    id: &str,
    state: &str,
    resolved_at: Option<i64>,
    payload_json: Option<&str>,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"
        UPDATE suggestions
        SET state = ?,
            resolved_at = COALESCE(?, resolved_at),
            payload_json = COALESCE(?, payload_json)
        WHERE id = ?
        "#,
    )
    .bind(state)
    .bind(resolved_at)
    .bind(payload_json)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub(crate) async fn insert_suggestion_evidence(
    pool: &SqlitePool,
    input: SuggestionEvidenceInsert,
) -> Result<String, StorageError> {
    let mut tx = pool.begin().await?;
    let id = insert_suggestion_evidence_in_tx(&mut tx, &input).await?;
    tx.commit().await?;
    Ok(id)
}

pub(crate) async fn insert_suggestion_evidence_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &SuggestionEvidenceInsert,
) -> Result<String, StorageError> {
    let id = format!("sev_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let evidence_str = input
        .evidence_json
        .as_ref()
        .map(|v| serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()));

    sqlx::query(
        r#"
        INSERT INTO suggestion_evidence (
            id,
            suggestion_id,
            evidence_type,
            ref_id,
            evidence_json,
            weight,
            created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&input.suggestion_id)
    .bind(&input.evidence_type)
    .bind(&input.ref_id)
    .bind(evidence_str)
    .bind(input.weight)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(id)
}

pub(crate) async fn list_suggestion_evidence(
    pool: &SqlitePool,
    suggestion_id: &str,
) -> Result<Vec<SuggestionEvidenceRecord>, StorageError> {
    let rows = sqlx::query(
        r#"
        SELECT id, suggestion_id, evidence_type, ref_id, evidence_json, weight, created_at
        FROM suggestion_evidence
        WHERE suggestion_id = ?
        ORDER BY created_at ASC
        "#,
    )
    .bind(suggestion_id)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| map_suggestion_evidence_row(&row))
        .collect()
}

fn map_suggestion_row(row: &sqlx::sqlite::SqliteRow) -> Result<SuggestionRecord, StorageError> {
    let payload_json: String = row.try_get("payload_json")?;
    let decision_context_json: Option<String> = row.try_get("decision_context_json")?;
    let evidence_count: i64 = row.try_get("evidence_count")?;

    Ok(SuggestionRecord {
        id: row.try_get("id")?,
        suggestion_type: row.try_get("suggestion_type")?,
        state: row.try_get("state")?,
        title: row.try_get("title")?,
        summary: row.try_get("summary")?,
        priority: row.try_get::<i64, _>("priority")? as u32,
        confidence: row.try_get("confidence")?,
        dedupe_key: row.try_get("dedupe_key")?,
        payload_json: parse_json_value(&payload_json)?,
        decision_context_json: decision_context_json
            .map(|s| parse_json_value(&s))
            .transpose()?,
        created_at: row.try_get("created_at")?,
        resolved_at: row.try_get("resolved_at")?,
        evidence_count: evidence_count as u32,
    })
}

fn map_suggestion_evidence_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<SuggestionEvidenceRecord, StorageError> {
    let evidence_json: Option<String> = row.try_get("evidence_json")?;
    Ok(SuggestionEvidenceRecord {
        id: row.try_get("id")?,
        suggestion_id: row.try_get("suggestion_id")?,
        evidence_type: row.try_get("evidence_type")?,
        ref_id: row.try_get("ref_id")?,
        evidence_json: evidence_json.map(|s| parse_json_value(&s)).transpose()?,
        weight: row.try_get("weight")?,
        created_at: row.try_get("created_at")?,
    })
}

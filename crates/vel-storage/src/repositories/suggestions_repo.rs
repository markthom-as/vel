use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    db::{
        SuggestionEvidenceInsert, SuggestionEvidenceRecord, SuggestionInsertV2, StorageError,
        SuggestionRecord,
    },
    mapping::parse_json_value,
};

pub(crate) async fn find_recent_suggestion_by_dedupe_key(
    pool: &SqlitePool,
    dedupe_key: &str,
) -> Result<Option<SuggestionRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            s.id,
            s.suggestion_type,
            s.state,
            s.title,
            s.summary,
            s.priority,
            s.confidence,
            s.dedupe_key,
            s.payload_json,
            s.decision_context_json,
            s.created_at,
            s.resolved_at,
            CAST(COUNT(se.id) AS INTEGER) AS evidence_count
        FROM suggestions s
        LEFT JOIN suggestion_evidence se ON se.suggestion_id = s.id
        WHERE s.dedupe_key = ?
        GROUP BY
            s.id,
            s.suggestion_type,
            s.state,
            s.title,
            s.summary,
            s.priority,
            s.confidence,
            s.dedupe_key,
            s.payload_json,
            s.decision_context_json,
            s.created_at,
            s.resolved_at
        ORDER BY s.created_at DESC, s.rowid DESC
        LIMIT 1
        "#,
    )
    .bind(dedupe_key)
    .fetch_optional(pool)
    .await?;
    row.map(|row| map_suggestion_row(&row)).transpose()
}

pub(crate) async fn insert_suggestion_v2(
    pool: &SqlitePool,
    input: SuggestionInsertV2,
) -> Result<String, StorageError> {
    let mut tx = pool.begin().await?;
    let id = insert_suggestion_v2_in_tx(&mut tx, input).await?;
    tx.commit().await?;
    Ok(id)
}

pub(crate) async fn insert_suggestion_v2_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: SuggestionInsertV2,
) -> Result<String, StorageError> {
    let id = format!("sug_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let payload_json = serde_json::to_string(&input.payload_json)
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    let decision_context_json = input
        .decision_context_json
        .as_ref()
        .map(serde_json::to_string)
        .transpose()
        .map_err(|error| StorageError::Validation(error.to_string()))?;
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
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&input.suggestion_type)
    .bind(&input.state)
    .bind(&input.title)
    .bind(&input.summary)
    .bind(input.priority)
    .bind(&input.confidence)
    .bind(&input.dedupe_key)
    .bind(payload_json)
    .bind(decision_context_json)
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
            s.id,
            s.suggestion_type,
            s.state,
            s.title,
            s.summary,
            s.priority,
            s.confidence,
            s.dedupe_key,
            s.payload_json,
            s.decision_context_json,
            s.created_at,
            s.resolved_at,
            CAST(COUNT(se.id) AS INTEGER) AS evidence_count
        FROM suggestions s
        LEFT JOIN suggestion_evidence se ON se.suggestion_id = s.id
        WHERE (? IS NULL OR s.state = ?)
        GROUP BY
            s.id,
            s.suggestion_type,
            s.state,
            s.title,
            s.summary,
            s.priority,
            s.confidence,
            s.dedupe_key,
            s.payload_json,
            s.decision_context_json,
            s.created_at,
            s.resolved_at
        ORDER BY s.created_at DESC, s.rowid DESC
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
            s.id,
            s.suggestion_type,
            s.state,
            s.title,
            s.summary,
            s.priority,
            s.confidence,
            s.dedupe_key,
            s.payload_json,
            s.decision_context_json,
            s.created_at,
            s.resolved_at,
            CAST(COUNT(se.id) AS INTEGER) AS evidence_count
        FROM suggestions s
        LEFT JOIN suggestion_evidence se ON se.suggestion_id = s.id
        WHERE s.id = ?
        GROUP BY
            s.id,
            s.suggestion_type,
            s.state,
            s.title,
            s.summary,
            s.priority,
            s.confidence,
            s.dedupe_key,
            s.payload_json,
            s.decision_context_json,
            s.created_at,
            s.resolved_at
        "#,
    )
    .bind(id)
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
    let mut tx = pool.begin().await?;
    update_suggestion_state_in_tx(&mut tx, id, state, resolved_at, payload_json).await?;
    tx.commit().await?;
    Ok(())
}

pub(crate) async fn insert_suggestion_evidence(
    pool: &SqlitePool,
    input: SuggestionEvidenceInsert,
) -> Result<String, StorageError> {
    let id = format!("sugev_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let evidence_json = input
        .evidence_json
        .as_ref()
        .map(serde_json::to_string)
        .transpose()
        .map_err(|error| StorageError::Validation(error.to_string()))?;
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
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
    )
    .bind(&id)
    .bind(&input.suggestion_id)
    .bind(&input.evidence_type)
    .bind(&input.ref_id)
    .bind(evidence_json)
    .bind(input.weight)
    .bind(now)
    .execute(pool)
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
            ORDER BY created_at DESC
            "#,
    )
    .bind(suggestion_id)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| map_suggestion_evidence_row(&row))
        .collect()
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn update_suggestion_state_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
    state: &str,
    resolved_at: Option<i64>,
    payload_json: Option<&str>,
) -> Result<(), StorageError> {
    if let Some(payload) = payload_json {
        sqlx::query(r#"UPDATE suggestions SET state = ?, resolved_at = ?, payload_json = ? WHERE id = ?"#)
            .bind(state)
            .bind(resolved_at)
            .bind(payload)
            .bind(id)
            .execute(&mut **tx)
            .await?;
    } else {
        sqlx::query(r#"UPDATE suggestions SET state = ?, resolved_at = ? WHERE id = ?"#)
            .bind(state)
            .bind(resolved_at)
            .bind(id)
            .execute(&mut **tx)
            .await?;
    }
    Ok(())
}

pub(crate) fn map_suggestion_row(row: &sqlx::sqlite::SqliteRow) -> Result<SuggestionRecord, StorageError> {
    let payload_json = row.try_get::<String, _>("payload_json")?;
    let decision_context_json = row.try_get::<Option<String>, _>("decision_context_json")?;
    let evidence_count = row.try_get::<i64, _>("evidence_count")?;
    Ok(SuggestionRecord {
        id: row.try_get("id")?,
        suggestion_type: row.try_get("suggestion_type")?,
        state: row.try_get("state")?,
        title: row.try_get("title")?,
        summary: row.try_get("summary")?,
        priority: row.try_get("priority")?,
        confidence: row.try_get("confidence")?,
        dedupe_key: row.try_get("dedupe_key")?,
        payload_json: parse_json_value(&payload_json)?,
        decision_context_json: decision_context_json
            .as_deref()
            .map(parse_json_value)
            .transpose()?,
        created_at: row.try_get("created_at")?,
        resolved_at: row.try_get("resolved_at")?,
        evidence_count: evidence_count.max(0) as u32,
    })
}

fn map_suggestion_evidence_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<SuggestionEvidenceRecord, StorageError> {
    let evidence_json = row.try_get::<Option<String>, _>("evidence_json")?;
    Ok(SuggestionEvidenceRecord {
        id: row.try_get("id")?,
        suggestion_id: row.try_get("suggestion_id")?,
        evidence_type: row.try_get("evidence_type")?,
        ref_id: row.try_get("ref_id")?,
        evidence_json: evidence_json.as_deref().map(parse_json_value).transpose()?,
        weight: row.try_get("weight")?,
        created_at: row.try_get("created_at")?,
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

    async fn seed_suggestion(pool: &SqlitePool, suggestion_id: &str) {
        sqlx::query(
            r#"
            INSERT INTO suggestions (
                id,
                suggestion_type,
                state,
                payload_json,
                created_at,
                priority
            )
            VALUES (?, 'alignment', 'open', '{}', ?, 50)
            "#,
        )
        .bind(suggestion_id)
        .bind(OffsetDateTime::now_utc().unix_timestamp())
        .execute(pool)
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn insert_and_list_suggestion_evidence_round_trip() {
        let pool = test_pool().await;
        let suggestion_id = "sug_repo_evidence";
        seed_suggestion(&pool, suggestion_id).await;

        let evidence_id = insert_suggestion_evidence(
            &pool,
            SuggestionEvidenceInsert {
                suggestion_id: suggestion_id.to_string(),
                evidence_type: "nudge".to_string(),
                ref_id: "ref-1".to_string(),
                evidence_json: Some(json!({"source": "repo"})),
                weight: Some(0.75),
            },
        )
        .await
        .unwrap();
        let evidence = list_suggestion_evidence(&pool, suggestion_id)
            .await
            .unwrap();

        assert_eq!(evidence.len(), 1);
        assert_eq!(evidence[0].id, evidence_id);
        assert_eq!(evidence[0].evidence_type, "nudge");
        assert_eq!(evidence[0].ref_id, "ref-1");
        assert_eq!(evidence[0].weight, Some(0.75));
        assert_eq!(evidence[0].evidence_json, Some(json!({"source": "repo"})));
    }
}

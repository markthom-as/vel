use sqlx::{Row, Sqlite, SqlitePool, Transaction};

use crate::{
    db::{StorageError, SuggestionRecord},
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

fn map_suggestion_row(row: &sqlx::sqlite::SqliteRow) -> Result<SuggestionRecord, StorageError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    async fn seed_suggestion(
        pool: &SqlitePool,
        id: &str,
        dedupe_key: &str,
        created_at: i64,
        state: &str,
        payload_json: &str,
    ) {
        sqlx::query(
            r#"
            INSERT INTO suggestions (
                id,
                suggestion_type,
                state,
                payload_json,
                created_at,
                priority,
                dedupe_key
            )
            VALUES (?, 'repo_test', ?, ?, ?, 50, ?)
            "#,
        )
        .bind(id)
        .bind(state)
        .bind(payload_json)
        .bind(created_at)
        .bind(dedupe_key)
        .execute(pool)
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn find_recent_suggestion_by_dedupe_key_prefers_latest_row() {
        let pool = test_pool().await;
        seed_suggestion(
            &pool,
            "sug_repo_1",
            "increase_prep_window",
            100,
            "rejected",
            r#"{"version":"first"}"#,
        )
        .await;
        seed_suggestion(
            &pool,
            "sug_repo_2",
            "increase_prep_window",
            200,
            "pending",
            r#"{"version":"second"}"#,
        )
        .await;
        sqlx::query(
            r#"
            INSERT INTO suggestion_evidence (
                id, suggestion_id, evidence_type, ref_id, evidence_json, weight, created_at
            )
            VALUES ('sugev_repo_1', 'sug_repo_2', 'signal', 'sig_1', '{"score":1}', NULL, 200)
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        let found = find_recent_suggestion_by_dedupe_key(&pool, "increase_prep_window")
            .await
            .unwrap()
            .expect("suggestion should exist");
        assert_eq!(found.id, "sug_repo_2");
        assert_eq!(found.state, "pending");
        assert_eq!(found.payload_json["version"], "second");
        assert_eq!(found.evidence_count, 1);
    }

    #[tokio::test]
    async fn update_suggestion_state_preserves_payload_when_none() {
        let pool = test_pool().await;
        seed_suggestion(
            &pool,
            "sug_repo_update",
            "keep_payload",
            100,
            "pending",
            r#"{"version":"before"}"#,
        )
        .await;

        update_suggestion_state(&pool, "sug_repo_update", "accepted", Some(123), None)
            .await
            .unwrap();

        let row = sqlx::query_as::<_, (String, Option<i64>, String)>(
            "SELECT state, resolved_at, payload_json FROM suggestions WHERE id = ?",
        )
        .bind("sug_repo_update")
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(row.0, "accepted");
        assert_eq!(row.1, Some(123));
        assert_eq!(row.2, r#"{"version":"before"}"#);
    }

    #[tokio::test]
    async fn update_suggestion_state_in_tx_rolls_back_with_transaction() {
        let pool = test_pool().await;
        seed_suggestion(
            &pool,
            "sug_repo_tx",
            "tx_key",
            100,
            "pending",
            r#"{"version":"before"}"#,
        )
        .await;

        {
            let mut tx = pool.begin().await.unwrap();
            update_suggestion_state_in_tx(
                &mut tx,
                "sug_repo_tx",
                "resolved",
                Some(555),
                Some(r#"{"version":"after"}"#),
            )
            .await
            .unwrap();
            tx.rollback().await.unwrap();
        }

        let row = sqlx::query_as::<_, (String, Option<i64>, String)>(
            "SELECT state, resolved_at, payload_json FROM suggestions WHERE id = ?",
        )
        .bind("sug_repo_tx")
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(row.0, "pending");
        assert_eq!(row.1, None);
        assert_eq!(row.2, r#"{"version":"before"}"#);
    }
}

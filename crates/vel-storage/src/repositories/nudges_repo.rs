use serde_json::{json, Value as JsonValue};
use sqlx::{Row, Sqlite, SqlitePool, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::{NudgeEventRecord, NudgeInsert, NudgeRecord, StorageError};

pub(crate) async fn insert_nudge(
    pool: &SqlitePool,
    input: NudgeInsert,
) -> Result<String, StorageError> {
    let mut tx = pool.begin().await?;
    let nudge_id = insert_nudge_in_tx(&mut tx, &input).await?;
    tx.commit().await?;
    Ok(nudge_id)
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn insert_nudge_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    input: &NudgeInsert,
) -> Result<String, StorageError> {
    let nudge_id = format!("nud_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let metadata_json = serde_json::to_string(input.metadata_json.as_ref().unwrap_or(&json!({})))
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    sqlx::query(
        r#"
        INSERT INTO nudges (
            nudge_id,
            nudge_type,
            level,
            state,
            related_commitment_id,
            message,
            created_at,
            snoozed_until,
            resolved_at,
            signals_snapshot_json,
            inference_snapshot_json,
            metadata_json
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&nudge_id)
    .bind(&input.nudge_type)
    .bind(&input.level)
    .bind(&input.state)
    .bind(&input.related_commitment_id)
    .bind(&input.message)
    .bind(now)
    .bind(input.snoozed_until)
    .bind(input.resolved_at)
    .bind(&input.signals_snapshot_json)
    .bind(&input.inference_snapshot_json)
    .bind(&metadata_json)
    .execute(&mut **tx)
    .await?;
    Ok(nudge_id)
}

pub(crate) async fn get_nudge(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<NudgeRecord>, StorageError> {
    let row = sqlx::query(
        r#"SELECT nudge_id, nudge_type, level, state, related_commitment_id, message, created_at, snoozed_until, resolved_at, signals_snapshot_json, inference_snapshot_json, metadata_json FROM nudges WHERE nudge_id = ?"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    row.map(|row| map_nudge_row(&row)).transpose()
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn get_nudge_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    id: &str,
) -> Result<Option<NudgeRecord>, StorageError> {
    let row = sqlx::query(
        r#"SELECT nudge_id, nudge_type, level, state, related_commitment_id, message, created_at, snoozed_until, resolved_at, signals_snapshot_json, inference_snapshot_json, metadata_json FROM nudges WHERE nudge_id = ?"#,
    )
    .bind(id)
    .fetch_optional(&mut **tx)
    .await?;
    row.map(|row| map_nudge_row(&row)).transpose()
}

pub(crate) async fn list_nudges(
    pool: &SqlitePool,
    state_filter: Option<&str>,
    limit: u32,
) -> Result<Vec<NudgeRecord>, StorageError> {
    let limit = limit.min(100) as i64;
    let rows = sqlx::query(
        r#"
        SELECT nudge_id, nudge_type, level, state, related_commitment_id, message, created_at, snoozed_until, resolved_at, signals_snapshot_json, inference_snapshot_json, metadata_json
        FROM nudges
        WHERE (? IS NULL OR state = ?)
        ORDER BY created_at DESC
        LIMIT ?
        "#,
    )
    .bind(state_filter)
    .bind(state_filter)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(|row| map_nudge_row(&row)).collect()
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn list_nudges_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    state_filter: Option<&str>,
    limit: u32,
) -> Result<Vec<NudgeRecord>, StorageError> {
    let limit = limit.min(100) as i64;
    let rows = sqlx::query(
        r#"
        SELECT nudge_id, nudge_type, level, state, related_commitment_id, message, created_at, snoozed_until, resolved_at, signals_snapshot_json, inference_snapshot_json, metadata_json
        FROM nudges
        WHERE (? IS NULL OR state = ?)
        ORDER BY created_at DESC
        LIMIT ?
        "#,
    )
    .bind(state_filter)
    .bind(state_filter)
    .bind(limit)
    .fetch_all(&mut **tx)
    .await?;
    rows.into_iter().map(|row| map_nudge_row(&row)).collect()
}

pub(crate) async fn update_nudge_state(
    pool: &SqlitePool,
    nudge_id: &str,
    state: &str,
    snoozed_until: Option<i64>,
    resolved_at: Option<i64>,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    update_nudge_state_in_tx(&mut tx, nudge_id, state, snoozed_until, resolved_at).await?;
    tx.commit().await?;
    Ok(())
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn update_nudge_state_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    nudge_id: &str,
    state: &str,
    snoozed_until: Option<i64>,
    resolved_at: Option<i64>,
) -> Result<(), StorageError> {
    sqlx::query(
        r#"UPDATE nudges SET state = ?, snoozed_until = ?, resolved_at = ? WHERE nudge_id = ?"#,
    )
    .bind(state)
    .bind(snoozed_until)
    .bind(resolved_at)
    .bind(nudge_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub(crate) async fn update_nudge_lifecycle(
    pool: &SqlitePool,
    nudge_id: &str,
    level: &str,
    state: &str,
    message: &str,
    snoozed_until: Option<i64>,
    resolved_at: Option<i64>,
    inference_snapshot_json: Option<&str>,
    metadata_json: &JsonValue,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    update_nudge_lifecycle_in_tx(
        &mut tx,
        nudge_id,
        level,
        state,
        message,
        snoozed_until,
        resolved_at,
        inference_snapshot_json,
        metadata_json,
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn update_nudge_lifecycle_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    nudge_id: &str,
    level: &str,
    state: &str,
    message: &str,
    snoozed_until: Option<i64>,
    resolved_at: Option<i64>,
    inference_snapshot_json: Option<&str>,
    metadata_json: &JsonValue,
) -> Result<(), StorageError> {
    let metadata_json = serde_json::to_string(metadata_json)
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    sqlx::query(
        r#"
        UPDATE nudges
        SET level = ?, state = ?, message = ?, snoozed_until = ?, resolved_at = ?, inference_snapshot_json = ?, metadata_json = ?
        WHERE nudge_id = ?
        "#,
    )
    .bind(level)
    .bind(state)
    .bind(message)
    .bind(snoozed_until)
    .bind(resolved_at)
    .bind(inference_snapshot_json)
    .bind(metadata_json)
    .bind(nudge_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub(crate) async fn count_nudge_events(pool: &SqlitePool) -> Result<i64, StorageError> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM nudge_events")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn count_nudge_events_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
) -> Result<i64, StorageError> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM nudge_events")
        .fetch_one(&mut **tx)
        .await?;
    Ok(row.0)
}

pub(crate) async fn insert_nudge_event(
    pool: &SqlitePool,
    nudge_id: &str,
    event_type: &str,
    payload_json: &str,
    timestamp: i64,
) -> Result<(), StorageError> {
    let mut tx = pool.begin().await?;
    insert_nudge_event_in_tx(&mut tx, nudge_id, event_type, payload_json, timestamp).await?;
    tx.commit().await?;
    Ok(())
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn insert_nudge_event_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    nudge_id: &str,
    event_type: &str,
    payload_json: &str,
    timestamp: i64,
) -> Result<(), StorageError> {
    let id = format!("nve_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    sqlx::query(
        r#"INSERT INTO nudge_events (id, nudge_id, event_type, payload_json, timestamp, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&id)
    .bind(nudge_id)
    .bind(event_type)
    .bind(payload_json)
    .bind(timestamp)
    .bind(now)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub(crate) async fn list_nudge_events(
    pool: &SqlitePool,
    nudge_id: &str,
    limit: u32,
) -> Result<Vec<NudgeEventRecord>, StorageError> {
    let limit = limit.min(100) as i64;
    let rows = sqlx::query(
        r#"
        SELECT id, nudge_id, event_type, payload_json, timestamp, created_at
        FROM nudge_events
        WHERE nudge_id = ?
        ORDER BY rowid ASC
        LIMIT ?
        "#,
    )
    .bind(nudge_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| map_nudge_event_row(&row))
        .collect()
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn list_nudge_events_in_tx(
    tx: &mut Transaction<'_, Sqlite>,
    nudge_id: &str,
    limit: u32,
) -> Result<Vec<NudgeEventRecord>, StorageError> {
    let limit = limit.min(100) as i64;
    let rows = sqlx::query(
        r#"
        SELECT id, nudge_id, event_type, payload_json, timestamp, created_at
        FROM nudge_events
        WHERE nudge_id = ?
        ORDER BY rowid ASC
        LIMIT ?
        "#,
    )
    .bind(nudge_id)
    .bind(limit)
    .fetch_all(&mut **tx)
    .await?;
    rows.into_iter()
        .map(|row| map_nudge_event_row(&row))
        .collect()
}

fn map_nudge_row(row: &sqlx::sqlite::SqliteRow) -> Result<NudgeRecord, StorageError> {
    let metadata_json: String = row.try_get("metadata_json")?;
    Ok(NudgeRecord {
        nudge_id: row.try_get("nudge_id")?,
        nudge_type: row.try_get("nudge_type")?,
        level: row.try_get("level")?,
        state: row.try_get("state")?,
        related_commitment_id: row.try_get("related_commitment_id")?,
        message: row.try_get("message")?,
        created_at: row.try_get("created_at")?,
        snoozed_until: row.try_get("snoozed_until")?,
        resolved_at: row.try_get("resolved_at")?,
        signals_snapshot_json: row.try_get("signals_snapshot_json")?,
        inference_snapshot_json: row.try_get("inference_snapshot_json")?,
        metadata_json: serde_json::from_str(&metadata_json).unwrap_or_else(|_| json!({})),
    })
}

fn map_nudge_event_row(row: &sqlx::sqlite::SqliteRow) -> Result<NudgeEventRecord, StorageError> {
    let payload_json = row.try_get::<String, _>("payload_json")?;
    Ok(NudgeEventRecord {
        id: row.try_get("id")?,
        nudge_id: row.try_get("nudge_id")?,
        event_type: row.try_get("event_type")?,
        payload_json: serde_json::from_str(&payload_json)
            .unwrap_or(JsonValue::Object(Default::default())),
        timestamp: row.try_get("timestamp")?,
        created_at: row.try_get("created_at")?,
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

    fn sample_nudge(state: &str) -> NudgeInsert {
        NudgeInsert {
            nudge_type: "focus".to_string(),
            level: "warning".to_string(),
            state: state.to_string(),
            related_commitment_id: Some("com_1".to_string()),
            message: "Take a short planning pass".to_string(),
            snoozed_until: None,
            resolved_at: None,
            signals_snapshot_json: Some(r#"{"signals":1}"#.to_string()),
            inference_snapshot_json: Some(r#"{"reason":"drift"}"#.to_string()),
            metadata_json: Some(json!({"source":"repo-test"})),
        }
    }

    #[tokio::test]
    async fn nudge_crud_round_trip() {
        let pool = test_pool().await;
        let nudge_id = insert_nudge(&pool, sample_nudge("open")).await.unwrap();

        let fetched = get_nudge(&pool, &nudge_id).await.unwrap().unwrap();
        assert_eq!(fetched.nudge_id, nudge_id);
        assert_eq!(fetched.state, "open");
        assert_eq!(fetched.level, "warning");
        assert_eq!(fetched.metadata_json["source"], "repo-test");

        let listed = list_nudges(&pool, Some("open"), 10).await.unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].nudge_id, nudge_id);

        update_nudge_state(&pool, &nudge_id, "snoozed", Some(1_700_000_300), None)
            .await
            .unwrap();
        let snoozed = get_nudge(&pool, &nudge_id).await.unwrap().unwrap();
        assert_eq!(snoozed.state, "snoozed");
        assert_eq!(snoozed.snoozed_until, Some(1_700_000_300));

        update_nudge_lifecycle(
            &pool,
            &nudge_id,
            "danger",
            "resolved",
            "Handled",
            None,
            Some(1_700_000_400),
            Some(r#"{"reason":"completed"}"#),
            &json!({"source":"repo-test","final":"yes"}),
        )
        .await
        .unwrap();
        let resolved = get_nudge(&pool, &nudge_id).await.unwrap().unwrap();
        assert_eq!(resolved.level, "danger");
        assert_eq!(resolved.state, "resolved");
        assert_eq!(resolved.message, "Handled");
        assert_eq!(resolved.resolved_at, Some(1_700_000_400));
        assert_eq!(
            resolved.inference_snapshot_json.as_deref(),
            Some(r#"{"reason":"completed"}"#)
        );
        assert_eq!(resolved.metadata_json["final"], "yes");
    }

    #[tokio::test]
    async fn nudge_events_round_trip_and_count() {
        let pool = test_pool().await;
        let nudge_id = insert_nudge(&pool, sample_nudge("open")).await.unwrap();

        insert_nudge_event(
            &pool,
            &nudge_id,
            "nudge_created",
            r#"{"channel":"local"}"#,
            1_700_000_100,
        )
        .await
        .unwrap();
        insert_nudge_event(
            &pool,
            &nudge_id,
            "nudge_snoozed",
            r#"{"minutes":30}"#,
            1_700_000_200,
        )
        .await
        .unwrap();

        let events = list_nudge_events(&pool, &nudge_id, 10).await.unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, "nudge_created");
        assert_eq!(events[1].event_type, "nudge_snoozed");
        assert_eq!(events[1].payload_json["minutes"], 30);

        let count = count_nudge_events(&pool).await.unwrap();
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn nudge_tx_helpers_roll_back() {
        let pool = test_pool().await;

        let inserted_id = {
            let mut tx = pool.begin().await.unwrap();
            let nudge_id = insert_nudge_in_tx(&mut tx, &sample_nudge("open"))
                .await
                .unwrap();

            let fetched = get_nudge_in_tx(&mut tx, &nudge_id).await.unwrap().unwrap();
            assert_eq!(fetched.state, "open");

            let listed = list_nudges_in_tx(&mut tx, Some("open"), 10).await.unwrap();
            assert_eq!(listed.len(), 1);

            update_nudge_state_in_tx(&mut tx, &nudge_id, "snoozed", Some(1_700_001_000), None)
                .await
                .unwrap();
            update_nudge_lifecycle_in_tx(
                &mut tx,
                &nudge_id,
                "danger",
                "resolved",
                "rolled-back",
                None,
                Some(1_700_001_200),
                None,
                &json!({"rollback":true}),
            )
            .await
            .unwrap();
            insert_nudge_event_in_tx(
                &mut tx,
                &nudge_id,
                "nudge_resolved",
                r#"{"outcome":"temporary"}"#,
                1_700_001_300,
            )
            .await
            .unwrap();

            let in_tx_count = count_nudge_events_in_tx(&mut tx).await.unwrap();
            assert_eq!(in_tx_count, 1);
            let in_tx_events = list_nudge_events_in_tx(&mut tx, &nudge_id, 10)
                .await
                .unwrap();
            assert_eq!(in_tx_events.len(), 1);

            tx.rollback().await.unwrap();
            nudge_id
        };

        let persisted = get_nudge(&pool, &inserted_id).await.unwrap();
        assert!(persisted.is_none());
        let count = count_nudge_events(&pool).await.unwrap();
        assert_eq!(count, 0);
    }
}

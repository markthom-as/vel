use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::{DailyCheckInEventInsert, DailyCheckInEventRecord, StorageError};
use crate::mapping::parse_json_value;

pub(crate) async fn insert_daily_check_in_event(
    pool: &SqlitePool,
    input: DailyCheckInEventInsert,
) -> Result<String, StorageError> {
    let event_id = format!("dci_{}", Uuid::new_v4().simple());
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let keywords_json = serde_json::to_string(&input.keywords_json)
        .map_err(|error| StorageError::Validation(error.to_string()))?;
    let meta_json = serde_json::to_string(&input.meta_json)
        .map_err(|error| StorageError::Validation(error.to_string()))?;

    sqlx::query(
        r#"
        INSERT INTO daily_check_in_events (
            check_in_event_id,
            session_id,
            prompt_id,
            check_in_type,
            session_phase,
            source,
            answered_at,
            text,
            scale,
            scale_min,
            scale_max,
            keywords_json,
            confidence,
            schema_version,
            skipped,
            skip_reason_code,
            skip_reason_text,
            replaced_by_event_id,
            meta_json,
            created_at,
            updated_at,
            run_id
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&event_id)
    .bind(&input.session_id)
    .bind(&input.prompt_id)
    .bind(&input.check_in_type)
    .bind(&input.session_phase)
    .bind(&input.source)
    .bind(input.answered_at)
    .bind(&input.text)
    .bind(input.scale)
    .bind(input.scale_min)
    .bind(input.scale_max)
    .bind(&keywords_json)
    .bind(input.confidence)
    .bind(input.schema_version)
    .bind(i64::from(input.skipped))
    .bind(&input.skip_reason_code)
    .bind(&input.skip_reason_text)
    .bind(&input.replaced_by_event_id)
    .bind(&meta_json)
    .bind(now)
    .bind(now)
    .bind(&input.run_id)
    .execute(pool)
    .await?;

    Ok(event_id)
}

pub(crate) async fn get_daily_check_in_event(
    pool: &SqlitePool,
    event_id: &str,
) -> Result<Option<DailyCheckInEventRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            check_in_event_id,
            session_id,
            prompt_id,
            check_in_type,
            session_phase,
            source,
            answered_at,
            text,
            scale,
            scale_min,
            scale_max,
            keywords_json,
            confidence,
            schema_version,
            skipped,
            skip_reason_code,
            skip_reason_text,
            replaced_by_event_id,
            meta_json,
            created_at,
            updated_at,
            run_id
        FROM daily_check_in_events
        WHERE check_in_event_id = ?
        "#,
    )
    .bind(event_id)
    .fetch_optional(pool)
    .await?;

    row.map(map_daily_check_in_event_row).transpose()
}

pub(crate) async fn list_daily_check_in_events_for_session(
    pool: &SqlitePool,
    session_id: &str,
    check_in_type: Option<&str>,
    session_phase: Option<&str>,
    include_skipped: bool,
    limit: u32,
) -> Result<Vec<DailyCheckInEventRecord>, StorageError> {
    let limit = limit.min(500) as i64;
    let include_skipped = i64::from(include_skipped);
    let rows = sqlx::query(
        r#"
        SELECT
            check_in_event_id,
            session_id,
            prompt_id,
            check_in_type,
            session_phase,
            source,
            answered_at,
            text,
            scale,
            scale_min,
            scale_max,
            keywords_json,
            confidence,
            schema_version,
            skipped,
            skip_reason_code,
            skip_reason_text,
            replaced_by_event_id,
            meta_json,
            created_at,
            updated_at,
            run_id
        FROM daily_check_in_events
        WHERE session_id = ?
          AND (? IS NULL OR check_in_type = ?)
          AND (? IS NULL OR session_phase = ?)
          AND (? = 1 OR skipped = 0)
        ORDER BY created_at DESC, rowid DESC
        LIMIT ?
        "#,
    )
    .bind(session_id)
    .bind(check_in_type)
    .bind(check_in_type)
    .bind(session_phase)
    .bind(session_phase)
    .bind(include_skipped)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(map_daily_check_in_event_row).collect()
}

fn map_daily_check_in_event_row(
    row: sqlx::sqlite::SqliteRow,
) -> Result<DailyCheckInEventRecord, StorageError> {
    let keywords_json: String = row.try_get("keywords_json")?;
    let meta_json: String = row.try_get("meta_json")?;
    let skipped: i64 = row.try_get("skipped")?;
    Ok(DailyCheckInEventRecord {
        event_id: row.try_get("check_in_event_id")?,
        session_id: row.try_get("session_id")?,
        prompt_id: row.try_get("prompt_id")?,
        check_in_type: row.try_get("check_in_type")?,
        session_phase: row.try_get("session_phase")?,
        source: row.try_get("source")?,
        answered_at: row.try_get("answered_at")?,
        text: row.try_get("text")?,
        scale: row.try_get("scale")?,
        scale_min: row.try_get("scale_min")?,
        scale_max: row.try_get("scale_max")?,
        keywords_json: parse_json_value(&keywords_json)?,
        confidence: row.try_get("confidence")?,
        schema_version: row.try_get("schema_version")?,
        skipped: skipped != 0,
        skip_reason_code: row.try_get("skip_reason_code")?,
        skip_reason_text: row.try_get("skip_reason_text")?,
        replaced_by_event_id: row.try_get("replaced_by_event_id")?,
        meta_json: parse_json_value(&meta_json)?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        run_id: row.try_get("run_id")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::StorageError;

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn daily_check_in_events_round_trip() {
        let pool = test_pool().await;
        let event_id = insert_daily_check_in_event(
            &pool,
            DailyCheckInEventInsert {
                session_id: "dls_1".to_string(),
                prompt_id: "prompt_1".to_string(),
                check_in_type: "other".to_string(),
                session_phase: "morning".to_string(),
                source: "user".to_string(),
                answered_at: Some(1_700_000_100),
                text: Some("Focus on one call block".to_string()),
                scale: None,
                scale_min: -10,
                scale_max: 10,
                keywords_json: JsonValue::Array(vec![]),
                confidence: Some(0.93),
                schema_version: 1,
                skipped: false,
                skip_reason_code: None,
                skip_reason_text: None,
                replaced_by_event_id: None,
                run_id: None,
                meta_json: serde_json::json!({"source": "integration"}),
            },
        )
        .await
        .unwrap();

        let row = get_daily_check_in_event(&pool, &event_id)
            .await
            .unwrap()
            .expect("row");
        assert_eq!(row.event_id, event_id);
        assert_eq!(row.session_phase, "morning");
        assert_eq!(row.check_in_type, "other");

        let listed = list_daily_check_in_events_for_session(&pool, "dls_1", None, None, false, 10)
            .await
            .unwrap();
        assert_eq!(listed.len(), 1);
    }

    #[tokio::test]
    async fn check_in_round_trip_ignores_phase_filter() {
        let pool = test_pool().await;
        insert_daily_check_in_event(
            &pool,
            DailyCheckInEventInsert {
                session_id: "dls_2".to_string(),
                prompt_id: "prompt_a".to_string(),
                check_in_type: "mood".to_string(),
                session_phase: "standup".to_string(),
                source: "user".to_string(),
                answered_at: None,
                text: None,
                scale: Some(-4),
                scale_min: -10,
                scale_max: 10,
                keywords_json: JsonValue::Array(vec![]),
                confidence: None,
                schema_version: 1,
                skipped: true,
                skip_reason_code: Some("not_now".to_string()),
                skip_reason_text: Some("later".to_string()),
                replaced_by_event_id: None,
                run_id: None,
                meta_json: serde_json::json!({"source": "integration"}),
            },
        )
        .await
        .unwrap();

        let filtered = list_daily_check_in_events_for_session(
            &pool,
            "dls_2",
            Some("mood"),
            None,
            true,
            10,
        )
            .await
            .unwrap();
        assert_eq!(filtered.len(), 1);
        let filtered_empty = list_daily_check_in_events_for_session(
            &pool,
            "dls_2",
            Some("pain"),
            None,
            true,
            10,
        )
        .await
        .unwrap();
        assert_eq!(filtered_empty.len(), 0);
    }

    #[tokio::test]
    async fn check_in_list_includes_skips_only_when_requested() {
        let pool = test_pool().await;
        insert_daily_check_in_event(
            &pool,
            DailyCheckInEventInsert {
                session_id: "dls_3".to_string(),
                prompt_id: "prompt_a".to_string(),
                check_in_type: "other".to_string(),
                session_phase: "morning".to_string(),
                source: "user".to_string(),
                answered_at: None,
                text: None,
                scale: None,
                scale_min: -10,
                scale_max: 10,
                keywords_json: JsonValue::Array(vec![]),
                confidence: None,
                schema_version: 1,
                skipped: true,
                skip_reason_code: Some("not_now".to_string()),
                skip_reason_text: Some("later".to_string()),
                replaced_by_event_id: None,
                run_id: None,
                meta_json: serde_json::json!({"source": "integration"}),
            },
        )
        .await
        .unwrap();

        let filtered = list_daily_check_in_events_for_session(
            &pool,
            "dls_3",
            None,
            None,
            false,
            10,
        )
        .await
        .unwrap();
        assert_eq!(filtered.len(), 0);

        let filtered = list_daily_check_in_events_for_session(
            &pool,
            "dls_3",
            None,
            None,
            true,
            10,
        )
        .await
        .unwrap();
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].skipped);
    }

    #[tokio::test]
    async fn check_in_list_filters_by_phase() {
        let pool = test_pool().await;
        insert_daily_check_in_event(
            &pool,
            DailyCheckInEventInsert {
                session_id: "dls_4".to_string(),
                prompt_id: "prompt_a".to_string(),
                check_in_type: "other".to_string(),
                session_phase: "morning".to_string(),
                source: "user".to_string(),
                answered_at: None,
                text: None,
                scale: None,
                scale_min: -10,
                scale_max: 10,
                keywords_json: JsonValue::Array(vec![]),
                confidence: None,
                schema_version: 1,
                skipped: false,
                skip_reason_code: None,
                skip_reason_text: None,
                replaced_by_event_id: None,
                run_id: None,
                meta_json: serde_json::json!({"source": "integration"}),
            },
        )
        .await
        .unwrap();
        insert_daily_check_in_event(
            &pool,
            DailyCheckInEventInsert {
                session_id: "dls_4".to_string(),
                prompt_id: "prompt_b".to_string(),
                check_in_type: "other".to_string(),
                session_phase: "standup".to_string(),
                source: "user".to_string(),
                answered_at: None,
                text: None,
                scale: None,
                scale_min: -10,
                scale_max: 10,
                keywords_json: JsonValue::Array(vec![]),
                confidence: None,
                schema_version: 1,
                skipped: false,
                skip_reason_code: None,
                skip_reason_text: None,
                replaced_by_event_id: None,
                run_id: None,
                meta_json: serde_json::json!({"source": "integration"}),
            },
        )
        .await
        .unwrap();

        let filtered = list_daily_check_in_events_for_session(
            &pool,
            "dls_4",
            None,
            Some("morning"),
            true,
            10,
        )
        .await
        .unwrap();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].session_phase, "morning");
    }

}

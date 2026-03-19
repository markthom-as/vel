use sqlx::{Row, SqlitePool};
use time::OffsetDateTime;
use vel_core::{
    DailyLoopPhase, DailyLoopPrompt, DailyLoopSession, DailyLoopSessionId, DailyLoopSessionOutcome,
    DailyLoopSessionState, DailyLoopStartMetadata, DailyLoopStatus, DailyLoopTurnState,
};

use crate::{
    db::{DailySessionRecord, StorageError},
    mapping::timestamp_to_datetime,
};

pub(crate) async fn create_daily_session(
    pool: &SqlitePool,
    session: &DailyLoopSession,
    now: OffsetDateTime,
) -> Result<DailySessionRecord, StorageError> {
    sqlx::query(
        r#"
        INSERT INTO daily_sessions (
            session_id,
            session_date,
            phase,
            status,
            start_json,
            turn_state,
            current_prompt_json,
            state_json,
            outcome_json,
            created_at,
            updated_at,
            completed_at,
            cancelled_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(session.id.to_string())
    .bind(&session.session_date)
    .bind(serde_json::to_string(&session.phase)?)
    .bind(serde_json::to_string(&session.status)?)
    .bind(serde_json::to_string(&session.start)?)
    .bind(serde_json::to_string(&session.turn_state)?)
    .bind(
        session
            .current_prompt
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?,
    )
    .bind(serde_json::to_string(&session.state)?)
    .bind(
        session
            .outcome
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?,
    )
    .bind(now.unix_timestamp())
    .bind(now.unix_timestamp())
    .bind(completed_at(session.status, now))
    .bind(cancelled_at(session.status, now))
    .execute(pool)
    .await?;

    get_daily_session(pool, session.id.as_ref())
        .await?
        .ok_or_else(|| {
            StorageError::NotFound(format!("daily session {} missing after insert", session.id))
        })
}

pub(crate) async fn get_daily_session(
    pool: &SqlitePool,
    session_id: &str,
) -> Result<Option<DailySessionRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            session_id,
            session_date,
            phase,
            status,
            start_json,
            turn_state,
            current_prompt_json,
            state_json,
            outcome_json,
            created_at,
            updated_at,
            completed_at,
            cancelled_at
        FROM daily_sessions
        WHERE session_id = ?
        "#,
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_daily_session_row).transpose()
}

pub(crate) async fn get_active_daily_session_for_date(
    pool: &SqlitePool,
    session_date: &str,
    phase: DailyLoopPhase,
) -> Result<Option<DailySessionRecord>, StorageError> {
    let row = sqlx::query(
        r#"
        SELECT
            session_id,
            session_date,
            phase,
            status,
            start_json,
            turn_state,
            current_prompt_json,
            state_json,
            outcome_json,
            created_at,
            updated_at,
            completed_at,
            cancelled_at
        FROM daily_sessions
        WHERE session_date = ?
          AND phase = ?
          AND status IN ('active', 'waiting_for_input')
        ORDER BY updated_at DESC
        LIMIT 1
        "#,
    )
    .bind(session_date)
    .bind(serde_json::to_string(&phase)?)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_daily_session_row).transpose()
}

pub(crate) async fn update_daily_session_state(
    pool: &SqlitePool,
    session_id: &str,
    status: DailyLoopStatus,
    turn_state: DailyLoopTurnState,
    current_prompt: Option<&DailyLoopPrompt>,
    state: &DailyLoopSessionState,
    outcome: Option<&DailyLoopSessionOutcome>,
    now: OffsetDateTime,
) -> Result<Option<DailySessionRecord>, StorageError> {
    sqlx::query(
        r#"
        UPDATE daily_sessions
        SET status = ?,
            turn_state = ?,
            current_prompt_json = ?,
            state_json = ?,
            outcome_json = ?,
            updated_at = ?,
            completed_at = CASE
                WHEN ? = 'completed' THEN COALESCE(completed_at, ?)
                ELSE NULL
            END,
            cancelled_at = CASE
                WHEN ? = 'cancelled' THEN COALESCE(cancelled_at, ?)
                ELSE NULL
            END
        WHERE session_id = ?
        "#,
    )
    .bind(serde_json::to_string(&status)?)
    .bind(serde_json::to_string(&turn_state)?)
    .bind(current_prompt.map(serde_json::to_string).transpose()?)
    .bind(serde_json::to_string(state)?)
    .bind(outcome.map(serde_json::to_string).transpose()?)
    .bind(now.unix_timestamp())
    .bind(serde_json::to_string(&status)?)
    .bind(now.unix_timestamp())
    .bind(serde_json::to_string(&status)?)
    .bind(now.unix_timestamp())
    .bind(session_id)
    .execute(pool)
    .await?;

    get_daily_session(pool, session_id).await
}

pub(crate) async fn complete_daily_session(
    pool: &SqlitePool,
    session_id: &str,
    state: &DailyLoopSessionState,
    outcome: &DailyLoopSessionOutcome,
    now: OffsetDateTime,
) -> Result<Option<DailySessionRecord>, StorageError> {
    update_daily_session_state(
        pool,
        session_id,
        DailyLoopStatus::Completed,
        DailyLoopTurnState::Completed,
        None,
        state,
        Some(outcome),
        now,
    )
    .await
}

pub(crate) async fn cancel_daily_session(
    pool: &SqlitePool,
    session_id: &str,
    state: &DailyLoopSessionState,
    outcome: Option<&DailyLoopSessionOutcome>,
    now: OffsetDateTime,
) -> Result<Option<DailySessionRecord>, StorageError> {
    update_daily_session_state(
        pool,
        session_id,
        DailyLoopStatus::Cancelled,
        DailyLoopTurnState::Completed,
        None,
        state,
        outcome,
        now,
    )
    .await
}

fn completed_at(status: DailyLoopStatus, now: OffsetDateTime) -> Option<i64> {
    (status == DailyLoopStatus::Completed).then_some(now.unix_timestamp())
}

fn cancelled_at(status: DailyLoopStatus, now: OffsetDateTime) -> Option<i64> {
    (status == DailyLoopStatus::Cancelled).then_some(now.unix_timestamp())
}

fn map_daily_session_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<DailySessionRecord, StorageError> {
    Ok(DailySessionRecord {
        session: DailyLoopSession {
            id: DailyLoopSessionId::from(row.try_get::<String, _>("session_id")?),
            session_date: row.try_get("session_date")?,
            phase: serde_json::from_str::<DailyLoopPhase>(&row.try_get::<String, _>("phase")?)?,
            status: serde_json::from_str::<DailyLoopStatus>(&row.try_get::<String, _>("status")?)?,
            start: serde_json::from_str::<DailyLoopStartMetadata>(
                &row.try_get::<String, _>("start_json")?,
            )?,
            turn_state: serde_json::from_str::<DailyLoopTurnState>(
                &row.try_get::<String, _>("turn_state")?,
            )?,
            current_prompt: row
                .try_get::<Option<String>, _>("current_prompt_json")?
                .map(|value| serde_json::from_str::<DailyLoopPrompt>(&value))
                .transpose()?,
            state: serde_json::from_str::<DailyLoopSessionState>(
                &row.try_get::<String, _>("state_json")?,
            )?,
            outcome: row
                .try_get::<Option<String>, _>("outcome_json")?
                .map(|value| serde_json::from_str::<DailyLoopSessionOutcome>(&value))
                .transpose()?,
        },
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
        updated_at: timestamp_to_datetime(row.try_get("updated_at")?)?,
        completed_at: row
            .try_get::<Option<i64>, _>("completed_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        cancelled_at: row
            .try_get::<Option<i64>, _>("cancelled_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use time::macros::datetime;
    use vel_core::{
        DailyCommitmentDraft, DailyDeferredTask, DailyFocusBlockProposal, DailyLoopPromptKind,
        DailyLoopSessionOutcome, DailyLoopStartSource, DailyLoopSurface, DailyStandupBucket,
        DailyStandupOutcome, MorningFrictionCallout, MorningIntentSignal, MorningOverviewState,
    };

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        MIGRATOR.run(&pool).await.unwrap();
        pool
    }

    fn morning_session() -> DailyLoopSession {
        DailyLoopSession {
            id: DailyLoopSessionId::from("dls_test_morning".to_string()),
            session_date: "2026-03-19".to_string(),
            phase: DailyLoopPhase::MorningOverview,
            status: DailyLoopStatus::WaitingForInput,
            start: DailyLoopStartMetadata {
                source: DailyLoopStartSource::Manual,
                surface: DailyLoopSurface::Cli,
            },
            turn_state: DailyLoopTurnState::WaitingForInput,
            current_prompt: Some(DailyLoopPrompt {
                prompt_id: "prompt_1".to_string(),
                kind: DailyLoopPromptKind::IntentQuestion,
                text: "What most needs to happen before noon?".to_string(),
                ordinal: 1,
                allow_skip: true,
            }),
            state: DailyLoopSessionState::MorningOverview(MorningOverviewState {
                snapshot: "Two meetings before lunch.".to_string(),
                friction_callouts: vec![MorningFrictionCallout {
                    label: "Prep debt".to_string(),
                    detail: "Review notes are stale.".to_string(),
                }],
                signals: vec![MorningIntentSignal::FocusIntent {
                    text: "Protect writing time".to_string(),
                }],
            }),
            outcome: None,
        }
    }

    fn standup_outcome() -> DailyLoopSessionOutcome {
        DailyLoopSessionOutcome::Standup(DailyStandupOutcome {
            commitments: vec![DailyCommitmentDraft {
                title: "Ship Phase 10 storage seam".to_string(),
                bucket: DailyStandupBucket::Must,
                source_ref: Some("ticket:10-01".to_string()),
            }],
            deferred_tasks: vec![DailyDeferredTask {
                title: "Inbox cleanup".to_string(),
                source_ref: Some("todoist:1".to_string()),
                reason: "Outside the top three".to_string(),
            }],
            confirmed_calendar: vec!["10:00 review stays on".to_string()],
            focus_blocks: vec![DailyFocusBlockProposal {
                label: "Storage implementation".to_string(),
                start_at: datetime!(2026-03-19 15:00:00 UTC),
                end_at: datetime!(2026-03-19 16:00:00 UTC),
                reason: "Best uninterrupted slot".to_string(),
            }],
        })
    }

    #[tokio::test]
    async fn daily_sessions_round_trip_typed_payloads() {
        let pool = test_pool().await;
        let now = datetime!(2026-03-19 14:30:00 UTC);
        let session = morning_session();

        let created = create_daily_session(&pool, &session, now)
            .await
            .expect("daily session should persist");
        let loaded = get_daily_session(&pool, created.session.id.as_ref())
            .await
            .expect("daily session should reload")
            .expect("daily session should exist");

        assert_eq!(loaded.session, session);
        assert_eq!(loaded.created_at, now);
        assert_eq!(loaded.updated_at, now);
        assert!(loaded.completed_at.is_none());
        assert!(loaded.cancelled_at.is_none());
    }

    #[tokio::test]
    async fn daily_sessions_active_lookup_uses_date_and_phase() {
        let pool = test_pool().await;
        let now = datetime!(2026-03-19 14:30:00 UTC);
        let morning = morning_session();
        create_daily_session(&pool, &morning, now)
            .await
            .expect("morning session should persist");

        let mut completed = morning_session();
        completed.id = DailyLoopSessionId::from("dls_completed".to_string());
        completed.phase = DailyLoopPhase::Standup;
        completed.status = DailyLoopStatus::Completed;
        completed.turn_state = DailyLoopTurnState::Completed;
        completed.current_prompt = None;
        completed.state = DailyStandupOutcome {
            commitments: vec![],
            deferred_tasks: vec![],
            confirmed_calendar: vec![],
            focus_blocks: vec![],
        }
        .into();
        completed.outcome = Some(standup_outcome());
        create_daily_session(&pool, &completed, now + time::Duration::minutes(5))
            .await
            .expect("completed standup should persist");

        let active =
            get_active_daily_session_for_date(&pool, "2026-03-19", DailyLoopPhase::MorningOverview)
                .await
                .expect("lookup should succeed")
                .expect("active morning should be found");

        assert_eq!(active.session.id, morning.id);
        assert_eq!(active.session.phase, DailyLoopPhase::MorningOverview);
    }

    #[tokio::test]
    async fn daily_sessions_terminal_updates_write_status_and_timestamps() {
        let pool = test_pool().await;
        let created_at = datetime!(2026-03-19 14:30:00 UTC);
        let completed_at = datetime!(2026-03-19 15:00:00 UTC);
        let cancelled_at = datetime!(2026-03-19 15:15:00 UTC);
        let session = morning_session();

        create_daily_session(&pool, &session, created_at)
            .await
            .expect("morning session should persist");

        let completed = complete_daily_session(
            &pool,
            session.id.as_ref(),
            &session.state,
            &DailyLoopSessionOutcome::MorningOverview {
                signals: vec![MorningIntentSignal::MustDoHint {
                    text: "Finalize review notes".to_string(),
                }],
            },
            completed_at,
        )
        .await
        .expect("completion should succeed")
        .expect("completed session should reload");

        assert_eq!(completed.session.status, DailyLoopStatus::Completed);
        assert_eq!(completed.session.turn_state, DailyLoopTurnState::Completed);
        assert_eq!(completed.completed_at, Some(completed_at));
        assert!(completed.cancelled_at.is_none());

        let cancelled = cancel_daily_session(
            &pool,
            session.id.as_ref(),
            &session.state,
            completed.session.outcome.as_ref(),
            cancelled_at,
        )
        .await
        .expect("cancellation should succeed")
        .expect("cancelled session should reload");

        assert_eq!(cancelled.session.status, DailyLoopStatus::Cancelled);
        assert_eq!(cancelled.cancelled_at, Some(cancelled_at));
    }
}

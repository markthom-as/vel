use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{ApiResponse, LoopData, LoopUpdateRequest};

use crate::{errors::AppError, state::AppState};

const KNOWN_LOOP_KINDS: [vel_core::LoopKind; 15] = [
    vel_core::LoopKind::CaptureIngest,
    vel_core::LoopKind::RetryDueRuns,
    vel_core::LoopKind::QueueWorkScheduler,
    vel_core::LoopKind::EvaluateCurrentState,
    vel_core::LoopKind::SyncCalendar,
    vel_core::LoopKind::SyncTodoist,
    vel_core::LoopKind::SyncActivity,
    vel_core::LoopKind::SyncHealth,
    vel_core::LoopKind::SyncGit,
    vel_core::LoopKind::SyncMessaging,
    vel_core::LoopKind::SyncReminders,
    vel_core::LoopKind::SyncNotes,
    vel_core::LoopKind::SyncTranscripts,
    vel_core::LoopKind::WeeklySynthesis,
    vel_core::LoopKind::StaleNudgeReconciliation,
];

fn map_loop_data(record: vel_storage::RuntimeLoopRecord) -> LoopData {
    LoopData {
        kind: record.loop_kind,
        enabled: record.enabled,
        interval_seconds: record.interval_seconds,
        last_started_at: record.last_started_at,
        last_finished_at: record.last_finished_at,
        last_status: record.last_status,
        last_error: record.last_error,
        next_due_at: record.next_due_at,
    }
}

fn configured_loop_defaults(state: &AppState, loop_kind: vel_core::LoopKind) -> (bool, i64) {
    match loop_kind {
        vel_core::LoopKind::CaptureIngest => (true, 5),
        vel_core::LoopKind::RetryDueRuns => (true, 5),
        vel_core::LoopKind::QueueWorkScheduler => state
            .policy_config
            .queue_work_scheduler_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((true, 30)),
        vel_core::LoopKind::EvaluateCurrentState => state
            .policy_config
            .evaluate_current_state_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((true, 300)),
        vel_core::LoopKind::SyncCalendar => state
            .policy_config
            .sync_calendar_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((true, 900)),
        vel_core::LoopKind::SyncTodoist => state
            .policy_config
            .sync_todoist_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((true, 600)),
        vel_core::LoopKind::SyncActivity => state
            .policy_config
            .sync_activity_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((false, 300)),
        vel_core::LoopKind::SyncHealth => state
            .policy_config
            .sync_health_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((false, 900)),
        vel_core::LoopKind::SyncGit => state
            .policy_config
            .sync_git_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((false, 600)),
        vel_core::LoopKind::SyncMessaging => state
            .policy_config
            .sync_messaging_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((true, 300)),
        vel_core::LoopKind::SyncReminders => state
            .policy_config
            .sync_reminders_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((false, 600)),
        vel_core::LoopKind::SyncNotes => state
            .policy_config
            .sync_notes_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((false, 900)),
        vel_core::LoopKind::SyncTranscripts => state
            .policy_config
            .sync_transcripts_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((false, 900)),
        vel_core::LoopKind::WeeklySynthesis => state
            .policy_config
            .weekly_synthesis_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((true, 86_400)),
        vel_core::LoopKind::StaleNudgeReconciliation => state
            .policy_config
            .stale_nudge_reconciliation_loop()
            .map(|cfg| (cfg.enabled, cfg.interval_seconds as i64))
            .unwrap_or((true, 1_800)),
    }
}

async fn ensure_known_loop_rows(state: &AppState) -> Result<(), AppError> {
    for loop_kind in KNOWN_LOOP_KINDS {
        let (enabled, interval_seconds) = configured_loop_defaults(state, loop_kind);
        state
            .storage
            .ensure_runtime_loop(
                &loop_kind.to_string(),
                enabled,
                interval_seconds,
                Some(time::OffsetDateTime::now_utc().unix_timestamp() + interval_seconds),
            )
            .await?;
    }
    Ok(())
}

pub async fn list_loops(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<LoopData>>>, AppError> {
    ensure_known_loop_rows(&state).await?;
    let loops = state
        .storage
        .list_runtime_loops()
        .await?
        .into_iter()
        .map(map_loop_data)
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(loops, request_id)))
}

pub async fn get_loop(
    State(state): State<AppState>,
    Path(loop_kind): Path<String>,
) -> Result<Json<ApiResponse<LoopData>>, AppError> {
    let loop_kind = loop_kind.trim();
    let parsed_loop_kind = loop_kind
        .parse::<vel_core::LoopKind>()
        .map_err(|_| AppError::not_found("loop not found"))?;
    let (enabled, interval_seconds) = configured_loop_defaults(&state, parsed_loop_kind);
    state
        .storage
        .ensure_runtime_loop(
            loop_kind,
            enabled,
            interval_seconds,
            Some(time::OffsetDateTime::now_utc().unix_timestamp() + interval_seconds),
        )
        .await?;
    let record = state
        .storage
        .get_runtime_loop(loop_kind)
        .await?
        .ok_or_else(|| AppError::not_found("loop not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_loop_data(record),
        request_id,
    )))
}

pub async fn update_loop(
    State(state): State<AppState>,
    Path(loop_kind): Path<String>,
    Json(body): Json<LoopUpdateRequest>,
) -> Result<Json<ApiResponse<LoopData>>, AppError> {
    if let Some(interval_seconds) = body.interval_seconds {
        if interval_seconds <= 0 {
            return Err(AppError::bad_request("interval_seconds must be positive"));
        }
    }

    let loop_kind = loop_kind.trim();
    let parsed_loop_kind = loop_kind
        .parse::<vel_core::LoopKind>()
        .map_err(|_| AppError::not_found("loop not found"))?;
    let (enabled, interval_seconds) = configured_loop_defaults(&state, parsed_loop_kind);
    state
        .storage
        .ensure_runtime_loop(
            loop_kind,
            enabled,
            interval_seconds,
            Some(time::OffsetDateTime::now_utc().unix_timestamp() + interval_seconds),
        )
        .await?;
    let record = state
        .storage
        .update_runtime_loop_config(loop_kind, body.enabled, body.interval_seconds)
        .await
        .map_err(map_loop_storage_error)?
        .ok_or_else(|| AppError::not_found("loop not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_loop_data(record),
        request_id,
    )))
}

fn map_loop_storage_error(error: vel_storage::StorageError) -> AppError {
    match error {
        vel_storage::StorageError::Validation(message) => AppError::bad_request(message),
        other => AppError::from(other),
    }
}

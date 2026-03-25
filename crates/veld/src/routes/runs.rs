use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use vel_api_types::WsEventType;
use vel_api_types::{
    ApiResponse, ArtifactSummaryData, RunDetailData, RunEventData, RunSummaryData, RunUpdateRequest,
};
use vel_core::RunEventType;
use vel_core::{RunKind, RunStatus};

use crate::{broadcast::WsEnvelope, errors::AppError, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ListRunsQuery {
    pub limit: Option<u32>,
    pub kind: Option<String>,
    pub today: Option<bool>,
}

fn duration_ms(
    started_at: Option<time::OffsetDateTime>,
    finished_at: Option<time::OffsetDateTime>,
) -> Option<i64> {
    started_at.and_then(|s| finished_at.map(|f| (f - s).whole_milliseconds() as i64))
}

fn start_of_today_utc() -> i64 {
    let now = time::OffsetDateTime::now_utc();
    let date = now.date();
    date.midnight().assume_utc().unix_timestamp()
}

#[derive(Debug, Clone, Default)]
struct RunOperatorMetadata {
    retry_scheduled_at: Option<time::OffsetDateTime>,
    retry_reason: Option<String>,
    blocked_reason: Option<String>,
    unsupported_retry_override: bool,
    unsupported_retry_override_reason: Option<String>,
}

#[derive(Debug, Clone)]
struct TraceMetadata {
    trace_id: String,
    parent_run_id: Option<vel_core::RunId>,
}

fn json_string_field(value: &serde_json::Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string)
}

fn trace_metadata(run: &vel_core::Run, events: &[vel_core::RunEvent]) -> TraceMetadata {
    let parent_run_id = run
        .input_json
        .get("parent_run_id")
        .and_then(serde_json::Value::as_str)
        .map(|value| value.to_string().into())
        .or_else(|| {
            run.output_json
                .as_ref()
                .and_then(|value| json_string_field(value, "parent_run_id"))
                .map(Into::into)
        })
        .or_else(|| {
            events.iter().rev().find_map(|event| {
                json_string_field(&event.payload_json, "parent_run_id").map(Into::into)
            })
        });

    let trace_id = run
        .input_json
        .get("trace_id")
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string)
        .or_else(|| {
            run.output_json
                .as_ref()
                .and_then(|value| json_string_field(value, "trace_id"))
        })
        .or_else(|| {
            events
                .iter()
                .rev()
                .find_map(|event| json_string_field(&event.payload_json, "trace_id"))
        })
        .unwrap_or_else(|| run.id.to_string());

    TraceMetadata {
        trace_id,
        parent_run_id,
    }
}

fn run_operator_metadata(output: Option<&serde_json::Value>) -> RunOperatorMetadata {
    let retry_scheduled_at = output
        .and_then(|v| v.get("retry_scheduled_at_ts"))
        .and_then(serde_json::Value::as_i64)
        .and_then(|ts| time::OffsetDateTime::from_unix_timestamp(ts).ok());
    let retry_reason = output
        .and_then(|v| v.get("retry_reason"))
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string);
    let blocked_reason = output
        .and_then(|v| v.get("blocked_reason"))
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string);
    let unsupported_retry_override = output
        .and_then(|v| v.get("unsupported_retry_override"))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let unsupported_retry_override_reason = output
        .and_then(|v| v.get("unsupported_retry_override_reason"))
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string);
    RunOperatorMetadata {
        retry_scheduled_at,
        retry_reason,
        blocked_reason,
        unsupported_retry_override,
        unsupported_retry_override_reason,
    }
}

fn merge_output_metadata(
    existing_output: Option<&serde_json::Value>,
    status: RunStatus,
    blocked_reason: Option<&str>,
    retry_at: Option<time::OffsetDateTime>,
    retry_reason: Option<&str>,
    unsupported_retry_override: bool,
    unsupported_retry_override_reason: Option<&str>,
) -> Option<serde_json::Value> {
    let mut output = existing_output.cloned();
    if status != RunStatus::RetryScheduled && status != RunStatus::Blocked {
        return output;
    }

    let mut map = output
        .as_ref()
        .and_then(serde_json::Value::as_object)
        .cloned()
        .unwrap_or_default();

    if status == RunStatus::RetryScheduled {
        if let Some(ts) = retry_at {
            map.insert(
                "retry_scheduled_at_ts".to_string(),
                serde_json::json!(ts.unix_timestamp()),
            );
        }
        if let Some(reason) = retry_reason {
            let trimmed = reason.trim();
            if !trimmed.is_empty() {
                map.insert("retry_reason".to_string(), serde_json::json!(trimmed));
            }
        }
        if unsupported_retry_override {
            map.insert(
                "unsupported_retry_override".to_string(),
                serde_json::json!(true),
            );
            let override_reason = unsupported_retry_override_reason
                .map(str::trim)
                .filter(|reason| !reason.is_empty())
                .unwrap_or("manual operator override");
            map.insert(
                "unsupported_retry_override_reason".to_string(),
                serde_json::json!(override_reason),
            );
        } else {
            map.remove("unsupported_retry_override");
            map.remove("unsupported_retry_override_reason");
        }
    }

    if status == RunStatus::Blocked {
        if let Some(reason) = blocked_reason {
            let trimmed = reason.trim();
            if !trimmed.is_empty() {
                map.insert("blocked_reason".to_string(), serde_json::json!(trimmed));
            }
        }
    }

    output = Some(serde_json::Value::Object(map));
    output
}

fn retry_metadata_from_events(
    events: &[vel_core::RunEvent],
) -> (
    Option<time::OffsetDateTime>,
    Option<String>,
    bool,
    Option<String>,
) {
    for event in events.iter().rev() {
        if event.event_type != RunEventType::RunRetryScheduled {
            continue;
        }
        let retry_at = event
            .payload_json
            .get("retry_at")
            .and_then(serde_json::Value::as_i64)
            .and_then(|ts| time::OffsetDateTime::from_unix_timestamp(ts).ok());
        let reason = event
            .payload_json
            .get("reason")
            .and_then(serde_json::Value::as_str)
            .map(ToString::to_string);
        let unsupported_retry_override = event
            .payload_json
            .get("unsupported_retry_override")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        let unsupported_retry_override_reason = event
            .payload_json
            .get("unsupported_retry_override_reason")
            .and_then(serde_json::Value::as_str)
            .map(ToString::to_string);
        return (
            retry_at,
            reason,
            unsupported_retry_override,
            unsupported_retry_override_reason,
        );
    }
    (None, None, false, None)
}

fn automatic_retry_policy(kind: vel_core::RunKind) -> (bool, Option<String>) {
    let policy = kind.retry_policy();
    (
        policy.automatic_retry_supported,
        policy.automatic_retry_reason.map(ToString::to_string),
    )
}

async fn run_summary_data(
    state: &AppState,
    run: vel_core::Run,
) -> Result<RunSummaryData, AppError> {
    let (automatic_retry_supported, automatic_retry_reason) = automatic_retry_policy(run.kind);
    let events = state.storage.list_run_events(run.id.as_ref()).await?;
    let trace = trace_metadata(&run, &events);
    let mut metadata = run_operator_metadata(run.output_json.as_ref());
    if run.status == RunStatus::RetryScheduled {
        let (retry_at, retry_reason, unsupported_retry_override, unsupported_retry_override_reason) =
            retry_metadata_from_events(&events);
        if metadata.retry_scheduled_at.is_none() {
            metadata.retry_scheduled_at = retry_at;
        }
        if metadata.retry_reason.is_none() {
            metadata.retry_reason = retry_reason;
        }
        if !metadata.unsupported_retry_override {
            metadata.unsupported_retry_override = unsupported_retry_override;
        }
        if metadata.unsupported_retry_override_reason.is_none() {
            metadata.unsupported_retry_override_reason = unsupported_retry_override_reason;
        }
    }

    Ok(RunSummaryData {
        id: run.id,
        kind: run.kind.to_string(),
        status: run.status.to_string(),
        trace_id: trace.trace_id,
        parent_run_id: trace.parent_run_id,
        automatic_retry_supported,
        automatic_retry_reason,
        unsupported_retry_override: metadata.unsupported_retry_override,
        unsupported_retry_override_reason: metadata.unsupported_retry_override_reason,
        created_at: run.created_at,
        started_at: run.started_at,
        finished_at: run.finished_at,
        duration_ms: duration_ms(run.started_at, run.finished_at),
        retry_scheduled_at: metadata.retry_scheduled_at,
        retry_reason: metadata.retry_reason,
        blocked_reason: metadata.blocked_reason,
    })
}

pub async fn list_runs(
    State(state): State<AppState>,
    Query(q): Query<ListRunsQuery>,
) -> Result<Json<ApiResponse<Vec<RunSummaryData>>>, AppError> {
    let limit = q.limit.unwrap_or(20).clamp(1, 100);
    let kind_filter = q
        .kind
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.parse::<RunKind>()
                .map_err(|_| AppError::bad_request("invalid run kind"))
        })
        .transpose()?;
    let since_ts = q.today.unwrap_or(false).then(start_of_today_utc);
    let runs = state.storage.list_runs(kind_filter, None, limit).await?;
    let mut data = Vec::with_capacity(runs.len());
    let mut filtered_runs = runs;
    if let Some(since_ts) = since_ts {
        filtered_runs.retain(|run| run.created_at.unix_timestamp() >= since_ts);
    }
    for r in filtered_runs {
        data.push(run_summary_data(&state, r).await?);
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_run(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<RunDetailData>>, AppError> {
    let run = state
        .storage
        .get_run_by_id(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("run not found"))?;
    let events = state.storage.list_run_events(run.id.as_ref()).await?;
    let trace = trace_metadata(&run, &events);
    let (retry_at, retry_reason, unsupported_retry_override, unsupported_retry_override_reason) =
        retry_metadata_from_events(&events);
    let event_data = events
        .iter()
        .map(|e| RunEventData {
            seq: e.seq,
            event_type: e.event_type.to_string(),
            trace_id: e
                .payload_json
                .get("trace_id")
                .and_then(serde_json::Value::as_str)
                .map(ToString::to_string),
            payload: e.payload_json.clone(),
            created_at: e.created_at,
        })
        .collect();

    let artifacts = state
        .storage
        .list_artifacts_for_run(run.id.as_ref())
        .await?
        .into_iter()
        .map(|record| ArtifactSummaryData {
            artifact_id: record.artifact_id,
            artifact_type: record.artifact_type,
            title: record.title,
            storage_uri: record.storage_uri,
            storage_kind: record.storage_kind.to_string(),
            size_bytes: record.size_bytes,
        })
        .collect::<Vec<_>>();

    let mut metadata = run_operator_metadata(run.output_json.as_ref());
    if metadata.retry_scheduled_at.is_none() {
        metadata.retry_scheduled_at = retry_at;
    }
    if metadata.retry_reason.is_none() {
        metadata.retry_reason = retry_reason;
    }
    if !metadata.unsupported_retry_override {
        metadata.unsupported_retry_override = unsupported_retry_override;
    }
    if metadata.unsupported_retry_override_reason.is_none() {
        metadata.unsupported_retry_override_reason = unsupported_retry_override_reason;
    }
    let (automatic_retry_supported, automatic_retry_reason) = automatic_retry_policy(run.kind);
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        RunDetailData {
            id: run.id,
            kind: run.kind.to_string(),
            status: run.status.to_string(),
            trace_id: trace.trace_id,
            parent_run_id: trace.parent_run_id,
            automatic_retry_supported,
            automatic_retry_reason,
            unsupported_retry_override: metadata.unsupported_retry_override,
            unsupported_retry_override_reason: metadata.unsupported_retry_override_reason,
            input: run.input_json,
            output: run.output_json,
            error: run.error_json,
            created_at: run.created_at,
            started_at: run.started_at,
            finished_at: run.finished_at,
            duration_ms: duration_ms(run.started_at, run.finished_at),
            retry_scheduled_at: metadata.retry_scheduled_at,
            retry_reason: metadata.retry_reason,
            blocked_reason: metadata.blocked_reason,
            events: event_data,
            artifacts,
        },
        request_id,
    )))
}

pub async fn update_run(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<RunUpdateRequest>,
) -> Result<Json<ApiResponse<RunDetailData>>, AppError> {
    if body.retry_at.is_some() && body.status.trim() != "retry_scheduled" {
        return Err(AppError::bad_request(
            "retry_at is only valid when status is retry_scheduled",
        ));
    }
    if body.retry_after_seconds.is_some() && body.status.trim() != "retry_scheduled" {
        return Err(AppError::bad_request(
            "retry_after_seconds is only valid when status is retry_scheduled",
        ));
    }
    if body.reason.is_some() && body.status.trim() != "retry_scheduled" {
        return Err(AppError::bad_request(
            "reason is only valid when status is retry_scheduled",
        ));
    }
    if body.allow_unsupported_retry && body.status.trim() != "retry_scheduled" {
        return Err(AppError::bad_request(
            "allow_unsupported_retry is only valid when status is retry_scheduled",
        ));
    }
    if body.blocked_reason.is_some() && body.status.trim() != "blocked" {
        return Err(AppError::bad_request(
            "blocked_reason is only valid when status is blocked",
        ));
    }
    if body.retry_at.is_some() && body.retry_after_seconds.is_some() {
        return Err(AppError::bad_request(
            "retry_at and retry_after_seconds are mutually exclusive",
        ));
    }

    let status: RunStatus = body
        .status
        .trim()
        .parse()
        .map_err(|e: vel_core::VelCoreError| AppError::bad_request(e.to_string()))?;
    let id = id.trim();
    let existing = state
        .storage
        .get_run_by_id(id)
        .await?
        .ok_or_else(|| AppError::not_found("run not found"))?;
    let policy = existing.kind.retry_policy();
    if status == RunStatus::RetryScheduled
        && !policy.automatic_retry_supported
        && !body.allow_unsupported_retry
    {
        return Err(AppError::bad_request(format!(
            "automatic retry is unsupported for run kind {}; pass allow_unsupported_retry=true to override",
            existing.kind
        )));
    }
    let retry_at = if status == RunStatus::RetryScheduled {
        if let Some(at) = body.retry_at {
            Some(at)
        } else if let Some(delay) = body.retry_after_seconds {
            Some(time::OffsetDateTime::now_utc() + time::Duration::seconds(delay as i64))
        } else {
            Some(time::OffsetDateTime::now_utc())
        }
    } else {
        None
    };
    let retry_reason = body.reason.as_deref();
    let blocked_reason = body.blocked_reason.as_deref();
    let output_json = merge_output_metadata(
        existing.output_json.as_ref(),
        status,
        blocked_reason,
        retry_at,
        retry_reason,
        body.allow_unsupported_retry,
        if body.allow_unsupported_retry {
            Some("manual operator override")
        } else {
            None
        },
    );
    state
        .storage
        .update_run_status(
            id,
            status,
            None,
            None,
            output_json.as_ref(),
            existing.error_json.as_ref(),
        )
        .await?;
    if status == RunStatus::RetryScheduled {
        let payload = serde_json::json!({
            "retry_at": retry_at.expect("retry_at is computed for retry_scheduled").unix_timestamp(),
            "reason": retry_reason,
            "unsupported_retry_override": body.allow_unsupported_retry,
            "unsupported_retry_override_reason": body
                .allow_unsupported_retry
                .then_some("manual operator override"),
        });
        state
            .storage
            .append_run_event_auto(id, RunEventType::RunRetryScheduled, &payload)
            .await?;
    }
    let _ = broadcast_run_updated(&state, id).await;
    get_run(State(state), Path(id.to_string())).await
}

pub async fn broadcast_run_updated(state: &AppState, run_id: &str) -> Result<(), AppError> {
    if let Some(run) = state.storage.get_run_by_id(run_id).await? {
        let payload = serde_json::to_value(run_summary_data(state, run).await?).map_err(|e| {
            AppError::internal(format!("failed to serialize run update payload: {}", e))
        })?;
        let _ = state
            .broadcast_tx
            .send(WsEnvelope::new(WsEventType::RunsUpdated, payload));
    }
    Ok(())
}

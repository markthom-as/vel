//! Context routes: thin handlers that call the run-backed context generation service and expose current context.

use axum::{
    extract::{Query, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, ContextTimelineEntry, CurrentContextData, EndOfDayData, MorningData, TodayData,
};

use crate::services::context_generation::{
    EndOfDayContextData, MorningContextData, TodayContextData,
};
use crate::services::context_runs;
use crate::{errors::AppError, state::AppState};
use vel_core::ContextMigrator;

#[derive(Debug, serde::Deserialize)]
pub struct ContextTimelineQuery {
    pub limit: Option<u32>,
}

pub async fn today(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<TodayData>>, AppError> {
    let output = context_runs::generate_today(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_today_data(output.data),
        request_id,
    )))
}

pub async fn morning(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<MorningData>>, AppError> {
    let output = context_runs::generate_morning(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_morning_data(output.data),
        request_id,
    )))
}

pub async fn end_of_day(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<EndOfDayData>>, AppError> {
    let output = context_runs::generate_end_of_day(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_end_of_day_data(output.data),
        request_id,
    )))
}

fn map_today_data(data: TodayContextData) -> TodayData {
    TodayData {
        date: data.date,
        recent_captures: data.recent_captures.into_iter().map(Into::into).collect(),
        focus_candidates: data.focus_candidates,
        reminders: data.reminders,
    }
}

fn map_morning_data(data: MorningContextData) -> MorningData {
    MorningData {
        date: data.date,
        top_active_threads: data.top_active_threads,
        pending_commitments: data.pending_commitments,
        suggested_focus: data.suggested_focus,
        key_reminders: data.key_reminders,
    }
}

fn map_end_of_day_data(data: EndOfDayContextData) -> EndOfDayData {
    EndOfDayData {
        date: data.date,
        what_was_done: data.what_was_done.into_iter().map(Into::into).collect(),
        what_remains_open: data.what_remains_open,
        what_may_matter_tomorrow: data.what_may_matter_tomorrow,
    }
}

fn map_timeline_context_json(context_json: &str) -> serde_json::Value {
    if let Ok(context) = ContextMigrator::from_json_str(context_json) {
        return context.into_json();
    }

    serde_json::from_str::<serde_json::Value>(context_json)
        .ok()
        .filter(|value| value.is_object())
        .unwrap_or_else(|| serde_json::json!({}))
}

fn map_timeline_entries(rows: Vec<(String, i64, String)>) -> Vec<ContextTimelineEntry> {
    rows.into_iter()
        .map(|(id, timestamp, context_json)| ContextTimelineEntry {
            id,
            timestamp,
            context: map_timeline_context_json(&context_json),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use time::OffsetDateTime;
    use vel_core::{CaptureId, ContextCapture};

    #[test]
    fn maps_today_service_data_to_api_dto() {
        let capture = ContextCapture {
            capture_id: CaptureId::from("cap_test_today".to_string()),
            capture_type: "quick_note".to_string(),
            content_text: "remember lidar budget".to_string(),
            occurred_at: OffsetDateTime::now_utc(),
            source_device: Some("test-device".to_string()),
        };
        let dto = map_today_data(TodayContextData {
            date: "2026-03-17".to_string(),
            recent_captures: vec![capture.clone()],
            focus_candidates: vec!["lidar".to_string()],
            reminders: vec!["remember lidar budget".to_string()],
        });

        assert_eq!(dto.date, "2026-03-17");
        assert_eq!(dto.recent_captures.len(), 1);
        assert_eq!(dto.recent_captures[0].capture_id, capture.capture_id);
        assert_eq!(dto.recent_captures[0].content_text, capture.content_text);
        assert_eq!(dto.focus_candidates, vec!["lidar".to_string()]);
    }

    #[test]
    fn maps_end_of_day_service_data_to_api_dto() {
        let capture = ContextCapture {
            capture_id: CaptureId::from("cap_test_eod".to_string()),
            capture_type: "quick_note".to_string(),
            content_text: "follow up with Cornelius".to_string(),
            occurred_at: OffsetDateTime::now_utc(),
            source_device: None,
        };
        let dto = map_end_of_day_data(EndOfDayContextData {
            date: "2026-03-17".to_string(),
            what_was_done: vec![capture.clone()],
            what_remains_open: vec!["follow up with Cornelius".to_string()],
            what_may_matter_tomorrow: vec!["budget".to_string()],
        });

        assert_eq!(dto.date, "2026-03-17");
        assert_eq!(dto.what_was_done.len(), 1);
        assert_eq!(dto.what_was_done[0].capture_id, capture.capture_id);
        assert_eq!(
            dto.what_remains_open,
            vec!["follow up with Cornelius".to_string()]
        );
    }

    #[test]
    fn maps_morning_service_data_to_api_dto() {
        let dto = map_morning_data(MorningContextData {
            date: "2026-03-17".to_string(),
            top_active_threads: vec!["forecast".to_string()],
            pending_commitments: vec!["follow up".to_string()],
            suggested_focus: Some("forecast".to_string()),
            key_reminders: vec!["follow up".to_string()],
        });

        assert_eq!(dto.date, "2026-03-17");
        assert_eq!(dto.top_active_threads, vec!["forecast".to_string()]);
        assert_eq!(dto.pending_commitments, vec!["follow up".to_string()]);
        assert_eq!(dto.suggested_focus.as_deref(), Some("forecast"));
    }

    #[test]
    fn maps_timeline_rows_using_typed_context_migration_first() {
        let entries = map_timeline_entries(vec![(
            "ctl_test".to_string(),
            1_710_000_000,
            r#"{
              "mode": "morning_mode",
              "morning_state": "awake_unstarted",
              "meds_status": "pending",
              "attention_state": "drifting",
              "custom_future_field": { "ok": true }
            }"#
            .to_string(),
        )]);

        let context = &entries[0].context;
        assert!(context.is_object());
        assert_eq!(context["mode"], "morning_mode");
        assert_eq!(context["morning_state"], "awake_unstarted");
        assert_eq!(context["meds_status"], "pending");
        assert_eq!(context["inferred_activity"], "");
        assert_eq!(context["prep_window_active"], false);
        assert_eq!(context["custom_future_field"], json!({ "ok": true }));
    }

    #[test]
    fn maps_timeline_rows_to_empty_object_for_malformed_context_json() {
        let entries = map_timeline_entries(vec![
            ("ctl_bad_json".to_string(), 1_710_000_001, "{not-json}".to_string()),
            ("ctl_scalar".to_string(), 1_710_000_002, "true".to_string()),
        ]);

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].id, "ctl_bad_json");
        assert_eq!(entries[0].timestamp, 1_710_000_001);
        assert_eq!(entries[0].context, json!({}));
        assert_eq!(entries[1].id, "ctl_scalar");
        assert_eq!(entries[1].timestamp, 1_710_000_002);
        assert_eq!(entries[1].context, json!({}));
    }
}

/// GET /v1/context/current — persistent current context (singleton) written by inference engine.
pub async fn current(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Option<CurrentContextData>>>, AppError> {
    let row = state.storage.get_current_context().await?;
    let data = row.map(|(computed_at, context)| CurrentContextData {
        computed_at,
        context: context.into_json(),
    });
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

/// GET /v1/context/timeline — recent material context transitions (append-only timeline).
pub async fn timeline(
    State(state): State<AppState>,
    Query(q): Query<ContextTimelineQuery>,
) -> Result<Json<ApiResponse<Vec<ContextTimelineEntry>>>, AppError> {
    let limit = q.limit.unwrap_or(20);
    let rows = state.storage.list_context_timeline(limit).await?;
    let entries = map_timeline_entries(rows);
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(entries, request_id)))
}

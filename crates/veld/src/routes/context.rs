//! Context routes: thin handlers that call the run-backed context generation service and expose current context.

use axum::{
    extract::{Query, State},
    Json,
};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, ContextTimelineEntry, CurrentContextData, EndOfDayData, MorningData, TodayData,
};
use vel_core::ContextMigrator;

use crate::services::context_generation::{
    EndOfDayContextData, MorningContextData, TodayContextData,
};
use crate::services::context_runs;
use crate::{errors::AppError, state::AppState};

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

#[cfg(test)]
mod tests {
    use super::*;
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
}

/// GET /v1/context/current — persistent current context (singleton) written by inference engine.
pub async fn current(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Option<CurrentContextData>>>, AppError> {
    let row = state.storage.get_current_context().await?;
    let data = row.map(|(computed_at, context_str)| {
        let context = serde_json::from_str(&context_str).unwrap_or(serde_json::json!({}));
        let _ = ContextMigrator::from_json_value(context.clone());
        CurrentContextData {
            computed_at,
            context,
        }
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
    let entries: Vec<ContextTimelineEntry> = rows
        .into_iter()
        .filter_map(|(id, timestamp, context_json)| {
            let context = serde_json::from_str(&context_json).ok()?;
            Some(ContextTimelineEntry {
                id,
                timestamp,
                context,
            })
        })
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(entries, request_id)))
}

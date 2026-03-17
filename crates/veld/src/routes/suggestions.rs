//! Suggestions API: list, inspect, accept, reject, modify. See vel-agent-next-implementation-steps.md.

use axum::{extract::Path, extract::Query, extract::State, Json};
use uuid::Uuid;
use vel_api_types::{ApiResponse, SuggestionData, SuggestionUpdateRequest};

use crate::{errors::AppError, state::AppState};

fn map_suggestion(record: vel_storage::SuggestionRecord) -> SuggestionData {
    SuggestionData {
        id: record.id,
        suggestion_type: record.suggestion_type,
        state: record.state,
        title: record.title,
        summary: record.summary,
        priority: record.priority,
        confidence: record.confidence,
        evidence_count: record.evidence_count,
        decision_context_summary: record
            .decision_context_json
            .as_ref()
            .and_then(|json| json.get("summary"))
            .and_then(serde_json::Value::as_str)
            .map(ToString::to_string),
        payload: record.payload_json,
        created_at: record.created_at,
        resolved_at: record.resolved_at,
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ListSuggestionsQuery {
    pub state: Option<String>,
    pub limit: Option<u32>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<ListSuggestionsQuery>,
) -> Result<Json<ApiResponse<Vec<SuggestionData>>>, AppError> {
    let limit = q.limit.unwrap_or(50).min(100);
    let rows = state
        .storage
        .list_suggestions(q.state.as_deref(), limit)
        .await?;
    let data: Vec<SuggestionData> = rows.into_iter().map(map_suggestion).collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<SuggestionData>>, AppError> {
    let row = state
        .storage
        .get_suggestion_by_id(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("suggestion not found"))?;
    let data = map_suggestion(row);
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<SuggestionUpdateRequest>,
) -> Result<Json<ApiResponse<SuggestionData>>, AppError> {
    let id = id.trim();
    let _existing = state
        .storage
        .get_suggestion_by_id(id)
        .await?
        .ok_or_else(|| AppError::not_found("suggestion not found"))?;
    let new_state = body
        .state
        .as_deref()
        .ok_or_else(|| AppError::bad_request("state required"))?;
    let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
    let resolved_at = if new_state == "accepted" || new_state == "rejected" {
        Some(now_ts)
    } else {
        None
    };
    let payload_json = body
        .payload
        .as_ref()
        .map(|p| p.to_string())
        .unwrap_or_default();
    state
        .storage
        .update_suggestion_state(
            id,
            new_state,
            resolved_at,
            if payload_json.is_empty() {
                None
            } else {
                Some(&payload_json)
            },
        )
        .await?;
    let row = state.storage.get_suggestion_by_id(id).await?.unwrap();
    let data = map_suggestion(row);
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

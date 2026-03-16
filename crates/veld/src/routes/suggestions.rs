//! Suggestions API: list, inspect, accept, reject, modify. See vel-agent-next-implementation-steps.md.

use axum::{extract::Path, extract::Query, extract::State, Json};
use uuid::Uuid;
use vel_api_types::{ApiResponse, SuggestionData, SuggestionUpdateRequest};

use crate::{errors::AppError, state::AppState};

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
    let data: Vec<SuggestionData> = rows
        .into_iter()
        .map(
            |(id, stype, state, payload_json, created_at, resolved_at)| {
                let payload = serde_json::from_str(&payload_json).unwrap_or(serde_json::json!({}));
                SuggestionData {
                    id,
                    suggestion_type: stype,
                    state,
                    payload,
                    created_at,
                    resolved_at,
                }
            },
        )
        .collect();
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
    let (id, stype, state, payload_json, created_at, resolved_at) = row;
    let payload = serde_json::from_str(&payload_json).unwrap_or(serde_json::json!({}));
    let data = SuggestionData {
        id,
        suggestion_type: stype,
        state,
        payload,
        created_at,
        resolved_at,
    };
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
    let (id, stype, state, payload_json, created_at, resolved_at) = row;
    let payload = serde_json::from_str(&payload_json).unwrap_or(serde_json::json!({}));
    let data = SuggestionData {
        id,
        suggestion_type: stype,
        state,
        payload,
        created_at,
        resolved_at,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

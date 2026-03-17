//! Suggestions API: list, inspect, accept, reject, modify. See vel-agent-next-implementation-steps.md.

use axum::{extract::Path, extract::Query, extract::State, Json};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, SuggestionActionRequest, SuggestionData, SuggestionEvidenceData,
    SuggestionUpdateRequest,
};

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
        decision_context: None,
        evidence: None,
        payload: record.payload_json,
        created_at: record.created_at,
        resolved_at: record.resolved_at,
    }
}

fn map_suggestion_evidence(
    record: vel_storage::SuggestionEvidenceRecord,
) -> SuggestionEvidenceData {
    SuggestionEvidenceData {
        id: record.id,
        evidence_type: record.evidence_type,
        ref_id: record.ref_id,
        evidence: record.evidence_json,
        weight: record.weight,
        created_at: record.created_at,
    }
}

pub async fn evidence(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Vec<SuggestionEvidenceData>>>, AppError> {
    let suggestion_id = id.trim();
    let _ = state
        .storage
        .get_suggestion_by_id(suggestion_id)
        .await?
        .ok_or_else(|| AppError::not_found("suggestion not found"))?;
    let evidence = state
        .storage
        .list_suggestion_evidence(suggestion_id)
        .await?
        .into_iter()
        .map(map_suggestion_evidence)
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(evidence, request_id)))
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
    let evidence = state
        .storage
        .list_suggestion_evidence(&row.id)
        .await?
        .into_iter()
        .map(map_suggestion_evidence)
        .collect();
    let mut data = map_suggestion(row.clone());
    data.decision_context = row.decision_context_json;
    data.evidence = Some(evidence);
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<SuggestionUpdateRequest>,
) -> Result<Json<ApiResponse<SuggestionData>>, AppError> {
    let id = id.trim();
    let existing = state
        .storage
        .get_suggestion_by_id(id)
        .await?
        .ok_or_else(|| AppError::not_found("suggestion not found"))?;
    let new_state = body
        .state
        .as_deref()
        .ok_or_else(|| AppError::bad_request("state required"))?;
    apply_state_transition(&state, &existing, new_state, body.payload, None).await
}

pub async fn accept(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<SuggestionData>>, AppError> {
    let id = id.trim();
    let existing = state
        .storage
        .get_suggestion_by_id(id)
        .await?
        .ok_or_else(|| AppError::not_found("suggestion not found"))?;
    apply_state_transition(&state, &existing, "accepted", None, None).await
}

pub async fn reject(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<SuggestionActionRequest>,
) -> Result<Json<ApiResponse<SuggestionData>>, AppError> {
    let id = id.trim();
    let existing = state
        .storage
        .get_suggestion_by_id(id)
        .await?
        .ok_or_else(|| AppError::not_found("suggestion not found"))?;
    apply_state_transition(&state, &existing, "rejected", None, body.reason).await
}

async fn apply_state_transition(
    state: &AppState,
    existing: &vel_storage::SuggestionRecord,
    new_state: &str,
    payload_override: Option<serde_json::Value>,
    reason: Option<String>,
) -> Result<Json<ApiResponse<SuggestionData>>, AppError> {
    let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
    let resolved_at = if new_state == "accepted" || new_state == "rejected" {
        Some(now_ts)
    } else {
        None
    };
    let payload_json = merged_payload(existing, payload_override, reason);
    state
        .storage
        .update_suggestion_state(
            &existing.id,
            new_state,
            resolved_at,
            payload_json.as_deref(),
        )
        .await?;
    if new_state == "accepted" {
        let applied = crate::services::adaptive_policies::apply_suggestion_acceptance(
            &state.storage,
            &existing.suggestion_type,
            &existing.payload_json,
        )
        .await?;
        if applied {
            let _ = crate::services::evaluate::run_and_broadcast(state).await;
        }
    }
    let row = state.storage.get_suggestion_by_id(&existing.id).await?.unwrap();
    let mut data = map_suggestion(row.clone());
    data.decision_context = row.decision_context_json;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

fn merged_payload(
    existing: &vel_storage::SuggestionRecord,
    payload_override: Option<serde_json::Value>,
    reason: Option<String>,
) -> Option<String> {
    let Some(reason) = reason else {
        return payload_override.map(|payload| payload.to_string());
    };

    let mut payload = payload_override.unwrap_or_else(|| existing.payload_json.clone());
    if let Some(object) = payload.as_object_mut() {
        object.insert(
            "rejection_reason".to_string(),
            serde_json::Value::String(reason),
        );
        Some(payload.to_string())
    } else {
        Some(
            serde_json::json!({
                "value": payload,
                "rejection_reason": reason,
            })
            .to_string(),
        )
    }
}

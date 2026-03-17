use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use vel_api_types::{ApiResponse, UncertaintyData};

use crate::{errors::AppError, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ListUncertaintyQuery {
    pub status: Option<String>,
    pub limit: Option<u32>,
}

fn map_uncertainty(record: vel_storage::UncertaintyRecord) -> UncertaintyData {
    UncertaintyData {
        id: record.id,
        subject_type: record.subject_type,
        subject_id: record.subject_id,
        decision_kind: record.decision_kind,
        confidence_band: record.confidence_band,
        confidence_score: record.confidence_score,
        reasons: record.reasons_json,
        missing_evidence: record.missing_evidence_json,
        resolution_mode: record.resolution_mode,
        status: record.status,
        created_at: record.created_at,
        resolved_at: record.resolved_at,
    }
}

pub async fn list_uncertainty(
    State(state): State<AppState>,
    Query(query): Query<ListUncertaintyQuery>,
) -> Result<Json<ApiResponse<Vec<UncertaintyData>>>, AppError> {
    let status = query.status.as_deref().or(Some("open"));
    let limit = query.limit.unwrap_or(50);
    let data = state
        .storage
        .list_uncertainty_records(status, limit)
        .await?
        .into_iter()
        .map(map_uncertainty)
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_uncertainty(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<UncertaintyData>>, AppError> {
    let record = state
        .storage
        .get_uncertainty_record(&id)
        .await?
        .ok_or_else(|| AppError::not_found("uncertainty record not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_uncertainty(record),
        request_id,
    )))
}

pub async fn resolve_uncertainty(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<UncertaintyData>>, AppError> {
    let record = state
        .storage
        .resolve_uncertainty_record(&id)
        .await?
        .ok_or_else(|| AppError::not_found("uncertainty record not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_uncertainty(record),
        request_id,
    )))
}

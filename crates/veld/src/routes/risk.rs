//! Risk inspection: **read-only**. Return persisted risk snapshots. Do not recompute.
//! Recompute only via POST /v1/evaluate. See docs/tickets/repo-feedback/001.

use axum::{extract::Path, extract::State, Json};
use uuid::Uuid;
use vel_api_types::{ApiResponse, RiskData};

use crate::services::risk::snapshot_from_row;
use crate::{errors::AppError, state::AppState};

/// GET /v1/risk — list latest risk per commitment from storage (read-only).
pub async fn list_risk(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<RiskData>>>, AppError> {
    let rows = state.storage.list_commitment_risk_latest_all().await?;
    let data: Vec<RiskData> = rows
        .into_iter()
        .map(
            |(_, commitment_id, risk_score, risk_level, factors_json, computed_at)| {
                let snapshot = snapshot_from_row(
                    commitment_id,
                    risk_score,
                    risk_level,
                    &factors_json,
                    Some(computed_at),
                );
                RiskData::from(snapshot)
            },
        )
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

/// GET /v1/risk/:id — latest risk for one commitment from storage (read-only).
pub async fn get_commitment_risk(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<RiskData>>, AppError> {
    let commitment_id = id.trim();
    let rows = state
        .storage
        .list_commitment_risk_recent(commitment_id, 1)
        .await?;
    let (_, risk_score, risk_level, factors_json, computed_at) =
        rows.into_iter().next().ok_or_else(|| {
            AppError::not_found(
                "commitment not found or has no risk snapshot (run POST /v1/evaluate first)",
            )
        })?;
    let data = RiskData::from(snapshot_from_row(
        commitment_id.to_string(),
        risk_score,
        risk_level,
        &factors_json,
        Some(computed_at),
    ));
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

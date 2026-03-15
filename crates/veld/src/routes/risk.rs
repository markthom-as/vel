//! Risk inspection: compute and return risk snapshots. See vel-risk-engine-spec.md.

use axum::{extract::Path, extract::State, Json};
use uuid::Uuid;
use vel_api_types::{ApiResponse, RiskData};

use crate::{errors::AppError, state::AppState};
use crate::services::risk;

pub async fn compute_and_list(State(state): State<AppState>) -> Result<Json<ApiResponse<Vec<RiskData>>>, AppError> {
    let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
    let snapshots = risk::run(&state.storage, now_ts).await?;
    let data: Vec<RiskData> = snapshots
        .into_iter()
        .map(|s| {
            let factors = serde_json::from_str(&s.factors_json).unwrap_or(serde_json::json!({}));
            RiskData {
                commitment_id: s.commitment_id,
                risk_score: s.risk_score,
                risk_level: s.risk_level,
                factors,
                computed_at: Some(now_ts),
            }
        })
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_commitment_risk(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<RiskData>>, AppError> {
    let commitment_id = id.trim();
    let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
    let snapshots = risk::run(&state.storage, now_ts).await?;
    let s = snapshots
        .into_iter()
        .find(|s| s.commitment_id == commitment_id)
        .ok_or_else(|| AppError::not_found("commitment not found or has no risk snapshot"))?;
    let factors = serde_json::from_str(&s.factors_json).unwrap_or(serde_json::json!({}));
    let data = RiskData {
        commitment_id: s.commitment_id,
        risk_score: s.risk_score,
        risk_level: s.risk_level,
        factors,
        computed_at: Some(now_ts),
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

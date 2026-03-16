use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use vel_api_types::{ApiResponse, SignalCreateRequest, SignalData};
use vel_storage::SignalInsert;

use crate::{errors::AppError, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ListSignalsQuery {
    pub signal_type: Option<String>,
    pub since_ts: Option<i64>,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    50
}

pub async fn create_signal(
    State(state): State<AppState>,
    Json(payload): Json<SignalCreateRequest>,
) -> Result<Json<ApiResponse<SignalData>>, AppError> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let timestamp = payload.timestamp.unwrap_or(now);
    let signal_id = state
        .storage
        .insert_signal(SignalInsert {
            signal_type: payload.signal_type,
            source: payload.source,
            source_ref: payload.source_ref,
            timestamp,
            payload_json: Some(payload.payload),
        })
        .await?;

    if let Err(e) = state
        .storage
        .emit_event(
            "SIGNAL_INGESTED",
            "signal",
            Some(&signal_id),
            &serde_json::json!({ "signal_id": signal_id }).to_string(),
        )
        .await
    {
        tracing::warn!(error = %e, "failed to emit SIGNAL_INGESTED");
    }

    let signals = state.storage.list_signals(None, None, 1).await?;
    let record = signals
        .into_iter()
        .next()
        .ok_or_else(|| AppError::internal("signal not found after insert"))?;
    let data = SignalData {
        signal_id: record.signal_id,
        signal_type: record.signal_type,
        source: record.source,
        source_ref: record.source_ref,
        timestamp: record.timestamp,
        payload: record.payload_json,
        created_at: record.created_at,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn list_signals(
    State(state): State<AppState>,
    Query(q): Query<ListSignalsQuery>,
) -> Result<Json<ApiResponse<Vec<SignalData>>>, AppError> {
    let records = state
        .storage
        .list_signals(q.signal_type.as_deref(), q.since_ts, q.limit)
        .await?;
    let data: Vec<SignalData> = records
        .into_iter()
        .map(|r| SignalData {
            signal_id: r.signal_id,
            signal_type: r.signal_type,
            source: r.source,
            source_ref: r.source_ref,
            timestamp: r.timestamp,
            payload: r.payload_json,
            created_at: r.created_at,
        })
        .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

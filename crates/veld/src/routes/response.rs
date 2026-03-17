use axum::Json;
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;
use vel_api_types::ApiResponse;

use crate::errors::AppError;

pub fn success<T>(data: T) -> Json<ApiResponse<T>> {
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Json(ApiResponse::success(data, request_id))
}

pub fn map_request<T>(value: impl Serialize, label: &str) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    let json = serde_json::to_value(value)
        .map_err(|error| AppError::bad_request(format!("invalid {label}: {error}")))?;
    serde_json::from_value(json)
        .map_err(|error| AppError::bad_request(format!("invalid {label}: {error}")))
}

pub fn map_response<T>(value: impl Serialize, label: &str) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    let json = serde_json::to_value(value)
        .map_err(|error| AppError::internal(format!("failed to serialize {label}: {error}")))?;
    serde_json::from_value(json)
        .map_err(|error| AppError::internal(format!("failed to map {label}: {error}")))
}

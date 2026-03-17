use axum::Json;
use uuid::Uuid;
use vel_api_types::ApiResponse;

pub fn success<T>(data: T) -> Json<ApiResponse<T>> {
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Json(ApiResponse::success(data, request_id))
}

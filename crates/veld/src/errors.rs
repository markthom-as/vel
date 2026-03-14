use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;
use vel_api_types::ApiResponse;

pub struct AppError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl AppError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "validation_error",
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "internal_error",
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let request_id = format!("req_{}", Uuid::new_v4().simple());
        let body = Json(ApiResponse::<serde_json::Value>::error(
            self.code,
            self.message,
            request_id,
        ));

        (self.status, body).into_response()
    }
}

impl From<vel_storage::StorageError> for AppError {
    fn from(error: vel_storage::StorageError) -> Self {
        Self::internal(error.to_string())
    }
}


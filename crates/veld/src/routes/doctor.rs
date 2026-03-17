//! Diagnostic endpoint for `vel doctor`. Thin handler that calls the doctor service.

use axum::extract::State;
use axum::Json;
use uuid::Uuid;
use vel_api_types::{ApiResponse, DiagnosticCheck, DiagnosticStatus, DoctorData};

use crate::services::doctor;
use crate::{errors::AppError, state::AppState};

pub async fn doctor(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<DoctorData>>, AppError> {
    let report = doctor::run_diagnostics(&state).await;
    let data = DoctorData {
        checks: report
            .checks
            .into_iter()
            .map(|check| DiagnosticCheck {
                name: check.name,
                status: match check.status {
                    doctor::DoctorCheckStatus::Ok => DiagnosticStatus::Ok,
                    doctor::DoctorCheckStatus::Fail => DiagnosticStatus::Fail,
                },
                message: check.message,
            })
            .collect(),
        schema_version: report.schema_version,
        version: report.version,
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

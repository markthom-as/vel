use axum::{
    extract::{Query, State},
    response::Html,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use vel_api_types::{ApiResponse, GoogleCalendarAuthStartData, IntegrationsData};

use crate::{errors::AppError, services::integrations, state::AppState};

pub async fn get_integrations(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<IntegrationsData>>, AppError> {
    let data = integrations::get_integrations_with_config(&state.storage, &state.config).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct GoogleCalendarUpdateRequest {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub selected_calendar_ids: Option<Vec<String>>,
    pub all_calendars_selected: Option<bool>,
}

pub async fn patch_google_calendar(
    State(state): State<AppState>,
    Json(payload): Json<GoogleCalendarUpdateRequest>,
) -> Result<Json<ApiResponse<IntegrationsData>>, AppError> {
    integrations::update_google_settings(
        &state.storage,
        payload.client_id,
        payload.client_secret,
        payload.selected_calendar_ids,
        payload.all_calendars_selected,
    )
    .await?;
    let data = integrations::get_integrations_with_config(&state.storage, &state.config).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn disconnect_google_calendar(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<IntegrationsData>>, AppError> {
    integrations::disconnect_google_calendar(&state.storage).await?;
    let data = integrations::get_integrations_with_config(&state.storage, &state.config).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn start_google_calendar_auth(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<GoogleCalendarAuthStartData>>, AppError> {
    let data = integrations::start_google_auth(&state.storage, &state.config).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct GoogleCalendarCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
}

pub async fn google_calendar_oauth_callback(
    State(state): State<AppState>,
    Query(query): Query<GoogleCalendarCallbackQuery>,
) -> Result<Html<String>, AppError> {
    let code = query
        .code
        .as_deref()
        .ok_or_else(|| AppError::bad_request("google oauth callback missing code"))?;
    let oauth_state = query
        .state
        .as_deref()
        .ok_or_else(|| AppError::bad_request("google oauth callback missing state"))?;

    integrations::complete_google_auth(&state.storage, &state.config, oauth_state, code).await?;
    Ok(Html(
        "<html><body style=\"background:#09090b;color:#f4f4f5;font-family:sans-serif;padding:32px\"><h1>Google Calendar connected</h1><p>You can close this window and return to Vel settings.</p></body></html>".to_string(),
    ))
}

#[derive(Debug, Deserialize)]
pub struct TodoistUpdateRequest {
    pub api_token: Option<String>,
}

pub async fn patch_todoist(
    State(state): State<AppState>,
    Json(payload): Json<TodoistUpdateRequest>,
) -> Result<Json<ApiResponse<IntegrationsData>>, AppError> {
    integrations::update_todoist_settings(&state.storage, payload.api_token).await?;
    let data = integrations::get_integrations_with_config(&state.storage, &state.config).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn disconnect_todoist(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<IntegrationsData>>, AppError> {
    integrations::disconnect_todoist(&state.storage).await?;
    let data = integrations::get_integrations_with_config(&state.storage, &state.config).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

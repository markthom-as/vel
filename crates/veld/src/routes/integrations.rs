use axum::{
    extract::{Path, Query, State},
    response::Html,
    Json,
};
use serde::Deserialize;
use serde_json::Value as JsonValue;
use uuid::Uuid;
use vel_api_types::{
    ActionExplainData, ApiResponse, CanonicalExecutionDispatchData,
    CanonicalGoogleCalendarWriteIntentRequestData, CanonicalTodoistWriteIntentRequestData,
    CanonicalTodoistWriteIntentResponseData, CanonicalWriteIntentResponseData,
    GoogleCalendarAuthStartData, GoogleCalendarIntegrationData, IntegrationCalendarData,
    IntegrationConnectionData, IntegrationConnectionEventData,
    IntegrationConnectionSettingRefData, IntegrationGuidanceData, IntegrationLogEventData,
    IntegrationsData, LocalIntegrationData, LocalIntegrationPathSelectionData, TaskEventData,
    TaskFieldChangeData, TodoistIntegrationData,
};

use crate::{
    errors::AppError,
    services::{
        audit_emitter::AuditEmitter,
        gcal_write_bridge::{
            GoogleCalendarWriteBridgeRequest, GoogleCalendarWriteBridgeOutcome,
            bridge_google_calendar_write,
        },
        integrations,
        legacy_compat::deprecated_write_path_error,
        todoist_write_bridge::{TodoistWriteBridgeRequest, TodoistWriteBridgeOutcome, bridge_todoist_write},
    },
    state::AppState,
};

fn map_integration_log_event_dto(
    event: integrations::IntegrationLogEvent,
) -> IntegrationLogEventData {
    IntegrationLogEventData {
        id: event.id,
        integration_id: event.integration_id,
        event_name: event.event_name,
        status: event.status,
        message: event.message,
        payload: event.payload,
        created_at: event.created_at,
    }
}

async fn emit_canonical_write_audit(
    pool: &sqlx::SqlitePool,
    action_name: &str,
    object_id: &str,
    requested_change: &JsonValue,
    dry_run: bool,
    write_intent_id: &str,
    dispatch: Option<&crate::services::write_intent_dispatch::ExecutionDispatch>,
) -> Result<(), AppError> {
    let field_captures = requested_change
        .as_object()
        .map(|fields| {
            fields
                .iter()
                .map(|(field, value)| vel_core::AuditFieldCapture {
                    field: field.clone(),
                    before_after: None,
                    diff: Some(value.clone()),
                    reference: Some("requested_change".to_string()),
                    redacted: false,
                })
                .collect()
        })
        .unwrap_or_default();

    let (outcome, reason, downstream_operation_ref) = if dry_run {
        (
            vel_core::AuditEventKind::DryRun,
            "canonical dry_run recorded without provider mutation".to_string(),
            None,
        )
    } else if let Some(dispatch) = dispatch {
        (
            vel_core::AuditEventKind::DispatchSucceeded,
            "canonical write path dispatched through write_intent".to_string(),
            Some(dispatch.downstream.downstream_operation_ref.clone()),
        )
    } else {
        (
            vel_core::AuditEventKind::Allowed,
            "canonical write path allowed without downstream dispatch".to_string(),
            None,
        )
    };

    AuditEmitter
        .emit(
            pool,
            &vel_core::AuditRecord {
                action_name: action_name.to_string(),
                target_object_refs: vec![object_id.to_string()],
                dry_run,
                approval_required: !dry_run && downstream_operation_ref.is_none(),
                outcome,
                reason,
                field_captures,
                write_intent_ref: Some(write_intent_id.to_string()),
                downstream_operation_ref,
            },
        )
        .await?;

    Ok(())
}

fn map_execution_dispatch_dto(
    dispatch: crate::services::write_intent_dispatch::ExecutionDispatch,
) -> CanonicalExecutionDispatchData {
    CanonicalExecutionDispatchData {
        write_intent_id: dispatch.write_intent_id,
        approved_record_id: dispatch.approved_record_id,
        executing_record_id: dispatch.executing_record_id,
        terminal_record_id: dispatch.terminal_record_id,
        downstream_operation_ref: dispatch.downstream.downstream_operation_ref,
        downstream_status: dispatch.downstream.status,
        downstream_result: dispatch.downstream.result,
        downstream_error: dispatch.downstream.error,
    }
}

fn map_task_event_dto(
    event: vel_adapters_todoist::ownership_sync::TaskEventRecord,
) -> TaskEventData {
    TaskEventData {
        id: event.id,
        task_ref: event.task_ref,
        event_type: event.event_type,
        provenance: event.provenance,
        field_changes: event
            .field_changes
            .into_iter()
            .map(|change| TaskFieldChangeData {
                field_name: change.field_name,
                old_value: change.old_value,
                new_value: change.new_value,
            })
            .collect(),
    }
}

fn map_todoist_write_response(
    outcome: TodoistWriteBridgeOutcome,
) -> CanonicalTodoistWriteIntentResponseData {
    CanonicalTodoistWriteIntentResponseData {
        write_intent_id: outcome.write_intent_id,
        explain: ActionExplainData::from(outcome.explain),
        dispatch: outcome.dispatch.map(map_execution_dispatch_dto),
        task_events: outcome.task_events.into_iter().map(map_task_event_dto).collect(),
    }
}

fn map_google_write_response(
    outcome: GoogleCalendarWriteBridgeOutcome,
) -> CanonicalWriteIntentResponseData {
    CanonicalWriteIntentResponseData {
        write_intent_id: outcome.write_intent_id,
        explain: ActionExplainData::from(outcome.explain),
        dispatch: outcome.dispatch.map(map_execution_dispatch_dto),
    }
}

impl From<integrations::IntegrationGuidanceOutput> for IntegrationGuidanceData {
    fn from(value: integrations::IntegrationGuidanceOutput) -> Self {
        Self {
            title: value.title,
            detail: value.detail,
            action: value.action,
        }
    }
}

impl From<integrations::IntegrationCalendarOutput> for IntegrationCalendarData {
    fn from(value: integrations::IntegrationCalendarOutput) -> Self {
        Self {
            id: value.id,
            summary: value.summary,
            primary: value.primary,
            selected: value.selected,
        }
    }
}

impl From<integrations::GoogleCalendarIntegrationOutput> for GoogleCalendarIntegrationData {
    fn from(value: integrations::GoogleCalendarIntegrationOutput) -> Self {
        Self {
            configured: value.configured,
            connected: value.connected,
            has_client_id: value.has_client_id,
            has_client_secret: value.has_client_secret,
            calendars: value.calendars.into_iter().map(Into::into).collect(),
            all_calendars_selected: value.all_calendars_selected,
            last_sync_at: value.last_sync_at,
            last_sync_status: value.last_sync_status,
            last_error: value.last_error,
            last_item_count: value.last_item_count,
            guidance: value.guidance.map(Into::into),
        }
    }
}

impl From<integrations::TodoistIntegrationOutput> for TodoistIntegrationData {
    fn from(value: integrations::TodoistIntegrationOutput) -> Self {
        Self {
            configured: value.configured,
            connected: value.connected,
            has_api_token: value.has_api_token,
            last_sync_at: value.last_sync_at,
            last_sync_status: value.last_sync_status,
            last_error: value.last_error,
            last_item_count: value.last_item_count,
            guidance: value.guidance.map(Into::into),
        }
    }
}

impl From<integrations::LocalIntegrationOutput> for LocalIntegrationData {
    fn from(value: integrations::LocalIntegrationOutput) -> Self {
        Self {
            configured: value.configured,
            guidance: value.guidance.map(Into::into),
            source_path: value.source_path,
            selected_paths: value.selected_paths,
            available_paths: value.available_paths,
            internal_paths: value.internal_paths,
            suggested_paths: value.suggested_paths,
            source_kind: value.source_kind,
            last_sync_at: value.last_sync_at,
            last_sync_status: value.last_sync_status,
            last_error: value.last_error,
            last_item_count: value.last_item_count,
        }
    }
}

impl From<integrations::IntegrationsOutput> for IntegrationsData {
    fn from(value: integrations::IntegrationsOutput) -> Self {
        Self {
            google_calendar: value.google_calendar.into(),
            todoist: value.todoist.into(),
            activity: value.activity.into(),
            health: value.health.into(),
            git: value.git.into(),
            messaging: value.messaging.into(),
            reminders: value.reminders.into(),
            notes: value.notes.into(),
            transcripts: value.transcripts.into(),
        }
    }
}

impl From<integrations::IntegrationConnectionSettingRefOutput>
    for IntegrationConnectionSettingRefData
{
    fn from(value: integrations::IntegrationConnectionSettingRefOutput) -> Self {
        Self {
            setting_key: value.setting_key,
            setting_value: value.setting_value,
            created_at: value.created_at,
        }
    }
}

impl From<integrations::IntegrationConnectionOutput> for IntegrationConnectionData {
    fn from(value: integrations::IntegrationConnectionOutput) -> Self {
        Self {
            id: value.id,
            family: value.family,
            provider_key: value.provider_key,
            status: value.status,
            display_name: value.display_name,
            account_ref: value.account_ref,
            metadata: value.metadata,
            created_at: value.created_at,
            updated_at: value.updated_at,
            setting_refs: value.setting_refs.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<integrations::IntegrationConnectionEventOutput> for IntegrationConnectionEventData {
    fn from(value: integrations::IntegrationConnectionEventOutput) -> Self {
        Self {
            id: value.id,
            connection_id: value.connection_id,
            event_type: value.event_type,
            payload: value.payload,
            timestamp: value.timestamp,
            created_at: value.created_at,
        }
    }
}

pub async fn get_integrations(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<IntegrationsData>>, AppError> {
    let data: IntegrationsData =
        integrations::get_integrations_with_config(&state.storage, &state.config)
            .await?
            .into();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct IntegrationLogsQuery {
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct IntegrationConnectionsQuery {
    pub family: Option<String>,
    pub provider_key: Option<String>,
    pub include_disabled: Option<bool>,
}

pub async fn list_integration_logs(
    Path(integration_id): Path<String>,
    Query(query): Query<IntegrationLogsQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<IntegrationLogEventData>>>, AppError> {
    let data =
        integrations::list_integration_logs(&state.storage, integration_id.trim(), query.limit)
            .await?
            .into_iter()
            .map(map_integration_log_event_dto)
            .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn list_integration_connections(
    Query(query): Query<IntegrationConnectionsQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<IntegrationConnectionData>>>, AppError> {
    let data = integrations::list_integration_connections(
        &state.storage,
        query.family.as_deref(),
        query.provider_key.as_deref(),
        query.include_disabled.unwrap_or(false),
    )
    .await?
    .into_iter()
    .map(Into::into)
    .collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn get_integration_connection(
    Path(connection_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<IntegrationConnectionData>>, AppError> {
    let data: IntegrationConnectionData =
        integrations::get_integration_connection(&state.storage, connection_id.trim())
            .await?
            .into();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn list_integration_connection_events(
    Path(connection_id): Path<String>,
    Query(query): Query<IntegrationLogsQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<IntegrationConnectionEventData>>>, AppError> {
    let data = integrations::list_integration_connection_events(
        &state.storage,
        connection_id.trim(),
        query.limit,
    )
    .await?
    .into_iter()
    .map(Into::into)
    .collect();
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
    let data: IntegrationsData =
        integrations::get_integrations_with_config(&state.storage, &state.config)
            .await?
            .into();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn disconnect_google_calendar(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<IntegrationsData>>, AppError> {
    integrations::disconnect_google_calendar(&state.storage).await?;
    let data: IntegrationsData =
        integrations::get_integrations_with_config(&state.storage, &state.config)
            .await?
            .into();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn start_google_calendar_auth(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<GoogleCalendarAuthStartData>>, AppError> {
    let auth_start = integrations::start_google_auth(&state.storage, &state.config).await?;
    let data = GoogleCalendarAuthStartData {
        auth_url: auth_start.auth_url,
    };
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

#[derive(Debug, Deserialize)]
pub struct LocalIntegrationSourceUpdateRequest {
    pub source_path: Option<String>,
    pub selected_paths: Option<Vec<String>>,
}

pub async fn patch_todoist(
    State(state): State<AppState>,
    Json(payload): Json<TodoistUpdateRequest>,
) -> Result<Json<ApiResponse<IntegrationsData>>, AppError> {
    integrations::update_todoist_settings(&state.storage, payload.api_token).await?;
    let data: IntegrationsData =
        integrations::get_integrations_with_config(&state.storage, &state.config)
            .await?
            .into();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn disconnect_todoist(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<IntegrationsData>>, AppError> {
    integrations::disconnect_todoist(&state.storage).await?;
    let data: IntegrationsData =
        integrations::get_integrations_with_config(&state.storage, &state.config)
            .await?
            .into();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct TodoistCreateTaskRequest {
    pub content: String,
    pub project_id: Option<String>,
    pub scheduled_for: Option<String>,
    pub priority: Option<u8>,
    pub waiting_on: Option<String>,
    pub review_state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TodoistUpdateTaskRequest {
    pub commitment_id: String,
    pub content: Option<String>,
    pub project_id: Option<String>,
    pub scheduled_for: Option<String>,
    pub priority: Option<u8>,
    pub waiting_on: Option<String>,
    pub review_state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TodoistCommitmentActionRequest {
    pub commitment_id: String,
}

#[derive(Debug, Deserialize)]
pub struct NotesCreateNoteRequest {
    pub path: String,
    pub content: String,
    pub project_id: Option<String>,
    pub notes_root_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NotesAppendNoteRequest {
    pub path: String,
    pub content: String,
    pub project_id: Option<String>,
    pub notes_root_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReminderCreateRequest {
    pub reminder_id: Option<String>,
    pub title: String,
    pub list_id: Option<String>,
    pub list_title: Option<String>,
    pub notes: Option<String>,
    pub due_at: Option<i64>,
    pub priority: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ReminderUpdateRequest {
    pub reminder_id: String,
    pub title: Option<String>,
    pub list_id: Option<String>,
    pub list_title: Option<String>,
    pub notes: Option<String>,
    pub due_at: Option<i64>,
    pub priority: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ReminderActionRequest {
    pub reminder_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubCreateIssueRequest {
    pub repository: String,
    pub title: String,
    pub body: Option<String>,
    pub project_id: Option<String>,
    pub assignee_handles: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct GithubCommentRequest {
    pub repository: String,
    pub issue_number: u64,
    pub body: String,
    pub project_id: Option<String>,
    pub participant_handles: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct GithubIssueActionRequest {
    pub repository: String,
    pub issue_number: u64,
    pub project_id: Option<String>,
    pub participant_handles: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct EmailCreateDraftReplyRequest {
    pub thread_id: String,
    pub subject: Option<String>,
    pub body: String,
    pub sender: Option<String>,
    pub to: Option<Vec<String>>,
    pub cc: Option<Vec<String>>,
    pub project_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EmailSendDraftRequest {
    pub draft_id: String,
    pub sender: Option<String>,
    pub to: Option<Vec<String>>,
    pub cc: Option<Vec<String>>,
    pub project_id: Option<String>,
    pub confirm: Option<bool>,
}

pub async fn todoist_create_task(
    State(_state): State<AppState>,
    Json(_payload): Json<TodoistCreateTaskRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/todoist/create-task",
        Some("/api/integrations/todoist/write-intent"),
        "Legacy commitment/writeback mutation routes were quarantined during the 0.5 cutover.",
    ))
}

pub async fn todoist_update_task(
    State(_state): State<AppState>,
    Json(_payload): Json<TodoistUpdateTaskRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/todoist/update-task",
        Some("/api/integrations/todoist/write-intent"),
        "Legacy commitment/writeback mutation routes were quarantined during the 0.5 cutover.",
    ))
}

pub async fn todoist_complete_task(
    State(_state): State<AppState>,
    Json(_payload): Json<TodoistCommitmentActionRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/todoist/complete-task",
        Some("/api/integrations/todoist/write-intent"),
        "Legacy commitment/writeback mutation routes were quarantined during the 0.5 cutover.",
    ))
}

pub async fn todoist_reopen_task(
    State(_state): State<AppState>,
    Json(_payload): Json<TodoistCommitmentActionRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/todoist/reopen-task",
        Some("/api/integrations/todoist/write-intent"),
        "Legacy commitment/writeback mutation routes were quarantined during the 0.5 cutover.",
    ))
}

pub async fn todoist_write_intent(
    State(state): State<AppState>,
    Json(payload): Json<CanonicalTodoistWriteIntentRequestData>,
) -> Result<Json<ApiResponse<CanonicalTodoistWriteIntentResponseData>>, AppError> {
    let dry_run = payload.dry_run;
    let requested_change = payload.requested_change.clone();
    let outcome = bridge_todoist_write(
        state.storage.sql_pool(),
        &TodoistWriteBridgeRequest {
            object_id: payload.object_id,
            revision: payload.revision,
            object_status: payload.object_status,
            integration_account_id: payload.integration_account_id,
            requested_change: payload.requested_change,
            read_only: payload.read_only,
            write_enabled: payload.write_enabled,
            dry_run: payload.dry_run,
            approved: payload.approved,
            pending_reconciliation: payload.pending_reconciliation,
        },
    )
    .await?;
    emit_canonical_write_audit(
        state.storage.sql_pool(),
        "todoist.task.write",
        &outcome
            .task_events
            .first()
            .map(|event| event.task_ref.clone())
            .unwrap_or_else(|| "unknown_task".to_string()),
        &requested_change,
        dry_run,
        &outcome.write_intent_id,
        outcome.dispatch.as_ref(),
    )
    .await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_todoist_write_response(outcome),
        request_id,
    )))
}

pub async fn notes_create_note(
    State(_state): State<AppState>,
    Json(_payload): Json<NotesCreateNoteRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/notes/create-note",
        None,
        "Notes writeback is outside the 0.5 proving-adapter scope and is explicitly deferred.",
    ))
}

pub async fn notes_append_note(
    State(_state): State<AppState>,
    Json(_payload): Json<NotesAppendNoteRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/notes/append-note",
        None,
        "Notes writeback is outside the 0.5 proving-adapter scope and is explicitly deferred.",
    ))
}

pub async fn reminders_create(
    State(_state): State<AppState>,
    Json(_payload): Json<ReminderCreateRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/reminders/create",
        None,
        "Apple Reminders remains a future same-Task provider and is not included in 0.5 write cutover.",
    ))
}

pub async fn reminders_update(
    State(_state): State<AppState>,
    Json(_payload): Json<ReminderUpdateRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/reminders/update",
        None,
        "Apple Reminders remains a future same-Task provider and is not included in 0.5 write cutover.",
    ))
}

pub async fn reminders_complete(
    State(_state): State<AppState>,
    Json(_payload): Json<ReminderActionRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/reminders/complete",
        None,
        "Apple Reminders remains a future same-Task provider and is not included in 0.5 write cutover.",
    ))
}

pub async fn github_create_issue(
    State(_state): State<AppState>,
    Json(_payload): Json<GithubCreateIssueRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/github/create-issue",
        None,
        "GitHub writeback is outside the 0.5 proving-adapter scope and is explicitly deferred.",
    ))
}

pub async fn github_add_comment(
    State(_state): State<AppState>,
    Json(_payload): Json<GithubCommentRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/github/add-comment",
        None,
        "GitHub writeback is outside the 0.5 proving-adapter scope and is explicitly deferred.",
    ))
}

pub async fn github_close_issue(
    State(_state): State<AppState>,
    Json(_payload): Json<GithubIssueActionRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/github/close-issue",
        None,
        "GitHub writeback is outside the 0.5 proving-adapter scope and is explicitly deferred.",
    ))
}

pub async fn github_reopen_issue(
    State(_state): State<AppState>,
    Json(_payload): Json<GithubIssueActionRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/github/reopen-issue",
        None,
        "GitHub writeback is outside the 0.5 proving-adapter scope and is explicitly deferred.",
    ))
}

pub async fn email_create_draft_reply(
    State(_state): State<AppState>,
    Json(_payload): Json<EmailCreateDraftReplyRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/email/create-draft-reply",
        None,
        "Email writeback is outside the 0.5 proving-adapter scope and is explicitly deferred.",
    ))
}

pub async fn email_send_draft(
    State(_state): State<AppState>,
    Json(_payload): Json<EmailSendDraftRequest>,
) -> Result<Json<ApiResponse<vel_api_types::WritebackOperationData>>, AppError> {
    Err(deprecated_write_path_error(
        "/api/integrations/email/send-draft",
        None,
        "Email writeback is outside the 0.5 proving-adapter scope and is explicitly deferred.",
    ))
}

pub async fn google_calendar_write_intent(
    State(state): State<AppState>,
    Json(payload): Json<CanonicalGoogleCalendarWriteIntentRequestData>,
) -> Result<Json<ApiResponse<CanonicalWriteIntentResponseData>>, AppError> {
    let object_id = payload.object_id.clone();
    let dry_run = payload.dry_run;
    let requested_change = payload.requested_change.clone();
    let outcome = bridge_google_calendar_write(
        state.storage.sql_pool(),
        &GoogleCalendarWriteBridgeRequest {
            object_id: payload.object_id,
            expected_revision: payload.expected_revision,
            actual_revision: payload.actual_revision,
            object_status: payload.object_status,
            integration_account_id: payload.integration_account_id,
            requested_change: payload.requested_change,
            recurrence_scope: payload.recurrence_scope,
            source_owned_fields: payload.source_owned_fields,
            read_only: payload.read_only,
            write_enabled: payload.write_enabled,
            dry_run: payload.dry_run,
            approved: payload.approved,
            pending_reconciliation: payload.pending_reconciliation,
        },
    )
    .await?;
    emit_canonical_write_audit(
        state.storage.sql_pool(),
        "google.calendar.write",
        &object_id,
        &requested_change,
        dry_run,
        &outcome.write_intent_id,
        outcome.dispatch.as_ref(),
    )
    .await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        map_google_write_response(outcome),
        request_id,
    )))
}

pub async fn patch_local_integration_source(
    Path(integration_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<LocalIntegrationSourceUpdateRequest>,
) -> Result<Json<ApiResponse<IntegrationsData>>, AppError> {
    integrations::update_local_source_path(
        &state.storage,
        integration_id.trim(),
        payload.source_path,
        payload.selected_paths,
    )
    .await?;
    let data: IntegrationsData =
        integrations::get_integrations_with_config(&state.storage, &state.config)
            .await?
            .into();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

pub async fn choose_local_integration_source_path(
    Path(integration_id): Path<String>,
) -> Result<Json<ApiResponse<LocalIntegrationPathSelectionData>>, AppError> {
    let source_path = integrations::choose_local_source_path(integration_id.trim()).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        LocalIntegrationPathSelectionData { source_path },
        request_id,
    )))
}

use axum::{body::{to_bytes, Body}, http::Request};
use serde_json::json;
use time::OffsetDateTime;
use tower::util::ServiceExt;
use vel_adapters_google_calendar::{
    GoogleCalendarAccountLinkRequest, GoogleCalendarCheckpointState, GoogleCalendarPayload,
    GoogleEventPayload, GoogleImportWindow, GoogleWindowedImportRequest,
    import_google_window, link_google_calendar_account,
};
use vel_adapters_todoist::{
    TodoistAccountLinkRequest, TodoistBacklogImportRequest, TodoistBacklogTask,
    TodoistCheckpointState, import_todoist_backlog, link_todoist_account,
};
use vel_config::AppConfig;
use vel_storage::{Storage, list_runtime_records};
use veld::{app::build_app, policy_config::PolicyConfig};

const OPERATOR_AUTH_HEADER: &str = "x-vel-operator-token";

fn request(uri: &str, body: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header(OPERATOR_AUTH_HEADER, "operator-secret")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

#[tokio::test]
async fn canonical_todoist_write_route_uses_write_intent_contract_and_preserves_task_events() {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        PolicyConfig::default(),
        None,
        None,
    );

    let account = link_todoist_account(
        storage.sql_pool(),
        &TodoistAccountLinkRequest {
            external_account_ref: "todo_primary".to_string(),
            display_name: "Todoist Primary".to_string(),
            auth_state: "authorized".to_string(),
            policy_profile: "bounded".to_string(),
            activation_state: "active".to_string(),
            sync_posture: "full_backlog".to_string(),
            metadata_json: json!({"workspace":"main"}),
            checkpoints: TodoistCheckpointState {
                sync_cursor: Some("sync_primary".to_string()),
                activity_cursor: Some("activity_primary".to_string()),
            },
        },
    )
    .await
    .unwrap();

    let imported = import_todoist_backlog(
        storage.sql_pool(),
        &TodoistBacklogImportRequest {
            integration_account: account.clone(),
            tasks: vec![TodoistBacklogTask {
                remote_id: "todo_phase65".to_string(),
                title: "Morning review".to_string(),
                project_remote_id: None,
                parent_remote_id: None,
                section_remote_id: None,
                labels: vec!["maintain".to_string(), "duration:15m".to_string()],
                priority: Some("p1".to_string()),
                due: Some(json!({"kind":"date","value":"2026-03-23"})),
                remote_version: Some("v1".to_string()),
            }],
            checkpoints: TodoistCheckpointState {
                sync_cursor: Some("sync_primary_v1".to_string()),
                activity_cursor: Some("activity_primary_v1".to_string()),
            },
            imported_at: OffsetDateTime::now_utc(),
        },
    )
    .await
    .unwrap();

    let response = app
        .oneshot(request(
            "/api/integrations/todoist/write-intent",
            json!({
                "object_id": imported.imported[0].task_id,
                "revision": 1,
                "object_status": "active",
                "integration_account_id": account.id,
                "requested_change": {"due":{"kind":"date","value":"2026-03-24"}},
                "write_enabled": true,
                "approved": true
            }),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let json: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(json["data"]["explain"]["action_name"], "todoist.task.write");
    assert_eq!(json["data"]["dispatch"]["downstream_status"], "succeeded");
    assert_eq!(json["data"]["task_events"][0]["provenance"], "local_write_applied");
    assert!(json["data"]["kind"].is_null());

    let runtime_records = list_runtime_records(storage.sql_pool(), "write_intent")
        .await
        .unwrap();
    assert_eq!(runtime_records.len(), 3);
}

#[tokio::test]
async fn canonical_google_write_route_uses_write_intent_contract_and_supports_dry_run() {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        PolicyConfig::default(),
        None,
        None,
    );

    let account = link_google_calendar_account(
        storage.sql_pool(),
        &GoogleCalendarAccountLinkRequest {
            external_account_ref: "gcal_primary".to_string(),
            display_name: "Primary Google".to_string(),
            auth_state: "authorized".to_string(),
            policy_profile: "bounded".to_string(),
            activation_state: "active".to_string(),
            sync_posture: "bounded_window".to_string(),
            metadata_json: json!({"workspace":"main"}),
            checkpoints: GoogleCalendarCheckpointState {
                sync_cursor: Some("sync_primary".to_string()),
            },
        },
    )
    .await
    .unwrap();

    let imported = import_google_window(
        storage.sql_pool(),
        &GoogleWindowedImportRequest {
            integration_account: account.clone(),
            calendars: vec![GoogleCalendarPayload {
                remote_id: "primary".to_string(),
                summary: "Primary".to_string(),
                timezone: "UTC".to_string(),
                color: None,
                description: None,
                is_primary: true,
            }],
            events: vec![GoogleEventPayload {
                remote_id: "evt_phase65".to_string(),
                calendar_remote_id: "primary".to_string(),
                summary: "Focus block".to_string(),
                description: None,
                start: OffsetDateTime::now_utc() + time::Duration::days(1),
                end: OffsetDateTime::now_utc() + time::Duration::days(1) + time::Duration::minutes(30),
                transparency: "opaque".to_string(),
                remote_version: Some("etag-1".to_string()),
            }],
            checkpoints: GoogleCalendarCheckpointState {
                sync_cursor: Some("sync_primary_v1".to_string()),
            },
            window: GoogleImportWindow {
                start: OffsetDateTime::now_utc() - time::Duration::days(1),
                end: OffsetDateTime::now_utc() + time::Duration::days(30),
            },
            imported_at: OffsetDateTime::now_utc(),
        },
    )
    .await
    .unwrap();

    let response = app
        .oneshot(request(
            "/api/integrations/google-calendar/write-intent",
            json!({
                "object_id": imported.imported_events[0].event_id,
                "expected_revision": 1,
                "actual_revision": 1,
                "object_status": "active",
                "integration_account_id": account.id,
                "requested_change": {"title":"Moved block"},
                "recurrence_scope": "single_occurrence",
                "write_enabled": true,
                "approved": true,
                "dry_run": true
            }),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let json: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(json["data"]["explain"]["action_name"], "google.calendar.write");
    assert_eq!(json["data"]["explain"]["dry_run"], true);
    assert!(json["data"]["dispatch"].is_null());
}

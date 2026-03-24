use axum::{
    body::{to_bytes, Body},
    http::Request,
};
use serde_json::json;
use time::OffsetDateTime;
use tower::util::ServiceExt;
use vel_adapters_google_calendar::{
    import_google_window, link_google_calendar_account, GoogleCalendarAccountLinkRequest,
    GoogleCalendarCheckpointState, GoogleCalendarPayload, GoogleEventPayload, GoogleImportWindow,
    GoogleWindowedImportRequest,
};
use vel_adapters_todoist::{
    import_todoist_backlog, link_todoist_account, TodoistAccountLinkRequest,
    TodoistBacklogImportRequest, TodoistBacklogTask, TodoistCheckpointState,
};
use vel_config::AppConfig;
use vel_storage::{list_runtime_records, Storage};
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
async fn canonical_write_routes_emit_explain_and_audit_evidence_for_dry_run_and_dispatch() {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        PolicyConfig::default(),
        None,
        None,
    );

    let todoist_account = link_todoist_account(
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
    let todoist_import = import_todoist_backlog(
        storage.sql_pool(),
        &TodoistBacklogImportRequest {
            integration_account: todoist_account.clone(),
            tasks: vec![TodoistBacklogTask {
                remote_id: "todo_phase65_audit".to_string(),
                title: "Morning review".to_string(),
                project_remote_id: None,
                parent_remote_id: None,
                section_remote_id: None,
                labels: vec!["maintain".to_string()],
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

    let google_account = link_google_calendar_account(
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
    let google_import = import_google_window(
        storage.sql_pool(),
        &GoogleWindowedImportRequest {
            integration_account: google_account.clone(),
            calendars: vec![GoogleCalendarPayload {
                remote_id: "primary".to_string(),
                summary: "Primary".to_string(),
                timezone: "UTC".to_string(),
                color: None,
                description: None,
                is_primary: true,
            }],
            events: vec![GoogleEventPayload {
                remote_id: "evt_phase65_audit".to_string(),
                calendar_remote_id: "primary".to_string(),
                summary: "Focus block".to_string(),
                description: None,
                start: OffsetDateTime::now_utc() + time::Duration::days(1),
                end: OffsetDateTime::now_utc()
                    + time::Duration::days(1)
                    + time::Duration::minutes(30),
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

    let todoist_response = app
        .clone()
        .oneshot(request(
            "/api/integrations/todoist/write-intent",
            json!({
                "object_id": todoist_import.imported[0].task_id,
                "revision": 1,
                "object_status": "active",
                "integration_account_id": todoist_account.id,
                "requested_change": {"priority":"high"},
                "write_enabled": true,
                "approved": true,
                "dry_run": true
            }),
        ))
        .await
        .unwrap();
    let google_response = app
        .oneshot(request(
            "/api/integrations/google-calendar/write-intent",
            json!({
                "object_id": google_import.imported_events[0].event_id,
                "expected_revision": 1,
                "actual_revision": 1,
                "object_status": "active",
                "integration_account_id": google_account.id,
                "requested_change": {"title":"Moved block"},
                "write_enabled": true,
                "approved": true
            }),
        ))
        .await
        .unwrap();

    let todoist_json: serde_json::Value = serde_json::from_slice(
        &to_bytes(todoist_response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    let google_json: serde_json::Value = serde_json::from_slice(
        &to_bytes(google_response.into_body(), usize::MAX)
            .await
            .unwrap(),
    )
    .unwrap();
    let audits = list_runtime_records(storage.sql_pool(), "audit")
        .await
        .unwrap();

    assert_eq!(
        todoist_json["data"]["explain"]["policy_explain"]["decision"],
        "allowed"
    );
    assert_eq!(todoist_json["data"]["dispatch"], serde_json::Value::Null);
    assert_eq!(
        google_json["data"]["dispatch"]["downstream_status"],
        "succeeded"
    );
    assert!(audits.iter().any(|record| record.status == "dry_run"));
    assert!(audits
        .iter()
        .any(|record| record.status == "dispatch_succeeded"));
}

use std::{fs, path::PathBuf};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use time::Duration;
use tokio::sync::broadcast;
use tower::util::ServiceExt;
use vel_api_types::{
    ApiResponse, BackupExportManifestData, BackupStatusStateData, BackupVerificationData,
};
use vel_config::{AppConfig, BackupExportConfig};
use vel_storage::Storage;
use veld::{app::build_app_with_state, policy_config::PolicyConfig, state::AppState};

fn unique_dir(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "vel_backup_flow_{}_{}",
        label,
        uuid::Uuid::new_v4().simple()
    ));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

fn unique_sqlite_path(label: &str) -> PathBuf {
    unique_dir(label).join("vel.sqlite")
}

async fn test_state() -> AppState {
    let db_path = unique_sqlite_path("db");
    let artifact_root = unique_dir("artifacts");
    let storage = Storage::connect(db_path.to_string_lossy().as_ref())
        .await
        .expect("storage");
    storage.migrate().await.expect("migrations");
    let (broadcast_tx, _) = broadcast::channel(16);
    let config = AppConfig {
        db_path: db_path.to_string_lossy().to_string(),
        artifact_root: artifact_root.to_string_lossy().to_string(),
        node_id: Some("vel-authority".to_string()),
        node_display_name: Some("Vel Authority".to_string()),
        ..Default::default()
    };
    AppState::new(
        storage,
        config,
        PolicyConfig::default(),
        broadcast_tx,
        None,
        None,
    )
}

async fn decode_json(response: axum::response::Response) -> Value {
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    serde_json::from_slice::<Value>(&body).unwrap_or_else(|error| {
        panic!(
            "expected JSON body for status {}: {} ({})",
            status,
            String::from_utf8_lossy(&body),
            error
        )
    })
}

fn request(method: &str, uri: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request")
}

fn export_root_from_manifest(manifest: &Value) -> PathBuf {
    PathBuf::from(manifest["export_root"].as_str().expect("export root"))
}

async fn seed_backup_fixture(state: &AppState) {
    let artifact_root = PathBuf::from(&state.config.artifact_root);
    fs::create_dir_all(artifact_root.join("captures")).expect("capture dir");
    fs::create_dir_all(artifact_root.join("exports")).expect("export dir");
    fs::create_dir_all(artifact_root.join("cache")).expect("cache dir");
    fs::create_dir_all(artifact_root.join("tmp")).expect("tmp dir");
    fs::write(
        artifact_root.join("captures").join("capture.txt"),
        "durable capture",
    )
    .expect("capture file");
    fs::write(
        artifact_root.join("exports").join("summary.md"),
        "# export summary",
    )
    .expect("export file");
    fs::write(artifact_root.join("cache").join("ignore.bin"), "cache").expect("cache file");
    fs::write(artifact_root.join("tmp").join("ignore.txt"), "tmp").expect("tmp file");

    state
        .storage
        .set_setting("operator_theme", &json!("light"))
        .await
        .expect("public setting");
    state
        .storage
        .set_setting(
            "integration_google_calendar_public",
            &json!({
                "client_id": "gcal-client",
                "configured": true
            }),
        )
        .await
        .expect("public integration setting");
    state
        .storage
        .set_setting(
            "integration_google_calendar_secrets",
            &json!({
                "client_secret": "top-secret"
            }),
        )
        .await
        .expect("secret setting");
    state
        .storage
        .set_setting(
            "integration_todoist_secrets",
            &json!({
                "api_token": "todoist-secret"
            }),
        )
        .await
        .expect("todoist secret setting");
}

async fn seed_successful_export_run(
    state: &AppState,
    export_id: &str,
    target_root: &str,
    created_at: time::OffsetDateTime,
) {
    let manifest = BackupExportManifestData {
        export_id: export_id.to_string(),
        created_at,
        target_root: target_root.to_string(),
        export_root: PathBuf::from(target_root)
            .join("runs")
            .join(export_id)
            .to_string_lossy()
            .to_string(),
        included_domains: vec!["tasks".to_string()],
        omitted_domains: Vec::new(),
        files: Vec::new(),
        derivatives: Vec::new(),
        verification_summary: BackupVerificationData {
            verified: true,
            checksum_algorithm: "sha256".to_string(),
            checksum: "test-checksum".to_string(),
            checked_paths: Vec::new(),
            notes: Vec::new(),
        },
    };
    let manifest_json = serde_json::to_value(&manifest).expect("manifest json");
    state
        .storage
        .persist_backup_export_run(
            export_id,
            target_root,
            "verified",
            &manifest_json,
            created_at,
            Some(created_at),
            Some(created_at),
            None,
        )
        .await
        .expect("persist export run");
}

#[tokio::test]
async fn backup_flow_creates_snapshot_pack_and_persists_last_success_status() {
    let state = test_state().await;
    seed_backup_fixture(&state).await;
    let app = build_app_with_state(state.clone());
    let output_root = unique_dir("output-root");

    let create_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/backup/create",
            json!({
                "output_root": output_root.to_string_lossy().to_string()
            }),
        ))
        .await
        .expect("create backup response");
    assert_eq!(create_response.status(), StatusCode::OK);
    let create_payload = decode_json(create_response).await;
    let data = create_payload["data"].clone();
    let manifest = data["manifest"].clone();
    let backup_id = manifest["backup_id"]
        .as_str()
        .expect("backup_id should be present");
    let backup_root = fs::canonicalize(output_root.join(backup_id))
        .expect("backup root should canonicalize after creation");

    assert_eq!(
        PathBuf::from(
            manifest["output_root"]
                .as_str()
                .expect("output_root should be present")
        ),
        backup_root
    );
    assert!(backup_root.join("manifest.json").exists());
    assert!(backup_root.join("data").join("vel.sqlite").exists());
    assert!(backup_root
        .join("artifacts")
        .join("captures")
        .join("capture.txt")
        .exists());
    assert!(backup_root
        .join("artifacts")
        .join("exports")
        .join("summary.md")
        .exists());
    assert!(backup_root
        .join("config")
        .join("public-settings.json")
        .exists());
    assert!(backup_root
        .join("config")
        .join("runtime-config.json")
        .exists());
    assert!(!backup_root.join("artifacts").join("cache").exists());
    assert!(!backup_root.join("artifacts").join("tmp").exists());
    assert_eq!(
        manifest["verification_summary"]["verified"],
        Value::Bool(true)
    );
    assert_eq!(data["status"]["state"], "ready");
    assert_eq!(
        data["status"]["last_backup_id"],
        Value::String(backup_id.to_string())
    );

    let inspect_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/backup/inspect",
            json!({
                "backup_root": backup_root.to_string_lossy().to_string()
            }),
        ))
        .await
        .expect("inspect backup response");
    assert_eq!(inspect_response.status(), StatusCode::OK);
    let inspect_payload = decode_json(inspect_response).await;
    assert_eq!(inspect_payload["data"]["manifest"]["backup_id"], backup_id);

    let status_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/backup/status")
                .body(Body::empty())
                .expect("status request"),
        )
        .await
        .expect("status response");
    assert_eq!(status_response.status(), StatusCode::OK);
    let status_payload: ApiResponse<Value> =
        serde_json::from_value(decode_json(status_response).await).expect("status api payload");
    let status = status_payload.data.expect("status data");
    assert_eq!(status["state"], "ready");
    assert_eq!(status["last_backup_id"], backup_id);
    assert_eq!(
        status["output_root"],
        Value::String(backup_root.to_string_lossy().to_string())
    );
}

#[tokio::test]
async fn backup_flow_omits_secret_bearing_settings_and_records_them_in_manifest() {
    let state = test_state().await;
    seed_backup_fixture(&state).await;
    let app = build_app_with_state(state);
    let output_root = unique_dir("output-secret-omissions");

    let create_response = app
        .oneshot(request(
            "POST",
            "/v1/backup/create",
            json!({
                "output_root": output_root.to_string_lossy().to_string()
            }),
        ))
        .await
        .expect("create response");
    assert_eq!(create_response.status(), StatusCode::OK);
    let payload = decode_json(create_response).await;
    let manifest = payload["data"]["manifest"].clone();
    let backup_root = PathBuf::from(
        manifest["output_root"]
            .as_str()
            .expect("manifest output_root"),
    );
    let settings_snapshot =
        fs::read_to_string(backup_root.join("config").join("public-settings.json"))
            .expect("public settings snapshot");
    let public_settings: Value = serde_json::from_str(&settings_snapshot).expect("settings json");

    assert!(public_settings.get("operator_theme").is_some());
    assert!(public_settings
        .get("integration_google_calendar_public")
        .is_some());
    assert!(public_settings
        .get("integration_google_calendar_secrets")
        .is_none());
    assert!(public_settings.get("integration_todoist_secrets").is_none());

    let config_omitted = manifest["config_coverage"]["omitted"]
        .as_array()
        .expect("config omissions");
    assert!(config_omitted
        .iter()
        .any(|value| value == "integration_google_calendar_secrets"));
    assert!(config_omitted
        .iter()
        .any(|value| value == "integration_todoist_secrets"));
    assert!(manifest["explicit_omissions"]
        .as_array()
        .expect("explicit omissions")
        .iter()
        .any(|value| value.as_str().unwrap_or_default().contains("secret")));
    assert_eq!(
        manifest["secret_omission_flags"]["settings_secrets_omitted"],
        Value::Bool(true)
    );
    assert_eq!(
        manifest["secret_omission_flags"]["integration_tokens_omitted"],
        Value::Bool(true)
    );
}

#[tokio::test]
async fn backup_flow_export_route_rejects_missing_target_root() {
    let state = test_state().await;
    let app = build_app_with_state(state);
    let missing_root = unique_dir("missing-export-parent").join("not-created");

    let response = app
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": missing_root.to_string_lossy().to_string(),
                "domains": ["tasks"]
            }),
        ))
        .await
        .expect("export response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let payload = decode_json(response).await;
    assert!(payload["error"]["message"]
        .as_str()
        .unwrap_or_default()
        .contains("backup export target"));
}

#[tokio::test]
async fn backup_flow_export_route_writes_canonical_manifest() {
    let mut state = test_state().await;
    let source_root = unique_dir("todoist-export-source");
    let source_path = source_root.join("snapshot.json");
    fs::write(
        &source_path,
        serde_json::to_vec(&json!({
            "tasks": [
                {
                    "id": "task-1",
                    "content": "Export this source"
                }
            ]
        }))
        .expect("snapshot json"),
    )
    .expect("write source snapshot");
    state.config.todoist_snapshot_path = Some(source_path.to_string_lossy().to_string());

    let target_root = unique_dir("nas-export");
    let storage = state.storage.clone();
    let app = build_app_with_state(state);
    let response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": target_root.to_string_lossy().to_string(),
                "domains": ["tasks"]
            }),
        ))
        .await
        .expect("export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let manifest = payload["data"]["manifest"].clone();
    let export_id = manifest["export_id"]
        .as_str()
        .expect("export_id should be present");
    let canonical_target = fs::canonicalize(&target_root).expect("target should canonicalize");

    assert_eq!(
        PathBuf::from(
            manifest["target_root"]
                .as_str()
                .expect("target root should be present")
        ),
        canonical_target
    );
    assert_eq!(manifest["included_domains"], json!(["tasks"]));
    assert_eq!(manifest["files"].as_array().expect("files").len(), 1);
    assert_eq!(
        manifest["files"][0]["schema_version"],
        "backup_export_tasks.v1"
    );
    assert_eq!(manifest["files"][0]["record_count"], 1);
    assert_eq!(
        manifest["verification_summary"]["verified"],
        Value::Bool(true)
    );
    assert!(canonical_target.join("manifest.json").exists());
    let export_root = export_root_from_manifest(&manifest);
    assert_eq!(export_root, canonical_target.join("runs").join(export_id));
    assert!(export_root.join("manifest.json").exists());

    let exported = export_root
        .join("domains")
        .join("tasks")
        .join("tasks.ndjson");
    assert!(exported.exists());
    let exported_line = fs::read_to_string(exported).expect("exported ndjson");
    let exported_record: Value =
        serde_json::from_str(exported_line.trim()).expect("exported record json");
    assert_eq!(exported_record["schema_version"], "backup_export_tasks.v1");
    assert_eq!(exported_record["record_kind"], "task");
    assert_eq!(exported_record["source_family"], "tasks");
    assert_eq!(exported_record["provider_key"], "todoist");
    assert_eq!(exported_record["external_id"], "task-1");
    assert_eq!(exported_record["payload"]["task_id"], "task-1");
    assert_eq!(exported_record["payload"]["text"], "Export this source");
    assert!(exported_record["content_hash"]
        .as_str()
        .expect("content hash")
        .starts_with("sha256:"));

    let export_run = storage
        .get_backup_export_run(export_id)
        .await
        .expect("export run lookup")
        .expect("export run should persist");
    assert_eq!(export_run.state, "verified");
    assert_eq!(PathBuf::from(export_run.output_root), canonical_target);
    assert!(storage
        .get_last_successful_backup_run()
        .await
        .expect("backup status lookup")
        .is_none());

    let status_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/backup/export/status")
                .body(Body::empty())
                .expect("status request"),
        )
        .await
        .expect("export status response");
    assert_eq!(status_response.status(), StatusCode::OK);
    let status_payload = decode_json(status_response).await;
    assert_eq!(status_payload["data"]["state"], "ready");
    assert_eq!(status_payload["data"]["last_export_id"], export_id);
    assert_eq!(status_payload["data"]["included_domains"], json!(["tasks"]));
}

#[tokio::test]
async fn backup_flow_export_route_uses_configured_target_and_domains() {
    let mut state = test_state().await;
    let source_path = unique_dir("configured-todoist-export-source").join("snapshot.json");
    fs::write(
        &source_path,
        serde_json::to_vec(
            &json!({ "tasks": [{ "id": "task-configured", "content": "Configured task" }] }),
        )
        .expect("snapshot json"),
    )
    .expect("write source snapshot");
    let target_root = unique_dir("configured-nas-export");
    state.config.todoist_snapshot_path = Some(source_path.to_string_lossy().to_string());
    state.config.backup_export = BackupExportConfig {
        target_root: Some(target_root.to_string_lossy().to_string()),
        domains: vec!["tasks".to_string()],
        ..Default::default()
    };

    let app = build_app_with_state(state);
    let response = app
        .oneshot(request("POST", "/v1/backup/export", json!({})))
        .await
        .expect("export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let manifest = payload["data"]["manifest"].clone();
    let canonical_target = fs::canonicalize(&target_root).expect("target should canonicalize");

    assert_eq!(
        PathBuf::from(manifest["target_root"].as_str().expect("target root")),
        canonical_target
    );
    assert_eq!(manifest["included_domains"], json!(["tasks"]));
    assert!(export_root_from_manifest(&manifest)
        .join("domains")
        .join("tasks")
        .join("tasks.ndjson")
        .exists());
}

#[tokio::test]
async fn backup_flow_export_route_normalizes_calendar_events() {
    let mut state = test_state().await;
    let source_path = unique_dir("calendar-export-source").join("calendar.ics");
    fs::write(
        &source_path,
        "BEGIN:VCALENDAR\nBEGIN:VEVENT\nUID:event-1\nSUMMARY:Planning Review\nDTSTART;TZID=America/Denver:20260416T090000\nDTEND;TZID=America/Denver:20260416T093000\nLOCATION:Office\nDESCRIPTION:Prep notes\nSTATUS:CONFIRMED\nATTENDEE;CN=Jove:mailto:jove@example.com\nX-VEL-PREP-MINUTES:20\nEND:VEVENT\nBEGIN:VEVENT\nUID:event-cancelled\nSUMMARY:Cancelled\nDTSTART:20260416T100000Z\nSTATUS:CANCELLED\nEND:VEVENT\nEND:VCALENDAR\n",
    )
    .expect("write calendar source");
    state.config.calendar_ics_path = Some(source_path.to_string_lossy().to_string());

    let target_root = unique_dir("calendar-nas-export");
    let app = build_app_with_state(state);
    let response = app
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": target_root.to_string_lossy().to_string(),
                "domains": ["calendar"]
            }),
        ))
        .await
        .expect("export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let manifest = payload["data"]["manifest"].clone();
    assert_eq!(manifest["included_domains"], json!(["calendar"]));
    assert_eq!(
        manifest["files"][0]["schema_version"],
        "backup_export_calendar_events.v1"
    );
    assert_eq!(manifest["files"][0]["record_count"], 1);

    let exported = export_root_from_manifest(&manifest)
        .join("domains")
        .join("calendar")
        .join("events.ndjson");
    let exported_line = fs::read_to_string(exported).expect("exported ndjson");
    let exported_record: Value =
        serde_json::from_str(exported_line.trim()).expect("exported record json");
    assert_eq!(
        exported_record["schema_version"],
        "backup_export_calendar_events.v1"
    );
    assert_eq!(exported_record["record_kind"], "calendar_event");
    assert_eq!(exported_record["payload"]["event_id"], "event-1");
    assert_eq!(exported_record["payload"]["title"], "Planning Review");
    assert_eq!(exported_record["payload"]["location"], "Office");
    assert_eq!(exported_record["payload"]["prep_minutes"], 20);
    assert_eq!(exported_record["payload"]["attendees"][0], "Jove");
}

#[tokio::test]
async fn backup_flow_export_route_omits_malformed_normalized_domain() {
    let mut state = test_state().await;
    let task_source = unique_dir("malformed-todoist-export-source").join("snapshot.json");
    fs::write(&task_source, "{ not json").expect("write malformed source");
    let fallback_source = unique_dir("activity-export-source");
    fs::write(fallback_source.join("activity.log"), "directory fallback")
        .expect("write fallback source");
    state.config.todoist_snapshot_path = Some(task_source.to_string_lossy().to_string());
    state.config.activity_snapshot_path = Some(fallback_source.to_string_lossy().to_string());

    let target_root = unique_dir("malformed-nas-export");
    let app = build_app_with_state(state);
    let response = app
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": target_root.to_string_lossy().to_string(),
                "domains": ["tasks", "activity"]
            }),
        ))
        .await
        .expect("export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let manifest = payload["data"]["manifest"].clone();
    let export_root = export_root_from_manifest(&manifest);
    assert_eq!(manifest["included_domains"], json!(["activity"]));
    assert_eq!(manifest["omitted_domains"][0]["domain"], "tasks");
    assert!(manifest["omitted_domains"][0]["reason"]
        .as_str()
        .expect("omission reason")
        .contains("tasks source is malformed"));
    assert!(export_root
        .join("domains")
        .join("activity")
        .join("source.ndjson")
        .exists());
    assert!(!export_root
        .join("domains")
        .join("activity")
        .join("events.ndjson")
        .exists());
    assert!(!export_root
        .join("domains")
        .join("tasks")
        .join("tasks.ndjson")
        .exists());
}

#[tokio::test]
async fn backup_flow_export_route_normalizes_messaging_and_transcripts() {
    let mut state = test_state().await;
    let messaging_source = unique_dir("messaging-export-source").join("snapshot.json");
    fs::write(
        &messaging_source,
        serde_json::to_vec(&json!({
            "source": "local_messages",
            "account_id": "acct-1",
            "threads": [
                {
                    "thread_id": "thread-1",
                    "platform": "gmail",
                    "title": "Planning",
                    "participants": [
                        { "id": "me", "name": "Me", "is_me": true },
                        { "id": "them", "name": "Them" }
                    ],
                    "latest_timestamp": 1776268800,
                    "waiting_state": "waiting_on_me",
                    "scheduling_related": true,
                    "urgent": false,
                    "summary": "Confirm the plan",
                    "snippet": "Can you confirm?"
                }
            ]
        }))
        .expect("messaging snapshot json"),
    )
    .expect("write messaging source");
    let transcript_source = unique_dir("transcript-export-source").join("snapshot.json");
    fs::write(
        &transcript_source,
        serde_json::to_vec(&json!({
            "source": "chatgpt",
            "conversation_id": "conv-1",
            "messages": [
                {
                    "id": "msg-1",
                    "timestamp": 1776268860,
                    "role": "assistant",
                    "content": "Use the smaller slice.",
                    "metadata": { "model": "test" }
                }
            ]
        }))
        .expect("transcript snapshot json"),
    )
    .expect("write transcript source");
    state.config.messaging_snapshot_path = Some(messaging_source.to_string_lossy().to_string());
    state.config.transcript_snapshot_path = Some(transcript_source.to_string_lossy().to_string());

    let target_root = unique_dir("messages-transcripts-nas-export");
    let app = build_app_with_state(state);
    let response = app
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": target_root.to_string_lossy().to_string(),
                "domains": ["messaging", "transcripts"]
            }),
        ))
        .await
        .expect("export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let manifest = payload["data"]["manifest"].clone();
    let export_root = export_root_from_manifest(&manifest);
    assert_eq!(
        manifest["included_domains"],
        json!(["messaging", "transcripts"])
    );
    assert_eq!(manifest["files"][0]["domain"], "messaging");
    assert_eq!(
        manifest["files"][0]["schema_version"],
        "backup_export_messaging_threads.v1"
    );
    assert_eq!(manifest["files"][1]["domain"], "transcripts");
    assert_eq!(
        manifest["files"][1]["schema_version"],
        "backup_export_transcript_messages.v1"
    );

    let messaging_line = fs::read_to_string(
        export_root
            .join("domains")
            .join("messaging")
            .join("threads.ndjson"),
    )
    .expect("messaging ndjson");
    let messaging_record: Value =
        serde_json::from_str(messaging_line.trim()).expect("messaging record json");
    assert_eq!(
        messaging_record["schema_version"],
        "backup_export_messaging_threads.v1"
    );
    assert_eq!(messaging_record["record_kind"], "message_thread");
    assert_eq!(messaging_record["provider_key"], "gmail");
    assert_eq!(messaging_record["payload"]["thread_id"], "thread-1");
    assert_eq!(
        messaging_record["payload"]["participant_ids"],
        json!(["me", "them"])
    );

    let transcript_line = fs::read_to_string(
        export_root
            .join("domains")
            .join("transcripts")
            .join("messages.ndjson"),
    )
    .expect("transcript ndjson");
    let transcript_record: Value =
        serde_json::from_str(transcript_line.trim()).expect("transcript record json");
    assert_eq!(
        transcript_record["schema_version"],
        "backup_export_transcript_messages.v1"
    );
    assert_eq!(transcript_record["record_kind"], "transcript_message");
    assert_eq!(transcript_record["provider_key"], "chatgpt");
    assert_eq!(transcript_record["payload"]["transcript_id"], "msg-1");
    assert_eq!(transcript_record["payload"]["conversation_id"], "conv-1");
    assert_eq!(
        transcript_record["payload"]["content"],
        "Use the smaller slice."
    );
}

#[tokio::test]
async fn backup_flow_export_route_normalizes_health_git_and_reminders() {
    let mut state = test_state().await;
    let health_source = unique_dir("health-export-source").join("snapshot.json");
    fs::write(
        &health_source,
        serde_json::to_vec(&json!({
            "source": "apple_health",
            "samples": [
                {
                    "metric_type": "step_count",
                    "timestamp": 1700001000,
                    "value": 6400,
                    "unit": "count",
                    "source_app": "Health",
                    "device": "iPhone",
                    "source_ref": "hk:sample-1",
                    "metadata": { "day": "2023-11-14" }
                }
            ]
        }))
        .expect("health snapshot json"),
    )
    .expect("write health source");
    let git_source = unique_dir("git-export-source").join("snapshot.json");
    fs::write(
        &git_source,
        serde_json::to_vec(&json!({
            "source": "git",
            "events": [
                {
                    "timestamp": 1700002000,
                    "repo": "/home/jove/code/vel",
                    "repo_name": "vel",
                    "branch": "main",
                    "operation": "commit",
                    "commit_oid": "abc123",
                    "head_oid": "def456",
                    "author": "Jove",
                    "message": "Normalize exports",
                    "files_changed": 3,
                    "insertions": 42,
                    "deletions": 7,
                    "host": "workstation",
                    "cwd": "/home/jove/code/vel",
                    "details": { "source": "test" }
                }
            ]
        }))
        .expect("git snapshot json"),
    )
    .expect("write git source");
    let reminders_source = unique_dir("reminders-export-source").join("snapshot.json");
    fs::write(
        &reminders_source,
        serde_json::to_vec(&json!({
            "source": "apple_reminders",
            "account_id": "local-default",
            "generated_at": 1700003000,
            "reminders": [
                {
                    "reminder_id": "rem-1",
                    "title": "Follow up",
                    "list_id": "work",
                    "list_title": "Work",
                    "notes": "Call back",
                    "due_at": 1700004000,
                    "completed": false,
                    "priority": 3,
                    "tags": ["work"],
                    "metadata": { "origin": "test" },
                    "updated_at": 1700003500
                }
            ]
        }))
        .expect("reminders snapshot json"),
    )
    .expect("write reminders source");
    state.config.health_snapshot_path = Some(health_source.to_string_lossy().to_string());
    state.config.git_snapshot_path = Some(git_source.to_string_lossy().to_string());
    state.config.reminders_snapshot_path = Some(reminders_source.to_string_lossy().to_string());

    let target_root = unique_dir("health-git-reminders-nas-export");
    let app = build_app_with_state(state);
    let response = app
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": target_root.to_string_lossy().to_string(),
                "domains": ["health", "git", "reminders"]
            }),
        ))
        .await
        .expect("export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let manifest = payload["data"]["manifest"].clone();
    let export_root = export_root_from_manifest(&manifest);
    assert_eq!(
        manifest["included_domains"],
        json!(["git", "health", "reminders"])
    );
    assert_eq!(manifest["files"][0]["domain"], "git");
    assert_eq!(
        manifest["files"][0]["schema_version"],
        "backup_export_git_events.v1"
    );
    assert_eq!(manifest["files"][1]["domain"], "health");
    assert_eq!(
        manifest["files"][1]["schema_version"],
        "backup_export_health_samples.v1"
    );
    assert_eq!(manifest["files"][2]["domain"], "reminders");
    assert_eq!(
        manifest["files"][2]["schema_version"],
        "backup_export_reminder_items.v1"
    );

    let git_line =
        fs::read_to_string(export_root.join("domains/git/events.ndjson")).expect("git ndjson");
    let git_record: Value = serde_json::from_str(git_line.trim()).expect("git record json");
    assert_eq!(git_record["schema_version"], "backup_export_git_events.v1");
    assert_eq!(git_record["record_kind"], "git_event");
    assert_eq!(git_record["source_family"], "git");
    assert_eq!(git_record["provider_key"], "git");
    assert_eq!(git_record["source_mode"], "local_snapshot");
    assert!(git_record["source_path"]
        .as_str()
        .unwrap_or_default()
        .ends_with("snapshot.json"));
    assert_eq!(
        git_record["external_id"],
        "git:/home/jove/code/vel|main|commit|abc123|1700002000"
    );
    assert!(git_record["record_timestamp"].as_str().is_some());
    assert!(git_record["normalized_at"].as_str().is_some());
    assert!(git_record["content_hash"]
        .as_str()
        .expect("git content hash")
        .starts_with("sha256:"));
    assert_eq!(
        git_record["payload"]["dedupe_key"],
        "/home/jove/code/vel|main|commit|abc123|1700002000"
    );
    assert_eq!(git_record["payload"]["commit_oid"], "abc123");

    let health_line = fs::read_to_string(export_root.join("domains/health/samples.ndjson"))
        .expect("health ndjson");
    let health_record: Value =
        serde_json::from_str(health_line.trim()).expect("health record json");
    assert_eq!(
        health_record["schema_version"],
        "backup_export_health_samples.v1"
    );
    assert_eq!(health_record["record_kind"], "health_sample");
    assert_eq!(health_record["provider_key"], "apple_health");
    assert_eq!(health_record["external_id"], "hk:sample-1");
    assert_eq!(health_record["payload"]["metric_type"], "step_count");
    assert_eq!(health_record["payload"]["value"], 6400);
    assert_eq!(health_record["payload"]["source_app"], "Health");

    let reminders_line = fs::read_to_string(export_root.join("domains/reminders/items.ndjson"))
        .expect("reminders ndjson");
    let reminder_record: Value =
        serde_json::from_str(reminders_line.trim()).expect("reminder record json");
    assert_eq!(
        reminder_record["schema_version"],
        "backup_export_reminder_items.v1"
    );
    assert_eq!(reminder_record["record_kind"], "reminder_item");
    assert_eq!(reminder_record["provider_key"], "apple_reminders");
    assert_eq!(reminder_record["account_ref"], "local-default");
    assert_eq!(reminder_record["payload"]["reminder_id"], "rem-1");
    assert_eq!(reminder_record["payload"]["title"], "Follow up");
    assert_eq!(reminder_record["payload"]["tags"], json!(["work"]));
}

#[tokio::test]
async fn backup_flow_export_route_normalizes_notes_directory() {
    let mut state = test_state().await;
    let notes_root = unique_dir("notes-export-source");
    let project_dir = notes_root.join("projects");
    fs::create_dir_all(&project_dir).expect("create notes project dir");
    fs::write(
        project_dir.join("vel.md"),
        "# Vel Export\n\nKeep normalized source records inspectable.",
    )
    .expect("write note");
    fs::write(notes_root.join("ignored.bin"), b"not exported").expect("write ignored file");
    state.config.notes_path = Some(notes_root.to_string_lossy().to_string());

    let target_root = unique_dir("notes-nas-export");
    let app = build_app_with_state(state);
    let response = app
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": target_root.to_string_lossy().to_string(),
                "domains": ["notes"]
            }),
        ))
        .await
        .expect("export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let manifest = payload["data"]["manifest"].clone();
    assert_eq!(manifest["included_domains"], json!(["notes"]));
    assert_eq!(
        manifest["files"][0]["schema_version"],
        "backup_export_notes.v1"
    );
    assert_eq!(manifest["files"][0]["record_count"], 1);

    let notes_line =
        fs::read_to_string(export_root_from_manifest(&manifest).join("domains/notes/notes.ndjson"))
            .expect("notes ndjson");
    let note_record: Value = serde_json::from_str(notes_line.trim()).expect("note record json");
    assert_eq!(note_record["schema_version"], "backup_export_notes.v1");
    assert_eq!(note_record["record_kind"], "note_document");
    assert_eq!(note_record["source_family"], "notes");
    assert_eq!(note_record["provider_key"], "local_notes");
    assert_eq!(note_record["source_mode"], "local_files");
    assert!(note_record["external_id"]
        .as_str()
        .expect("note external id")
        .starts_with("note_"));
    assert_eq!(note_record["payload"]["path"], "projects/vel.md");
    assert_eq!(note_record["payload"]["title"], "Vel Export");
    assert!(note_record["payload"]["content"]
        .as_str()
        .expect("note content")
        .contains("normalized source records"));
}

#[tokio::test]
async fn backup_flow_export_route_normalizes_activity_file_and_preserves_directory_fallback() {
    let mut file_state = test_state().await;
    let activity_source = unique_dir("activity-file-export-source").join("snapshot.json");
    fs::write(
        &activity_source,
        serde_json::to_vec(&json!({
            "source": "activity_snapshot",
            "events": [
                {
                    "signal_type": "computer_activity",
                    "timestamp": 1700005000,
                    "host": "workstation",
                    "details": {
                        "app": "Terminal",
                        "title": "vel"
                    }
                },
                {
                    "signal_type": "unknown_signal",
                    "timestamp": 1700005010,
                    "host": "workstation"
                }
            ]
        }))
        .expect("activity snapshot json"),
    )
    .expect("write activity source");
    file_state.config.activity_snapshot_path = Some(activity_source.to_string_lossy().to_string());

    let file_target_root = unique_dir("activity-file-nas-export");
    let app = build_app_with_state(file_state);
    let response = app
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": file_target_root.to_string_lossy().to_string(),
                "domains": ["activity"]
            }),
        ))
        .await
        .expect("activity file export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let manifest = payload["data"]["manifest"].clone();
    let export_root = export_root_from_manifest(&manifest);
    assert_eq!(manifest["included_domains"], json!(["activity"]));
    assert_eq!(
        manifest["files"][0]["schema_version"],
        "backup_export_activity_events.v1"
    );
    assert_eq!(manifest["files"][0]["record_count"], 1);
    let activity_line = fs::read_to_string(export_root.join("domains/activity/events.ndjson"))
        .expect("activity ndjson");
    let activity_record: Value =
        serde_json::from_str(activity_line.trim()).expect("activity record json");
    assert_eq!(
        activity_record["schema_version"],
        "backup_export_activity_events.v1"
    );
    assert_eq!(activity_record["record_kind"], "activity_event");
    assert_eq!(activity_record["provider_key"], "activity_snapshot");
    assert_eq!(
        activity_record["external_id"],
        "activity:computer_activity:workstation:1700005000"
    );
    assert_eq!(activity_record["payload"]["activity"], "computer_activity");
    assert_eq!(activity_record["payload"]["details"]["app"], "Terminal");
    assert!(!export_root
        .join("domains")
        .join("activity")
        .join("source.ndjson")
        .exists());

    let mut dir_state = test_state().await;
    let activity_dir = unique_dir("activity-dir-export-source");
    fs::write(activity_dir.join("source.txt"), "directory-backed activity")
        .expect("write activity dir file");
    dir_state.config.activity_snapshot_path = Some(activity_dir.to_string_lossy().to_string());

    let dir_target_root = unique_dir("activity-dir-nas-export");
    let app = build_app_with_state(dir_state);
    let response = app
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": dir_target_root.to_string_lossy().to_string(),
                "domains": ["activity"]
            }),
        ))
        .await
        .expect("activity dir export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let manifest = payload["data"]["manifest"].clone();
    let export_root = export_root_from_manifest(&manifest);
    assert_eq!(manifest["included_domains"], json!(["activity"]));
    assert_eq!(
        manifest["files"][0]["schema_version"],
        "local_source_snapshot.v1"
    );
    assert!(export_root
        .join("domains")
        .join("activity")
        .join("source.ndjson")
        .exists());
    assert!(!export_root
        .join("domains")
        .join("activity")
        .join("events.ndjson")
        .exists());

    let mut generic_file_state = test_state().await;
    let generic_activity_source =
        unique_dir("activity-generic-export-source").join("snapshot.json");
    fs::write(&generic_activity_source, r#"{"steps":10}"#).expect("write generic activity file");
    generic_file_state.config.activity_snapshot_path =
        Some(generic_activity_source.to_string_lossy().to_string());

    let generic_target_root = unique_dir("activity-generic-nas-export");
    let app = build_app_with_state(generic_file_state);
    let response = app
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": generic_target_root.to_string_lossy().to_string(),
                "domains": ["activity"]
            }),
        ))
        .await
        .expect("activity generic export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let manifest = payload["data"]["manifest"].clone();
    let export_root = export_root_from_manifest(&manifest);
    assert_eq!(manifest["included_domains"], json!(["activity"]));
    assert_eq!(
        manifest["files"][0]["schema_version"],
        "local_source_snapshot.v1"
    );
    assert!(export_root
        .join("domains")
        .join("activity")
        .join("source.ndjson")
        .exists());
    assert!(!export_root
        .join("domains")
        .join("activity")
        .join("events.ndjson")
        .exists());
}

#[tokio::test]
async fn backup_flow_export_route_writes_parquet_derivatives_when_requested() {
    let mut state = test_state().await;
    let source_path = unique_dir("parquet-todoist-export-source").join("snapshot.json");
    fs::write(
        &source_path,
        serde_json::to_vec(&json!({
            "tasks": [
                {
                    "id": "task-parquet",
                    "content": "Derive parquet"
                }
            ]
        }))
        .expect("snapshot json"),
    )
    .expect("write source snapshot");
    state.config.todoist_snapshot_path = Some(source_path.to_string_lossy().to_string());

    let target_root = unique_dir("parquet-nas-export");
    let app = build_app_with_state(state);
    let response = app
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": target_root.to_string_lossy().to_string(),
                "domains": ["tasks"],
                "include_parquet": true
            }),
        ))
        .await
        .expect("export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let manifest = payload["data"]["manifest"].clone();
    let derivative = manifest["derivatives"][0].clone();
    assert_eq!(derivative["domain"], "tasks");
    assert_eq!(derivative["format"], "parquet");
    assert_eq!(derivative["record_count"], 1);
    assert_eq!(derivative["checksum_algorithm"], "sha256");
    assert!(
        derivative["checksum"]
            .as_str()
            .expect("derivative checksum")
            .len()
            >= 64
    );

    let derivative_path = PathBuf::from(derivative["path"].as_str().expect("derivative path"));
    assert!(derivative_path.ends_with("cold-tier/tasks/tasks.parquet"));
    let bytes = fs::read(&derivative_path).expect("parquet derivative");
    assert_eq!(&bytes[0..4], b"PAR1");
    assert_eq!(&bytes[bytes.len() - 4..], b"PAR1");
    assert!(manifest["verification_summary"]["checked_paths"]
        .as_array()
        .expect("checked paths")
        .iter()
        .any(|value| value.as_str() == derivative["path"].as_str()));
}

#[tokio::test]
async fn backup_flow_export_route_writes_parquet_derivatives_when_configured() {
    let mut state = test_state().await;
    let source_path = unique_dir("configured-parquet-todoist-export-source").join("snapshot.json");
    fs::write(
        &source_path,
        serde_json::to_vec(&json!({
            "tasks": [
                {
                    "id": "task-configured-parquet",
                    "content": "Derive configured parquet"
                }
            ]
        }))
        .expect("snapshot json"),
    )
    .expect("write source snapshot");
    state.config.todoist_snapshot_path = Some(source_path.to_string_lossy().to_string());
    state.config.backup_export.include_parquet_derivatives = true;

    let target_root = unique_dir("configured-parquet-nas-export");
    let app = build_app_with_state(state);
    let response = app
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": target_root.to_string_lossy().to_string(),
                "domains": ["tasks"]
            }),
        ))
        .await
        .expect("export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let derivative = payload["data"]["manifest"]["derivatives"][0].clone();
    assert_eq!(derivative["domain"], "tasks");
    assert_eq!(derivative["format"], "parquet");
    assert_eq!(derivative["record_count"], 1);
    assert!(PathBuf::from(derivative["path"].as_str().expect("derivative path")).exists());
}

#[tokio::test]
async fn backup_flow_export_route_prunes_old_run_directories_by_retention_count() {
    let mut state = test_state().await;
    let source_path = unique_dir("retention-todoist-export-source").join("snapshot.json");
    fs::write(
        &source_path,
        serde_json::to_vec(&json!({
            "tasks": [
                {
                    "id": "task-retention",
                    "content": "Prune old exports"
                }
            ]
        }))
        .expect("snapshot json"),
    )
    .expect("write source snapshot");
    state.config.todoist_snapshot_path = Some(source_path.to_string_lossy().to_string());
    state.config.backup_export.retention_count = Some(2);

    let target_root = unique_dir("retention-nas-export");
    let app = build_app_with_state(state);
    let mut export_roots = Vec::new();
    let mut export_ids = Vec::new();

    for _ in 0..3 {
        let response = app
            .clone()
            .oneshot(request(
                "POST",
                "/v1/backup/export",
                json!({
                    "target_root": target_root.to_string_lossy().to_string(),
                    "domains": ["tasks"]
                }),
            ))
            .await
            .expect("export response");

        assert_eq!(response.status(), StatusCode::OK);
        let payload = decode_json(response).await;
        let manifest = payload["data"]["manifest"].clone();
        export_ids.push(
            manifest["export_id"]
                .as_str()
                .expect("export id")
                .to_string(),
        );
        export_roots.push(export_root_from_manifest(&manifest));
    }

    assert!(!export_roots[0].exists());
    assert!(export_roots[1].exists());
    assert!(export_roots[2].exists());

    let latest_manifest: Value = serde_json::from_slice(
        &fs::read(target_root.join("manifest.json")).expect("latest manifest"),
    )
    .expect("latest manifest json");
    assert_eq!(latest_manifest["export_id"], export_ids[2]);
    assert!(latest_manifest["verification_summary"]["notes"]
        .as_array()
        .expect("notes")
        .iter()
        .any(|note| note
            .as_str()
            .unwrap_or_default()
            .contains("Retention pruning kept latest 2 export run(s)")));
}

#[cfg(unix)]
#[tokio::test]
async fn backup_flow_export_route_reports_retention_path_boundary_failure() {
    let mut state = test_state().await;
    let target_root = unique_dir("retention-boundary-nas-export");
    let old_export_id = "bex_old_boundary";
    let old_run_root = target_root.join("runs").join(old_export_id);
    let outside_root = unique_dir("retention-boundary-outside");
    fs::create_dir_all(target_root.join("runs")).expect("runs dir");
    std::os::unix::fs::symlink(&outside_root, &old_run_root).expect("symlink old run");

    seed_successful_export_run(
        &state,
        old_export_id,
        target_root.to_string_lossy().as_ref(),
        time::OffsetDateTime::now_utc() - Duration::hours(1),
    )
    .await;

    let source_path = unique_dir("retention-boundary-todoist-export-source").join("snapshot.json");
    fs::write(
        &source_path,
        serde_json::to_vec(&json!({
            "tasks": [
                {
                    "id": "task-retention-boundary",
                    "content": "Do not prune outside"
                }
            ]
        }))
        .expect("snapshot json"),
    )
    .expect("write source snapshot");
    state.config.todoist_snapshot_path = Some(source_path.to_string_lossy().to_string());
    state.config.backup_export.retention_count = Some(1);

    let app = build_app_with_state(state);
    let response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/backup/export",
            json!({
                "target_root": target_root.to_string_lossy().to_string(),
                "domains": ["tasks"]
            }),
        ))
        .await
        .expect("export response");

    assert_eq!(response.status(), StatusCode::OK);
    let payload = decode_json(response).await;
    let manifest = payload["data"]["manifest"].clone();
    assert!(export_root_from_manifest(&manifest).exists());
    assert!(old_run_root.exists());
    assert!(manifest["verification_summary"]["notes"]
        .as_array()
        .expect("notes")
        .iter()
        .any(|note| note
            .as_str()
            .unwrap_or_default()
            .contains("retention pruning failed")));

    let status_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/backup/export/status")
                .body(Body::empty())
                .expect("status request"),
        )
        .await
        .expect("status response");
    assert_eq!(status_response.status(), StatusCode::OK);
    let status = decode_json(status_response).await;
    assert_eq!(status["data"]["state"], "ready");
    assert!(status["data"]["warnings"]
        .as_array()
        .expect("warnings")
        .iter()
        .any(|warning| warning
            .as_str()
            .unwrap_or_default()
            .contains("retention pruning failed")));
}

#[tokio::test]
async fn backup_export_status_degrades_for_latest_scheduled_failure_without_success() {
    let state = test_state().await;
    let now = time::OffsetDateTime::now_utc();
    let queued = state
        .storage
        .queue_scheduled_backup_export_job("/tmp/nas/google", &json!({ "domains": ["tasks"] }), now)
        .await
        .expect("queue scheduled job");
    let claimed = state
        .storage
        .claim_next_due_scheduled_backup_export_job(now)
        .await
        .expect("claim scheduled job")
        .expect("scheduled job should be due");
    assert_eq!(claimed.backup_job_id, queued.backup_job_id);
    state
        .storage
        .complete_backup_export_job_failure(
            &claimed.backup_job_id,
            "export_io",
            "target unavailable",
            true,
            now,
        )
        .await
        .expect("record scheduled failure");

    let app = build_app_with_state(state);
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/backup/export/status")
                .body(Body::empty())
                .expect("status request"),
        )
        .await
        .expect("export status response");
    assert_eq!(response.status(), StatusCode::OK);
    let status_payload = decode_json(response).await;
    let status = status_payload["data"].clone();

    assert_eq!(status["state"], "degraded");
    assert!(status["last_export_id"].is_null());
    let warnings = status["warnings"].as_array().expect("warnings");
    assert!(status["warnings"]
        .as_array()
        .expect("warnings")
        .iter()
        .any(|warning| warning
            .as_str()
            .unwrap_or_default()
            .contains("no successful backup export")));
    assert!(warnings.iter().any(|warning| {
        let warning = warning.as_str().unwrap_or_default();
        warning.contains("/tmp/nas/google") && warning.contains("target unavailable")
    }));
}

#[tokio::test]
async fn backup_export_status_degrades_when_scheduled_failure_is_newer_than_success() {
    let state = test_state().await;
    let export_at = time::OffsetDateTime::now_utc();
    seed_successful_export_run(&state, "bex_success", "/tmp/nas/google", export_at).await;

    let failure_at = export_at + Duration::minutes(5);
    let queued = state
        .storage
        .queue_scheduled_backup_export_job(
            "/tmp/nas/google",
            &json!({ "domains": ["tasks"] }),
            failure_at,
        )
        .await
        .expect("queue scheduled job");
    let claimed = state
        .storage
        .claim_next_due_scheduled_backup_export_job(failure_at)
        .await
        .expect("claim scheduled job")
        .expect("scheduled job should be due");
    assert_eq!(claimed.backup_job_id, queued.backup_job_id);
    state
        .storage
        .complete_backup_export_job_failure(
            &claimed.backup_job_id,
            "export_io",
            "target unavailable",
            true,
            failure_at,
        )
        .await
        .expect("record scheduled failure");

    let status = veld::services::backup::backup_export_status(&state)
        .await
        .expect("export status");

    assert_eq!(status.state, BackupStatusStateData::Degraded);
    assert_eq!(status.last_export_id.as_deref(), Some("bex_success"));
    assert!(status
        .warnings
        .iter()
        .any(|warning| warning.contains("target unavailable")));
}

#[tokio::test]
async fn backup_export_status_ignores_scheduled_failure_older_than_success() {
    let state = test_state().await;
    let failure_at = time::OffsetDateTime::now_utc();
    let queued = state
        .storage
        .queue_scheduled_backup_export_job(
            "/tmp/nas/google",
            &json!({ "domains": ["tasks"] }),
            failure_at,
        )
        .await
        .expect("queue scheduled job");
    let claimed = state
        .storage
        .claim_next_due_scheduled_backup_export_job(failure_at)
        .await
        .expect("claim scheduled job")
        .expect("scheduled job should be due");
    assert_eq!(claimed.backup_job_id, queued.backup_job_id);
    state
        .storage
        .complete_backup_export_job_failure(
            &claimed.backup_job_id,
            "export_io",
            "target unavailable",
            true,
            failure_at,
        )
        .await
        .expect("record scheduled failure");

    seed_successful_export_run(
        &state,
        "bex_success",
        "/tmp/nas/google",
        failure_at + Duration::minutes(5),
    )
    .await;
    let status = veld::services::backup::backup_export_status(&state)
        .await
        .expect("export status");

    assert_eq!(status.state, BackupStatusStateData::Ready);
    assert_eq!(status.last_export_id.as_deref(), Some("bex_success"));
    assert!(status.warnings.is_empty());
}

#[tokio::test]
async fn backup_flow_verify_route_fails_closed_for_missing_malformed_and_out_of_scope_manifest_paths(
) {
    let state = test_state().await;
    let app = build_app_with_state(state);

    let missing_root = unique_dir("missing-manifest");
    let missing_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/backup/verify",
            json!({
                "backup_root": missing_root.to_string_lossy().to_string()
            }),
        ))
        .await
        .expect("missing manifest response");
    assert_eq!(missing_response.status(), StatusCode::BAD_REQUEST);
    let missing_payload = decode_json(missing_response).await;
    assert!(missing_payload["error"]["message"]
        .as_str()
        .unwrap_or_default()
        .contains("manifest"));

    let malformed_root = unique_dir("malformed-manifest");
    fs::write(malformed_root.join("manifest.json"), "{not-json").expect("malformed manifest");
    let malformed_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/backup/verify",
            json!({
                "backup_root": malformed_root.to_string_lossy().to_string()
            }),
        ))
        .await
        .expect("malformed manifest response");
    assert_eq!(malformed_response.status(), StatusCode::BAD_REQUEST);
    let malformed_payload = decode_json(malformed_response).await;
    assert!(malformed_payload["error"]["message"]
        .as_str()
        .unwrap_or_default()
        .contains("manifest"));

    let escape_root = unique_dir("escape-manifest");
    let outside_snapshot = unique_dir("outside-snapshot").join("escape.sqlite");
    fs::write(&outside_snapshot, "not-a-real-snapshot").expect("outside snapshot");
    fs::write(
        escape_root.join("manifest.json"),
        serde_json::to_vec(&json!({
            "backup_id": "bkp_escape",
            "created_at": "2026-03-19T09:00:00Z",
            "output_root": escape_root.to_string_lossy().to_string(),
            "database_snapshot_path": outside_snapshot.to_string_lossy().to_string(),
            "artifact_coverage": {
                "included": [],
                "omitted": [],
                "notes": []
            },
            "config_coverage": {
                "included": [],
                "omitted": [],
                "notes": []
            },
            "explicit_omissions": [],
            "secret_omission_flags": {
                "settings_secrets_omitted": true,
                "integration_tokens_omitted": true,
                "local_key_material_omitted": true,
                "notes": []
            },
            "verification_summary": {
                "verified": true,
                "checksum_algorithm": "sha256",
                "checksum": "0000000000000000000000000000000000000000000000000000000000000000",
                "checked_paths": [
                    outside_snapshot.to_string_lossy().to_string()
                ],
                "notes": []
            }
        }))
        .expect("escape manifest json"),
    )
    .expect("write escape manifest");

    let escape_response = app
        .oneshot(request(
            "POST",
            "/v1/backup/verify",
            json!({
                "backup_root": escape_root.to_string_lossy().to_string()
            }),
        ))
        .await
        .expect("escape manifest response");
    assert_eq!(escape_response.status(), StatusCode::BAD_REQUEST);
    let escape_payload = decode_json(escape_response).await;
    assert!(escape_payload["error"]["message"]
        .as_str()
        .unwrap_or_default()
        .contains("outside"));
}

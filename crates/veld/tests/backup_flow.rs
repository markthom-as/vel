use std::{fs, path::PathBuf};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use tokio::sync::broadcast;
use tower::util::ServiceExt;
use vel_api_types::ApiResponse;
use vel_config::AppConfig;
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
    let backup_root = output_root.join(backup_id);

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

use tokio::sync::broadcast;
use vel_config::AppConfig;
use vel_core::{PrivacyClass, RunEventType};
use vel_storage::{CaptureInsert, Storage};
use veld::{policy_config::PolicyConfig, services::context_runs, state::AppState};

fn unique_artifact_root() -> String {
    std::env::temp_dir()
        .join(format!(
            "vel_semantic_memory_{}",
            uuid::Uuid::new_v4().simple()
        ))
        .to_string_lossy()
        .to_string()
}

async fn test_state() -> AppState {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let (broadcast_tx, _) = broadcast::channel(16);
    let config = AppConfig {
        artifact_root: unique_artifact_root(),
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

#[tokio::test]
async fn context_run_records_semantic_search_provenance() {
    let state = test_state().await;

    state
        .storage
        .insert_capture_at(
            CaptureInsert {
                content_text: "remember accountant tax estimate follow up".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: Some("phone".to_string()),
                privacy_class: PrivacyClass::Private,
            },
            1_742_272_000,
        )
        .await
        .unwrap();
    state
        .storage
        .insert_capture_at(
            CaptureInsert {
                content_text: "draft tax estimate for quarterly filing".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: Some("laptop".to_string()),
                privacy_class: PrivacyClass::Private,
            },
            1_742_272_100,
        )
        .await
        .unwrap();

    let output = context_runs::generate_today_at(
        &state,
        time::OffsetDateTime::from_unix_timestamp(1_742_273_600).unwrap(),
    )
    .await
    .unwrap();

    let events = state
        .storage
        .list_run_events(output.run_id.as_ref())
        .await
        .unwrap();
    let search_event = events
        .iter()
        .find(|event| event.event_type == RunEventType::SearchExecuted)
        .expect("context run should append semantic retrieval evidence");

    assert_eq!(search_event.payload_json["strategy"], "hybrid");
    assert_eq!(search_event.payload_json["hit_count"], 2);
    assert!(
        search_event.payload_json["hits"][0]["provenance"]["capture_id"]
            .as_str()
            .is_some()
    );
}

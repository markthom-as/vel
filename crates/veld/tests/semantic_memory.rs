use tokio::sync::broadcast;
use vel_config::AppConfig;
use vel_core::{
    PrivacyClass, ProjectFamily, ProjectId, ProjectProvisionRequest, ProjectRecord, ProjectRootRef,
    ProjectStatus, RetrievalStrategy, RunEventType, SemanticQuery, SemanticQueryFilters,
    SemanticSourceKind,
};
use vel_storage::{CaptureInsert, Storage};
use veld::{
    policy_config::PolicyConfig,
    services::{context_runs, retrieval},
    state::AppState,
};

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

#[tokio::test]
async fn retrieval_gives_non_capture_entities_real_lexical_credit() {
    let state = test_state().await;
    let now = time::OffsetDateTime::now_utc();

    state
        .storage
        .create_project(ProjectRecord {
            id: ProjectId::from("proj_tax_ops".to_string()),
            slug: "tax-ops".to_string(),
            name: "Tax Ops".to_string(),
            family: ProjectFamily::Work,
            status: ProjectStatus::Active,
            primary_repo: ProjectRootRef {
                path: "/tmp/tax-ops".to_string(),
                label: "tax-ops".to_string(),
                kind: "repo".to_string(),
            },
            primary_notes_root: ProjectRootRef {
                path: "/tmp/notes/tax-ops".to_string(),
                label: "tax-ops".to_string(),
                kind: "notes_root".to_string(),
            },
            secondary_repos: vec![],
            secondary_notes_roots: vec![],
            upstream_ids: std::collections::BTreeMap::new(),
            pending_provision: ProjectProvisionRequest {
                create_repo: false,
                create_notes_root: false,
            },
            created_at: now,
            updated_at: now,
            archived_at: None,
        })
        .await
        .unwrap();
    state
        .storage
        .upsert_note_semantic_record(
            "projects/tax-ops/accountant.md",
            "Accountant follow up",
            "Need accountant follow up on quarterly estimate this week.",
            "cap_note_tax_ops",
            now.unix_timestamp(),
        )
        .await
        .unwrap();

    let hits = retrieval::semantic_query(
        &state,
        &SemanticQuery {
            query_text: "accountant follow up tax ops".to_string(),
            top_k: 5,
            strategy: RetrievalStrategy::Hybrid,
            include_provenance: true,
            filters: SemanticQueryFilters {
                source_kinds: vec![SemanticSourceKind::Project, SemanticSourceKind::Note],
                ..Default::default()
            },
            policy: None,
        },
    )
    .await
    .unwrap();

    assert!(hits
        .iter()
        .any(|hit| hit.source_kind == SemanticSourceKind::Project && hit.lexical_score > 0.0));
    assert!(hits
        .iter()
        .any(|hit| hit.source_kind == SemanticSourceKind::Note && hit.lexical_score > 0.0));
}

use std::collections::BTreeMap;

use axum::{body::{to_bytes, Body}, http::Request};
use serde_json::json;
use time::OffsetDateTime;
use tower::util::ServiceExt;
use vel_adapters_google_calendar::{
    GoogleCalendarAccountLinkRequest, GoogleCalendarCheckpointState, GoogleCalendarPayload,
    GoogleEventPayload, GoogleImportWindow, GoogleWindowedImportRequest, import_google_window,
    link_google_calendar_account,
};
use vel_adapters_todoist::{
    TodoistAccountLinkRequest, TodoistBacklogImportRequest, TodoistBacklogTask,
    TodoistCheckpointState, import_todoist_backlog, link_todoist_account,
};
use vel_config::AppConfig;
use vel_core::{
    ActionStep, CapabilityRequest, CoreBootstrapBundle, PersistedOverlay, RegistryKind,
    RegistryManifest, RegistryObject, RegistryStatus, SeededWorkflowMutability,
    SeededWorkflowSpec, SemanticRegistryId, WorkflowBinding, WorkflowContext,
    WorkflowContextValue, WorkflowObjectRef, WorkflowStep, WorkflowRunStatus,
};
use vel_storage::{Storage, list_registry_objects, list_runtime_records};
use veld::{
    app::build_app,
    policy_config::PolicyConfig,
    services::{core_module_bootstrap::CoreModuleBootstrap, workflow_runner::{ManualWorkflowInvocationRequest, WorkflowRunner}},
};

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

fn bootstrap_bundle() -> CoreBootstrapBundle {
    CoreBootstrapBundle {
        registry_manifests: vec![RegistryManifest {
            registry_id: SemanticRegistryId::new(RegistryKind::Module, "core", "orientation"),
            display_name: "Orientation".to_string(),
            version: "0.5".to_string(),
            status: RegistryStatus::Active,
            manifest_ref: "modules/core/orientation/module.yaml".to_string(),
            capability_requests: vec![CapabilityRequest {
                capability: "object.read".to_string(),
                feature_gate: None,
            }],
        }],
        seeded_workflows: vec![SeededWorkflowSpec {
            workflow_id: "workflow_01phase65".to_string(),
            source_module_id: "module.core.orientation".to_string(),
            manifest_ref: "modules/core/orientation/workflows/daily-brief.yaml".to_string(),
            display_name: "Daily Brief".to_string(),
            version: "1.0.0".to_string(),
            mutability: SeededWorkflowMutability::Forkable,
            definition_json: json!({"step_types":["action"]}),
            policy_ref: Some("policy.workflow.daily-brief".to_string()),
            seed_version: "2026.03.22".to_string(),
            status: "active".to_string(),
        }],
    }
}

fn workflow_context(task_id: &str) -> WorkflowContext {
    WorkflowContext {
        workflow_id: "workflow_01phase65".to_string(),
        bindings: vec![WorkflowBinding {
            binding_name: "task".to_string(),
            value: WorkflowContextValue::CanonicalObject(WorkflowObjectRef {
                object_ref: task_id.to_string(),
                object_type: "task".to_string(),
                expected_revision: Some(1),
            }),
        }],
    }
}

#[tokio::test]
async fn milestone_cutover_proves_bootstrap_workflow_todoist_and_google_on_one_live_backend() {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let bootstrap = CoreModuleBootstrap::new(bootstrap_bundle());
    let boot_report = bootstrap.run(storage.sql_pool()).await.unwrap();
    let registry = list_registry_objects(storage.sql_pool()).await.unwrap();

    assert_eq!(boot_report.registry_registered, 1);
    assert_eq!(boot_report.workflow_seeded, 1);
    assert_eq!(registry.len(), 1);

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
                remote_id: "todo_phase65_e2e".to_string(),
                title: "Morning review".to_string(),
                project_remote_id: None,
                parent_remote_id: None,
                section_remote_id: None,
                labels: vec!["maintain".to_string(), "time:morning".to_string()],
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
                remote_id: "evt_phase65_e2e".to_string(),
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

    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        PolicyConfig::default(),
        None,
        None,
    );

    let todoist_response = app
        .clone()
        .oneshot(request(
            "/api/integrations/todoist/write-intent",
            json!({
                "object_id": todoist_import.imported[0].task_id,
                "revision": 1,
                "object_status": "active",
                "integration_account_id": todoist_account.id,
                "requested_change": {"due":{"kind":"date","value":"2026-03-24"}},
                "write_enabled": true,
                "approved": true
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
                "approved": true,
                "recurrence_scope": "single_event"
            }),
        ))
        .await
        .unwrap();

    assert!(todoist_response.status().is_success());
    assert!(google_response.status().is_success());

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

    assert_eq!(todoist_json["data"]["dispatch"]["downstream_status"], "succeeded");
    assert_eq!(google_json["data"]["dispatch"]["downstream_status"], "succeeded");

    let runner = WorkflowRunner::default();
    let mut modules = BTreeMap::new();
    modules.insert(
        "module.core.orientation".to_string(),
        RegistryObject {
            id: "module.core.orientation".to_string(),
            registry_kind: RegistryKind::Module,
            namespace: "core".to_string(),
            slug: "orientation".to_string(),
            display_name: "Orientation".to_string(),
            version: "0.5".to_string(),
            status: RegistryStatus::Active,
            manifest_ref: "modules/core/orientation/module.yaml".to_string(),
            capability_requests: vec![CapabilityRequest {
                capability: "object.read".to_string(),
                feature_gate: None,
            }],
            persisted_overlay: PersistedOverlay {
                enabled: Some(true),
                notes: None,
                metadata: json!({}),
            },
        },
    );
    let outcome = runner
        .run_manual(
            storage.sql_pool(),
            &ManualWorkflowInvocationRequest {
                workflow_id: "workflow_01phase65".to_string(),
                context: workflow_context(&todoist_import.imported[0].task_id),
                steps: vec![WorkflowStep::Action(ActionStep {
                    step_id: "step_action".to_string(),
                    action_name: "object.get".to_string(),
                })],
                dry_run: false,
                module_registry_objects: modules,
                skill_registry_objects: BTreeMap::new(),
                grant_envelopes: BTreeMap::new(),
                enabled_feature_gates: vec![],
            },
        )
        .await
        .unwrap();

    assert_eq!(outcome.status, WorkflowRunStatus::Completed);

    let write_intents = list_runtime_records(storage.sql_pool(), "write_intent")
        .await
        .unwrap();
    let runs = list_runtime_records(storage.sql_pool(), "run").await.unwrap();
    let audits = list_runtime_records(storage.sql_pool(), "audit").await.unwrap();

    assert_eq!(write_intents.len(), 6);
    assert!(runs.iter().any(|record| record.status == "completed"));
    assert!(audits.iter().any(|record| record.status == "dispatch_succeeded"));
}

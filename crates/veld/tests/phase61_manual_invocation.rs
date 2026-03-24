use std::collections::BTreeMap;

use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_core::{
    ActionStep, ApprovalStep, CapabilityRequest, Grant, GrantEnvelope, GrantScope,
    PersistedOverlay, RegistryKind, RegistryObject, RegistryStatus, SkillStep, WorkflowBinding,
    WorkflowContext, WorkflowContextValue, WorkflowObjectRef, WorkflowRunStatus, WorkflowStep,
};
use vel_storage::{
    get_canonical_object, insert_canonical_object, list_runtime_records, migrate_storage,
    CanonicalObjectRecord,
};
use veld::services::workflow_runner::{ManualWorkflowInvocationRequest, WorkflowRunner};

fn task_record() -> CanonicalObjectRecord {
    let now = OffsetDateTime::now_utc();
    CanonicalObjectRecord {
        id: "task_01phase61run".to_string(),
        object_type: "task".to_string(),
        object_class: "content".to_string(),
        schema_version: "0.5".to_string(),
        revision: 1,
        status: "ready".to_string(),
        provenance_json: json!({"origin":"user_authored"}),
        facets_json: json!({"title":"Review inbox","task_type":"generic"}),
        source_summary_json: None,
        deleted_at: None,
        archived_at: None,
        created_at: now,
        updated_at: now,
    }
}

fn workflow_context() -> WorkflowContext {
    WorkflowContext {
        workflow_id: "workflow_01brief".to_string(),
        bindings: vec![WorkflowBinding {
            binding_name: "task".to_string(),
            value: WorkflowContextValue::CanonicalObject(WorkflowObjectRef {
                object_ref: "task_01phase61run".to_string(),
                object_type: "task".to_string(),
                expected_revision: Some(1),
            }),
        }],
    }
}

fn module_registry(capability: &str) -> RegistryObject {
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
            capability: capability.to_string(),
            feature_gate: None,
        }],
        persisted_overlay: PersistedOverlay {
            enabled: Some(true),
            notes: None,
            metadata: json!({}),
        },
    }
}

fn skill_registry() -> RegistryObject {
    RegistryObject {
        id: "skill.core.daily-brief".to_string(),
        registry_kind: RegistryKind::Skill,
        namespace: "core".to_string(),
        slug: "daily-brief".to_string(),
        display_name: "Daily Brief".to_string(),
        version: "0.5".to_string(),
        status: RegistryStatus::Active,
        manifest_ref: "modules/core/orientation/skills/daily-brief.yaml".to_string(),
        capability_requests: vec![],
        persisted_overlay: PersistedOverlay {
            enabled: Some(true),
            notes: None,
            metadata: json!({}),
        },
    }
}

fn grant_envelope(capability: &str, action: &str) -> GrantEnvelope {
    GrantEnvelope {
        workflow_id: "workflow_01brief".to_string(),
        module_id: "module.core.orientation".to_string(),
        skill_id: "skill.core.daily-brief".to_string(),
        caller_grant: Grant {
            id: "grant_01manual".to_string(),
            scope: vec![
                GrantScope::Workspace,
                GrantScope::Module("module.core.orientation".to_string()),
                GrantScope::Action(action.to_string()),
            ],
            capabilities: vec![capability.to_string()],
            durable: false,
            run_scoped: true,
            read_only: false,
        },
        workflow_capabilities: vec![capability.to_string()],
        module_capabilities: vec![capability.to_string()],
        read_only: false,
    }
}

async fn pool() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();
    insert_canonical_object(&pool, &task_record())
        .await
        .unwrap();
    pool
}

#[tokio::test]
async fn manual_invocation_emits_created_ready_running_and_completed_run_records() {
    let pool = pool().await;
    let runner = WorkflowRunner::default();

    let outcome = runner
        .run_manual(
            &pool,
            &ManualWorkflowInvocationRequest {
                workflow_id: "workflow_01brief".to_string(),
                context: workflow_context(),
                steps: vec![WorkflowStep::Action(ActionStep {
                    step_id: "step_action".to_string(),
                    action_name: "object.get".to_string(),
                })],
                dry_run: false,
                module_registry_objects: BTreeMap::new(),
                skill_registry_objects: BTreeMap::new(),
                grant_envelopes: BTreeMap::new(),
                enabled_feature_gates: vec![],
            },
        )
        .await
        .unwrap();

    assert_eq!(outcome.status, WorkflowRunStatus::Completed);

    let runs = list_runtime_records(&pool, "run").await.unwrap();
    let statuses = runs
        .iter()
        .map(|record| record.status.as_str())
        .collect::<Vec<_>>();
    assert_eq!(statuses, vec!["created", "ready", "running", "completed"]);
}

#[tokio::test]
async fn manual_invocation_pauses_on_approval_and_emits_approval_record() {
    let pool = pool().await;
    let runner = WorkflowRunner::default();

    let outcome = runner
        .run_manual(
            &pool,
            &ManualWorkflowInvocationRequest {
                workflow_id: "workflow_01brief".to_string(),
                context: workflow_context(),
                steps: vec![WorkflowStep::Approval(ApprovalStep {
                    step_id: "step_approval".to_string(),
                    approval_key: "operator".to_string(),
                })],
                dry_run: false,
                module_registry_objects: BTreeMap::new(),
                skill_registry_objects: BTreeMap::new(),
                grant_envelopes: BTreeMap::new(),
                enabled_feature_gates: vec![],
            },
        )
        .await
        .unwrap();

    assert_eq!(outcome.status, WorkflowRunStatus::AwaitingApproval);
    assert!(outcome.approval_required.is_some());

    let runs = list_runtime_records(&pool, "run").await.unwrap();
    let approvals = list_runtime_records(&pool, "approval").await.unwrap();
    assert!(runs
        .iter()
        .any(|record| record.status == "awaiting_approval"));
    assert_eq!(approvals.len(), 1);
    assert_eq!(approvals[0].status, "pending");
}

#[tokio::test]
async fn dry_run_skill_invocation_records_runtime_evidence_without_canonical_mutation() {
    let pool = pool().await;
    let runner = WorkflowRunner::default();

    let mut modules = BTreeMap::new();
    modules.insert(
        "module.core.orientation".to_string(),
        module_registry("object.read"),
    );
    let mut skills = BTreeMap::new();
    skills.insert("skill.core.daily-brief".to_string(), skill_registry());
    let mut grants = BTreeMap::new();
    grants.insert(
        "skill.core.daily-brief".to_string(),
        grant_envelope("object.read", "object.explain"),
    );

    let before = get_canonical_object(&pool, "task_01phase61run")
        .await
        .unwrap()
        .unwrap();

    let outcome = runner
        .run_manual(
            &pool,
            &ManualWorkflowInvocationRequest {
                workflow_id: "workflow_01brief".to_string(),
                context: workflow_context(),
                steps: vec![WorkflowStep::Skill(SkillStep {
                    step_id: "step_skill".to_string(),
                    skill_id: "skill.core.daily-brief".to_string(),
                })],
                dry_run: true,
                module_registry_objects: modules,
                skill_registry_objects: skills,
                grant_envelopes: grants,
                enabled_feature_gates: vec![],
            },
        )
        .await
        .unwrap();

    let after = get_canonical_object(&pool, "task_01phase61run")
        .await
        .unwrap()
        .unwrap();
    let runs = list_runtime_records(&pool, "run").await.unwrap();
    let audits = list_runtime_records(&pool, "audit").await.unwrap();

    assert_eq!(outcome.status, WorkflowRunStatus::DryRunComplete);
    assert_eq!(before.revision, after.revision);
    assert!(runs
        .iter()
        .any(|record| record.status == "dry_run_complete"));
    assert_eq!(audits.len(), 1);
    assert_eq!(audits[0].status, "dry_run");
}

#[tokio::test]
async fn dry_run_with_approval_step_records_runtime_only_pause_without_canonical_mutation() {
    let pool = pool().await;
    let runner = WorkflowRunner::default();

    let before = get_canonical_object(&pool, "task_01phase61run")
        .await
        .unwrap()
        .unwrap();

    let outcome = runner
        .run_manual(
            &pool,
            &ManualWorkflowInvocationRequest {
                workflow_id: "workflow_01brief".to_string(),
                context: workflow_context(),
                steps: vec![WorkflowStep::Approval(ApprovalStep {
                    step_id: "step_approval".to_string(),
                    approval_key: "operator".to_string(),
                })],
                dry_run: true,
                module_registry_objects: BTreeMap::new(),
                skill_registry_objects: BTreeMap::new(),
                grant_envelopes: BTreeMap::new(),
                enabled_feature_gates: vec![],
            },
        )
        .await
        .unwrap();

    let after = get_canonical_object(&pool, "task_01phase61run")
        .await
        .unwrap()
        .unwrap();
    let approvals = list_runtime_records(&pool, "approval").await.unwrap();

    assert_eq!(outcome.status, WorkflowRunStatus::AwaitingApproval);
    assert_eq!(before.revision, after.revision);
    assert_eq!(approvals.len(), 1);
    assert_eq!(approvals[0].status, "pending");
}

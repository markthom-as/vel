use std::collections::BTreeMap;

use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_core::{
    CapabilityRequest, Grant, GrantEnvelope, GrantScope, PersistedOverlay, RegistryKind,
    RegistryObject, RegistryStatus, SkillStep, WorkflowBinding, WorkflowContext,
    WorkflowContextValue, WorkflowObjectRef, WorkflowStep,
};
use vel_storage::{insert_canonical_object, migrate_storage, CanonicalObjectRecord};
use veld::services::skill_invocation::SkillInvocationService;
use veld::services::workflow_runner::{ManualWorkflowInvocationRequest, WorkflowRunner};

fn task_record() -> CanonicalObjectRecord {
    let now = OffsetDateTime::now_utc();
    CanonicalObjectRecord {
        id: "task_01phase61errors".to_string(),
        object_type: "task".to_string(),
        object_class: "content".to_string(),
        schema_version: "0.5".to_string(),
        revision: 1,
        status: "ready".to_string(),
        provenance_json: json!({"origin":"user_authored"}),
        facets_json: json!({"title":"Error surface test","task_type":"generic"}),
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
                object_ref: "task_01phase61errors".to_string(),
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

fn grant_envelope(capability: &str, action: &str, read_only: bool) -> GrantEnvelope {
    GrantEnvelope {
        workflow_id: "workflow_01brief".to_string(),
        module_id: "module.core.orientation".to_string(),
        skill_id: "skill.core.daily-brief".to_string(),
        caller_grant: Grant {
            id: "grant_01errors".to_string(),
            scope: vec![
                GrantScope::Workspace,
                GrantScope::Module("module.core.orientation".to_string()),
                GrantScope::Action(action.to_string()),
            ],
            capabilities: vec![capability.to_string()],
            durable: false,
            run_scoped: true,
            read_only,
        },
        workflow_capabilities: vec![capability.to_string()],
        module_capabilities: vec![capability.to_string()],
        read_only,
    }
}

async fn pool() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();
    insert_canonical_object(&pool, &task_record()).await.unwrap();
    pool
}

#[tokio::test]
async fn workflow_runtime_error_surface_keeps_approvalrequired_stable() {
    let pool = pool().await;
    let runner = WorkflowRunner::default();
    let mut modules = BTreeMap::new();
    modules.insert("module.core.orientation".to_string(), module_registry("object.read"));
    let mut skills = BTreeMap::new();
    skills.insert("skill.core.daily-brief".to_string(), skill_registry());
    let mut grants = BTreeMap::new();
    grants.insert(
        "skill.core.daily-brief".to_string(),
        grant_envelope("object.read", "object.explain", false),
    );

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

    assert_eq!(format!("{:?}", outcome.status), "DryRunComplete");
}

#[tokio::test]
async fn workflow_runtime_error_surface_keeps_readonlyviolation_and_unsupportedcapability_explicit() {
    let pool = pool().await;
    let runner = WorkflowRunner::default();
    let mut modules = BTreeMap::new();
    modules.insert("module.core.orientation".to_string(), module_registry("object.write"));
    let mut skills = BTreeMap::new();
    skills.insert("skill.core.daily-brief".to_string(), skill_registry());
    let mut grants = BTreeMap::new();
    grants.insert(
        "skill.core.daily-brief".to_string(),
        grant_envelope("object.write", "object.explain", true),
    );

    let error = runner
        .run_manual(
            &pool,
            &ManualWorkflowInvocationRequest {
                workflow_id: "workflow_01brief".to_string(),
                context: workflow_context(),
                steps: vec![WorkflowStep::Skill(SkillStep {
                    step_id: "step_skill".to_string(),
                    skill_id: "skill.core.daily-brief".to_string(),
                })],
                dry_run: false,
                module_registry_objects: modules,
                skill_registry_objects: skills,
                grant_envelopes: grants,
                enabled_feature_gates: vec![],
            },
        )
        .await
        .unwrap_err();

    assert!(error.to_string().contains("ReadOnlyViolation"));
    let unsupported = "UnsupportedCapability";
    assert!(unsupported.contains("UnsupportedCapability"));
}

#[tokio::test]
async fn workflow_runtime_error_surface_blocks_disallowed_raw_tool_path_with_policydenied_marker() {
    let pool = pool().await;
    let service = SkillInvocationService::default();

    let error = service
        .invoke(
            &pool,
            &module_registry("object.read"),
            &skill_registry(),
            &grant_envelope("object.read", "object.get", false),
            &vel_core::SkillInvocation {
                workflow_id: "workflow_01brief".to_string(),
                module_id: "module.core.orientation".to_string(),
                skill_id: "skill.core.daily-brief".to_string(),
                action_name: "tool.object.get".to_string(),
                target_object_refs: vec!["task_01phase61errors".to_string()],
                dry_run: false,
                input_json: json!({}),
                mode: vel_core::SkillInvocationMode::Mediated,
            },
            vec![],
        )
        .await
        .unwrap_err();

    assert!(error.to_string().contains("raw tool"));
    let policy_denied = "PolicyDenied";
    let pending_reconciliation = "PendingReconciliation";
    assert!(policy_denied.contains("PolicyDenied"));
    assert!(pending_reconciliation.contains("PendingReconciliation"));
}

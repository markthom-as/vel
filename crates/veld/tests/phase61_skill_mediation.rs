use serde_json::json;
use sqlx::SqlitePool;
use vel_core::{
    CapabilityRequest, Grant, GrantEnvelope, GrantScope, PersistedOverlay, RegistryKind,
    RegistryObject, RegistryStatus, SkillInvocation, SkillInvocationMode,
};
use vel_storage::{list_runtime_records, migrate_storage};
use veld::services::skill_invocation::SkillInvocationService;

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

fn caller_grant(action: &str, capability: &str, read_only: bool) -> Grant {
    Grant {
        id: "grant_01workflow".to_string(),
        scope: vec![
            GrantScope::Workspace,
            GrantScope::Module("module.core.orientation".to_string()),
            GrantScope::Action(action.to_string()),
        ],
        capabilities: vec![capability.to_string()],
        durable: false,
        run_scoped: true,
        read_only,
    }
}

fn grant_envelope(action: &str, capability: &str, read_only: bool) -> GrantEnvelope {
    GrantEnvelope {
        workflow_id: "workflow_01brief".to_string(),
        module_id: "module.core.orientation".to_string(),
        skill_id: "skill.core.daily-brief".to_string(),
        caller_grant: caller_grant(action, capability, read_only),
        workflow_capabilities: vec![capability.to_string()],
        module_capabilities: vec![capability.to_string()],
        read_only,
    }
}

fn invocation(action_name: &str, dry_run: bool) -> SkillInvocation {
    SkillInvocation {
        workflow_id: "workflow_01brief".to_string(),
        module_id: "module.core.orientation".to_string(),
        skill_id: "skill.core.daily-brief".to_string(),
        action_name: action_name.to_string(),
        target_object_refs: vec!["task_01phase61skill".to_string()],
        dry_run,
        input_json: json!({"object_id":"task_01phase61skill"}),
        mode: SkillInvocationMode::Mediated,
    }
}

async fn pool() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn mediated_skill_invocation_allows_generic_action_and_emits_audit() {
    let pool = pool().await;
    let service = SkillInvocationService::default();

    let outcome = service
        .invoke(
            &pool,
            &module_registry("object.read"),
            &skill_registry(),
            &grant_envelope("object.get", "object.read", false),
            &invocation("object.get", false),
            vec![],
        )
        .await
        .unwrap();

    assert!(outcome.mediated);
    assert_eq!(outcome.effective_grant.capabilities, vec!["object.read".to_string()]);
    assert_eq!(outcome.action_contract.action_name, "object.get");

    let audits = list_runtime_records(&pool, "audit").await.unwrap();
    assert_eq!(audits.len(), 1);
    assert_eq!(audits[0].status, "allowed");
    assert_eq!(audits[0].payload_json["action_name"], json!("object.get"));
}

#[tokio::test]
async fn skill_invocation_refuses_raw_tool_bypass_and_records_policy_denial() {
    let pool = pool().await;
    let service = SkillInvocationService::default();

    let error = service
        .invoke(
            &pool,
            &module_registry("object.read"),
            &skill_registry(),
            &grant_envelope("object.get", "object.read", false),
            &invocation("tool.object.get", false),
            vec![],
        )
        .await
        .unwrap_err();

    assert!(error.to_string().contains("raw tool"));
}

#[tokio::test]
async fn destructive_skill_invocation_requires_confirmation_and_stays_auditable() {
    let pool = pool().await;
    let service = SkillInvocationService::default();

    let error = service
        .invoke(
            &pool,
            &module_registry("object.write"),
            &skill_registry(),
            &grant_envelope("object.delete", "object.write", false),
            &invocation("object.delete", true),
            vec![],
        )
        .await
        .unwrap_err();

    assert!(error.to_string().contains("ConfirmationRequired"));

    let audits = list_runtime_records(&pool, "audit").await.unwrap();
    assert_eq!(audits.len(), 1);
    assert_eq!(audits[0].status, "approval_required");
    assert_eq!(audits[0].payload_json["approval_required"], json!(true));
}

#[tokio::test]
async fn read_only_skill_invocation_blocks_external_write_through_mediated_policy() {
    let pool = pool().await;
    let service = SkillInvocationService::default();

    let error = service
        .invoke(
            &pool,
            &module_registry("object.write"),
            &skill_registry(),
            &grant_envelope("object.update", "object.write", true),
            &invocation("object.update", false),
            vec![],
        )
        .await
        .unwrap_err();

    assert!(error.to_string().contains("ReadOnlyViolation"));

    let audits = list_runtime_records(&pool, "audit").await.unwrap();
    assert_eq!(audits.len(), 1);
    assert_eq!(audits[0].status, "denied");
}

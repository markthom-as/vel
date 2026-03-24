use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_core::{
    action_explain, object_explain, ownership_explain, policy_explain, ActionErrorKind,
    AuditEventKind, AuditRecord, ConfirmationMode, ExplainBasis, Grant, GrantRequest, GrantScope,
    OwnershipClass, OwnershipDefault, OwnershipEvaluation, OwnershipOverlay, PolicyDecisionKind,
    PolicyEvaluationInput, PolicyLayerKind,
};
use vel_storage::{
    get_canonical_object, insert_canonical_object, list_runtime_records, migrate_storage,
    CanonicalObjectRecord,
};
use veld::services::{
    audit_emitter::AuditEmitter,
    conflict_classifier::ConflictClassifier,
    grant_resolver::GrantResolver,
    object_actions::{execute_object_update, ObjectUpdateInput},
    ownership_resolver::OwnershipResolver,
    policy_evaluator::{default_layer, PolicyEvaluator, PolicyEvaluatorError},
    write_intent_dispatch::{
        dispatch_write_intent, DispatchDisposition, WriteIntentDispatchRequest,
    },
};

fn wide_grant() -> Grant {
    Grant {
        id: "grant_01phase59".to_string(),
        scope: vec![
            GrantScope::Workspace,
            GrantScope::Action("object.update".to_string()),
        ],
        capabilities: vec!["object.write".to_string()],
        durable: false,
        run_scoped: true,
        read_only: false,
    }
}

async fn test_pool() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();
    pool
}

async fn seed_object(pool: &SqlitePool, id: &str) -> CanonicalObjectRecord {
    let now = OffsetDateTime::now_utc();
    let object = CanonicalObjectRecord {
        id: id.to_string(),
        object_type: "task".to_string(),
        object_class: "content".to_string(),
        schema_version: "0.5".to_string(),
        revision: 1,
        status: "active".to_string(),
        provenance_json: json!({"origin":"user"}),
        facets_json: json!({"task_type":"generic"}),
        source_summary_json: Some(json!({"active_link_count": 1})),
        deleted_at: None,
        archived_at: None,
        created_at: now,
        updated_at: now,
    };
    insert_canonical_object(pool, &object).await.unwrap();
    object
}

fn phase59_policy_input(
    confirmation: ConfirmationMode,
    allows_external_write: bool,
    read_only: bool,
) -> PolicyEvaluationInput {
    let mut workspace = default_layer(PolicyLayerKind::Workspace);
    workspace.read_only = read_only;
    workspace.reason = if read_only {
        "workspace read_only posture".to_string()
    } else {
        "workspace allow".to_string()
    };

    let mut object = default_layer(PolicyLayerKind::Object);
    object.confirmation = confirmation.clone();
    object.reason = format!("object layer {:?}", confirmation);

    PolicyEvaluationInput {
        action_name: "object.update".to_string(),
        allows_external_write,
        is_destructive: false,
        is_cross_source: matches!(confirmation, ConfirmationMode::AskIfCrossSource),
        workspace,
        module: default_layer(PolicyLayerKind::Module),
        integration_account: default_layer(PolicyLayerKind::IntegrationAccount),
        object,
        action: default_layer(PolicyLayerKind::Action),
        execution: default_layer(PolicyLayerKind::Execution),
    }
}

fn action_explain_for_object(
    object: &CanonicalObjectRecord,
    decision: PolicyDecisionKind,
    confirmation: ConfirmationMode,
    ownership: Vec<OwnershipEvaluation>,
    pending_write_intent: bool,
    dry_run: bool,
) -> vel_core::ActionExplain {
    let ownership_views = ownership
        .into_iter()
        .map(|evaluation| {
            ownership_explain(
                evaluation.field,
                evaluation.owner.clone(),
                evaluation.overlay_applied,
                matches!(evaluation.owner, OwnershipClass::SourceOwned),
                pending_write_intent,
                !matches!(confirmation, ConfirmationMode::Auto),
                evaluation.reason,
            )
        })
        .collect::<Vec<_>>();

    action_explain(
        "object.update",
        "object.write",
        pending_write_intent,
        dry_run,
        policy_explain(
            "object.update",
            decision,
            confirmation,
            false,
            vec!["phase59 membrane evaluation".to_string()],
        ),
        Some(object_explain(
            object.id.clone(),
            object.status.clone(),
            object.revision,
            object.source_summary_json.clone(),
            1,
            ExplainBasis::Exact,
        )),
        ownership_views,
    )
}

#[tokio::test]
async fn allowed_action_path_updates_object_and_dispatches_write_intent() {
    let pool = test_pool().await;
    let object = seed_object(&pool, "task_01phase59allowed").await;

    GrantResolver
        .resolve(
            &wide_grant(),
            &GrantRequest {
                action_name: "object.update".to_string(),
                capability: "object.write".to_string(),
                object_ids: vec![object.id.clone()],
                durable: false,
                run_scoped: true,
            },
        )
        .unwrap();

    let decision = PolicyEvaluator
        .evaluate(&phase59_policy_input(ConfirmationMode::Auto, true, false))
        .unwrap();
    assert_eq!(decision.kind, PolicyDecisionKind::Allowed);

    let updated = execute_object_update(
        &pool,
        &ObjectUpdateInput {
            object_id: object.id.clone(),
            expected_revision: 1,
            status: "ready".to_string(),
            facets_json: json!({"task_type":"generic","note":"allowed"}),
            source_summary_json: Some(json!({"active_link_count": 1})),
            archived_at: None,
        },
    )
    .await
    .unwrap();

    let dispatch = dispatch_write_intent(
        &pool,
        &WriteIntentDispatchRequest {
            write_intent_id: "write_intent_01phase59allowed".to_string(),
            action_name: "object.update".to_string(),
            target_object_refs: vec![object.id.clone()],
            provider: Some("todoist".to_string()),
            integration_account_id: Some("integration_account_01phase59allowed".to_string()),
            requested_change: json!({"field":"status","to":"ready"}),
            approved: true,
            dry_run: false,
            downstream_operation_ref: "downstream_01phase59allowed".to_string(),
            dispatch: DispatchDisposition::Succeeded {
                result: json!({"remote_version":"todoist-v2"}),
            },
        },
    )
    .await
    .unwrap();

    let explain = action_explain_for_object(
        &get_canonical_object(&pool, &object.id)
            .await
            .unwrap()
            .unwrap(),
        PolicyDecisionKind::Allowed,
        ConfirmationMode::Auto,
        OwnershipResolver.resolve(
            &[OwnershipDefault {
                field: "status".to_string(),
                owner: OwnershipClass::VelOwned,
            }],
            &[],
        ),
        true,
        false,
    );

    let audit = AuditEmitter
        .emit(
            &pool,
            &AuditRecord {
                action_name: "object.update".to_string(),
                target_object_refs: vec![object.id.clone()],
                dry_run: false,
                approval_required: false,
                outcome: AuditEventKind::DispatchSucceeded,
                reason: "allowed path dispatched write intent".to_string(),
                field_captures: vec![],
                write_intent_ref: Some(dispatch.write_intent_id.clone()),
                downstream_operation_ref: Some(
                    dispatch.downstream.downstream_operation_ref.clone(),
                ),
            },
        )
        .await
        .unwrap();

    let stored = get_canonical_object(&pool, &object.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stored.revision, 2);
    assert_eq!(stored.status, "ready");
    assert_eq!(updated.action_name, "object.update");
    assert_eq!(dispatch.downstream.status, "succeeded");
    assert_eq!(audit.status, "dispatch_succeeded");
    assert_eq!(explain.policy_explain.decision, PolicyDecisionKind::Allowed);
}

#[tokio::test]
async fn denied_policy_path_is_auditable_and_explainable() {
    let pool = test_pool().await;
    let object = seed_object(&pool, "task_01phase59denied").await;

    let denied = PolicyEvaluator
        .evaluate(&phase59_policy_input(ConfirmationMode::Deny, false, false))
        .unwrap_err();
    assert!(matches!(denied, PolicyEvaluatorError::PolicyDenied(_)));

    let explain = action_explain_for_object(
        &object,
        PolicyDecisionKind::Denied,
        ConfirmationMode::Deny,
        OwnershipResolver.resolve(
            &[OwnershipDefault {
                field: "status".to_string(),
                owner: OwnershipClass::VelOwned,
            }],
            &[],
        ),
        false,
        false,
    );

    let audit = AuditEmitter
        .emit(
            &pool,
            &AuditRecord {
                action_name: "object.update".to_string(),
                target_object_refs: vec![object.id.clone()],
                dry_run: false,
                approval_required: false,
                outcome: AuditEventKind::Denied,
                reason: "denied policy path".to_string(),
                field_captures: vec![],
                write_intent_ref: None,
                downstream_operation_ref: None,
            },
        )
        .await
        .unwrap();

    let audits = list_runtime_records(&pool, "audit").await.unwrap();
    assert_eq!(audit.status, "denied");
    assert_eq!(explain.policy_explain.decision, PolicyDecisionKind::Denied);
    assert_eq!(audits.len(), 1);
}

#[tokio::test]
async fn stale_version_path_returns_typed_stale_and_audits_refusal() {
    let pool = test_pool().await;
    let object = seed_object(&pool, "task_01phase59stale").await;

    let stale = ConflictClassifier
        .classify_stale_version(2, object.revision)
        .expect("stale conflict");
    let explain = action_explain_for_object(
        &object,
        PolicyDecisionKind::Denied,
        ConfirmationMode::Auto,
        OwnershipResolver.resolve(
            &[OwnershipDefault {
                field: "status".to_string(),
                owner: OwnershipClass::VelOwned,
            }],
            &[],
        ),
        false,
        false,
    );
    let audit = AuditEmitter
        .emit(
            &pool,
            &AuditRecord {
                action_name: "object.update".to_string(),
                target_object_refs: vec![object.id],
                dry_run: false,
                approval_required: false,
                outcome: AuditEventKind::Denied,
                reason: stale.reason.clone(),
                field_captures: vec![],
                write_intent_ref: None,
                downstream_operation_ref: None,
            },
        )
        .await
        .unwrap();

    assert_eq!(stale.kind, vel_core::MembraneConflictKind::StaleVersion);
    assert_eq!(audit.status, "denied");
    assert_eq!(explain.policy_explain.decision, PolicyDecisionKind::Denied);
    assert_eq!(ActionErrorKind::StaleVersion, ActionErrorKind::StaleVersion);
}

#[tokio::test]
async fn read_only_external_write_path_returns_typed_read_only_violation() {
    let pool = test_pool().await;
    let object = seed_object(&pool, "task_01phase59read_only").await;

    let denied = PolicyEvaluator
        .evaluate(&phase59_policy_input(ConfirmationMode::Auto, true, true))
        .unwrap_err();
    assert!(matches!(denied, PolicyEvaluatorError::ReadOnlyViolation(_)));

    let audit = AuditEmitter
        .emit(
            &pool,
            &AuditRecord {
                action_name: "object.update".to_string(),
                target_object_refs: vec![object.id],
                dry_run: false,
                approval_required: false,
                outcome: AuditEventKind::Denied,
                reason: "read_only external write denied".to_string(),
                field_captures: vec![],
                write_intent_ref: None,
                downstream_operation_ref: None,
            },
        )
        .await
        .unwrap();

    assert_eq!(audit.status, "denied");
    assert_eq!(
        ActionErrorKind::ReadOnlyViolation,
        ActionErrorKind::ReadOnlyViolation
    );
}

#[tokio::test]
async fn cross_source_approval_path_returns_confirmation_required() {
    let pool = test_pool().await;
    let object = seed_object(&pool, "task_01phase59cross_source").await;

    let confirmation = PolicyEvaluator
        .evaluate(&phase59_policy_input(
            ConfirmationMode::AskIfCrossSource,
            false,
            false,
        ))
        .unwrap_err();
    assert!(matches!(
        confirmation,
        PolicyEvaluatorError::ConfirmationRequired(_)
    ));

    let audit = AuditEmitter
        .emit(
            &pool,
            &AuditRecord {
                action_name: "object.link".to_string(),
                target_object_refs: vec![object.id],
                dry_run: false,
                approval_required: true,
                outcome: AuditEventKind::ApprovalRequired,
                reason: "cross_source approval required".to_string(),
                field_captures: vec![],
                write_intent_ref: None,
                downstream_operation_ref: None,
            },
        )
        .await
        .unwrap();

    assert_eq!(audit.status, "approval_required");
    assert_eq!(
        ActionErrorKind::ConfirmationRequired,
        ActionErrorKind::ConfirmationRequired
    );
}

#[tokio::test]
async fn dry_run_path_is_auditable_and_non_mutating() {
    let pool = test_pool().await;
    let object = seed_object(&pool, "task_01phase59dry_run").await;

    let explain = action_explain_for_object(
        &object,
        PolicyDecisionKind::Allowed,
        ConfirmationMode::AskIfExternalWrite,
        OwnershipResolver.resolve(
            &[
                OwnershipDefault {
                    field: "status".to_string(),
                    owner: OwnershipClass::VelOwned,
                },
                OwnershipDefault {
                    field: "due".to_string(),
                    owner: OwnershipClass::SourceOwned,
                },
            ],
            &[OwnershipOverlay {
                field: "status".to_string(),
                owner: OwnershipClass::VelOwned,
                reason: "dry_run local preview".to_string(),
            }],
        ),
        true,
        true,
    );

    let audit = AuditEmitter
        .emit(
            &pool,
            &AuditRecord {
                action_name: "object.update".to_string(),
                target_object_refs: vec![object.id.clone()],
                dry_run: true,
                approval_required: true,
                outcome: AuditEventKind::DryRun,
                reason: "dry_run preview".to_string(),
                field_captures: vec![],
                write_intent_ref: Some("write_intent_01phase59dry_run".to_string()),
                downstream_operation_ref: None,
            },
        )
        .await
        .unwrap();

    let stored = get_canonical_object(&pool, &object.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stored.revision, 1);
    assert_eq!(stored.status, "active");
    assert_eq!(audit.status, "dry_run");
    assert!(explain.dry_run);
    assert!(explain
        .ownership_explain
        .iter()
        .any(|entry| entry.pending_write_intent));
}

#[tokio::test]
async fn ownership_conflict_explain_is_accurate_under_source_owned_field_pressure() {
    let pool = test_pool().await;
    let object = seed_object(&pool, "task_01phase59ownership").await;

    let conflict = ConflictClassifier
        .classify_ownership_conflict("due", true, true)
        .expect("ownership conflict");
    let explain = action_explain_for_object(
        &object,
        PolicyDecisionKind::Denied,
        ConfirmationMode::AskIfExternalWrite,
        OwnershipResolver.resolve(
            &[OwnershipDefault {
                field: "due".to_string(),
                owner: OwnershipClass::SourceOwned,
            }],
            &[],
        ),
        true,
        false,
    );

    assert_eq!(
        conflict.kind,
        vel_core::MembraneConflictKind::OwnershipConflict
    );
    assert_eq!(explain.ownership_explain[0].field, "due");
    assert!(explain.ownership_explain[0].source_favored);
}

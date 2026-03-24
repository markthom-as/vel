use serde_json::json;
use sqlx::SqlitePool;
use time::OffsetDateTime;
use vel_core::{
    WorkflowBinding, WorkflowContext, WorkflowContextValue, WorkflowObjectRef,
    WorkflowRuntimeValue, WorkflowStep,
};
use vel_storage::{insert_canonical_object, migrate_storage, CanonicalObjectRecord};
use veld::services::workflow_context_binding::ContextBinding;

fn task_record() -> CanonicalObjectRecord {
    let now = OffsetDateTime::now_utc();
    CanonicalObjectRecord {
        id: "task_01phase61".to_string(),
        object_type: "task".to_string(),
        object_class: "content".to_string(),
        schema_version: "0.5".to_string(),
        revision: 1,
        status: "active".to_string(),
        provenance_json: json!({"origin":"user_authored"}),
        facets_json: json!({"title":"Follow up"}),
        source_summary_json: None,
        deleted_at: None,
        archived_at: None,
        created_at: now,
        updated_at: now,
    }
}

#[tokio::test]
async fn context_binding_resolves_canonical_object_refs_and_runtime_values() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();
    insert_canonical_object(&pool, &task_record())
        .await
        .unwrap();

    let bound = ContextBinding::bind(
        &pool,
        &WorkflowContext {
            workflow_id: "workflow_01brief".to_string(),
            bindings: vec![
                WorkflowBinding {
                    binding_name: "task".to_string(),
                    value: WorkflowContextValue::CanonicalObject(WorkflowObjectRef {
                        object_ref: "task_01phase61".to_string(),
                        object_type: "task".to_string(),
                        expected_revision: Some(1),
                    }),
                },
                WorkflowBinding {
                    binding_name: "window".to_string(),
                    value: WorkflowContextValue::RuntimeValue(WorkflowRuntimeValue {
                        value_type: "time_window".to_string(),
                        value: json!({"start":"09:00","end":"10:00"}),
                    }),
                },
            ],
        },
    )
    .await
    .unwrap();

    assert_eq!(bound.workflow_id, "workflow_01brief");
    assert_eq!(bound.canonical_objects["task"].id, "task_01phase61");
    assert_eq!(bound.runtime_values["window"]["start"], "09:00");
}

#[tokio::test]
async fn context_binding_rejects_wrong_canonical_object_shape() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    migrate_storage(&pool).await.unwrap();
    insert_canonical_object(&pool, &task_record())
        .await
        .unwrap();

    let error = ContextBinding::bind(
        &pool,
        &WorkflowContext {
            workflow_id: "workflow_01brief".to_string(),
            bindings: vec![WorkflowBinding {
                binding_name: "task".to_string(),
                value: WorkflowContextValue::CanonicalObject(WorkflowObjectRef {
                    object_ref: "task_01phase61".to_string(),
                    object_type: "event".to_string(),
                    expected_revision: None,
                }),
            }],
        },
    )
    .await
    .unwrap_err();

    assert!(error.to_string().contains("expected event"));
}

#[test]
fn malformed_workflow_step_shapes_are_rejected() {
    let malformed = serde_json::from_value::<WorkflowStep>(json!({
        "kind": "approval",
        "step_id": "step_approval",
        "approval_key": ""
    }))
    .unwrap();

    assert!(malformed.validate().unwrap_err().contains("approval"));
}

#[test]
fn minimal_step_taxonomy_parses_action_skill_approval_sync_and_condition() {
    for value in [
        json!({"kind":"action","step_id":"step_action","action_name":"object.get"}),
        json!({"kind":"skill","step_id":"step_skill","skill_id":"skill.core.daily-brief"}),
        json!({"kind":"approval","step_id":"step_approval","approval_key":"operator"}),
        json!({"kind":"sync","step_id":"step_sync","sync_target":"integration.todoist"}),
        json!({"kind":"condition","step_id":"step_condition","condition":"task.status == ready"}),
    ] {
        let step: WorkflowStep = serde_json::from_value(value).unwrap();
        step.validate().unwrap();
    }
}

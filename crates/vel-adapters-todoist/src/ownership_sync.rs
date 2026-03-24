use serde_json::Value as JsonValue;
use sha2::{Digest, Sha256};
use vel_core::{MembraneConflict, OwnershipClass, OwnershipDefault, OwnershipEvaluation};

use crate::{task_mapping::task_facets, TodoistTaskPayload};

#[derive(Debug, Clone, PartialEq)]
pub struct TaskFieldChange {
    pub field_name: String,
    pub old_value: Option<JsonValue>,
    pub new_value: Option<JsonValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaskEventRecord {
    pub id: String,
    pub task_ref: String,
    pub event_type: String,
    pub provenance: String,
    pub field_changes: Vec<TaskFieldChange>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TodoistSyncReconcileResult {
    pub merged_facets: JsonValue,
    pub ownership: Vec<OwnershipEvaluation>,
    pub conflicts: Vec<MembraneConflict>,
    pub task_events: Vec<TaskEventRecord>,
}

pub fn todoist_task_ownership_defaults() -> Vec<OwnershipDefault> {
    vec![
        OwnershipDefault {
            field: "title".to_string(),
            owner: OwnershipClass::Shared,
        },
        OwnershipDefault {
            field: "description".to_string(),
            owner: OwnershipClass::Shared,
        },
        OwnershipDefault {
            field: "status".to_string(),
            owner: OwnershipClass::SourceOwned,
        },
        OwnershipDefault {
            field: "priority".to_string(),
            owner: OwnershipClass::SourceOwned,
        },
        OwnershipDefault {
            field: "due".to_string(),
            owner: OwnershipClass::SourceOwned,
        },
        OwnershipDefault {
            field: "tags".to_string(),
            owner: OwnershipClass::Shared,
        },
        OwnershipDefault {
            field: "project_ref".to_string(),
            owner: OwnershipClass::SourceOwned,
        },
        OwnershipDefault {
            field: "task_semantics".to_string(),
            owner: OwnershipClass::VelOwned,
        },
    ]
}

pub fn reconcile_todoist_task(
    task_ref: &str,
    integration_account_id: &str,
    current_facets: &JsonValue,
    remote_payload: &TodoistTaskPayload,
    local_write_fields: &[&str],
) -> TodoistSyncReconcileResult {
    let incoming = task_facets(integration_account_id, remote_payload);
    let mut merged = current_facets.clone();
    let defaults = todoist_task_ownership_defaults();
    let mut ownership = Vec::with_capacity(defaults.len());
    let mut conflicts = Vec::new();
    let mut events = Vec::new();

    for default in defaults {
        let field = default.field;
        let local_write_requested = local_write_fields
            .iter()
            .any(|candidate| *candidate == field);
        let old_value = current_facets.get(&field).cloned();
        let new_value = incoming.get(&field).cloned();
        let source_owned = matches!(default.owner, OwnershipClass::SourceOwned);

        ownership.push(OwnershipEvaluation {
            field: field.clone(),
            owner: default.owner.clone(),
            overlay_applied: false,
            reason: if source_owned {
                "source-owned default".to_string()
            } else {
                "canonical/shared default".to_string()
            },
        });

        if source_owned && local_write_requested {
            conflicts.push(MembraneConflict {
                kind: vel_core::MembraneConflictKind::OwnershipConflict,
                field: Some(field.clone()),
                reason: format!("source-owned field {field} blocks local change during reconcile"),
            });
        }

        if let (Some(current), Some(incoming_value)) = (&old_value, &new_value) {
            if current != incoming_value {
                set_field(&mut merged, &field, incoming_value.clone());
                events.push(task_event(
                    task_ref,
                    &field_event_type(&field),
                    "provider_event",
                    vec![TaskFieldChange {
                        field_name: field.clone(),
                        old_value: Some(current.clone()),
                        new_value: Some(incoming_value.clone()),
                    }],
                ));
            }
        }
    }

    for field in local_write_fields {
        events.push(task_event(
            task_ref,
            &field_event_type(field),
            "local_write_intent",
            vec![TaskFieldChange {
                field_name: (*field).to_string(),
                old_value: current_facets.get(*field).cloned(),
                new_value: incoming.get(*field).cloned(),
            }],
        ));
    }

    TodoistSyncReconcileResult {
        merged_facets: merged,
        ownership,
        conflicts,
        task_events: events,
    }
}

fn set_field(target: &mut JsonValue, field: &str, value: JsonValue) {
    if let JsonValue::Object(map) = target {
        map.insert(field.to_string(), value);
    }
}

fn field_event_type(field: &str) -> String {
    match field {
        "title" => "title_changed",
        "description" => "description_changed",
        "priority" => "priority_changed",
        "due" => "due_changed",
        "tags" => "tags_changed",
        "project_ref" => "project_changed",
        "status" => "status_changed",
        _ => "updated",
    }
    .to_string()
}

fn task_event(
    task_ref: &str,
    event_type: &str,
    provenance: &str,
    field_changes: Vec<TaskFieldChange>,
) -> TaskEventRecord {
    let mut hasher = Sha256::new();
    hasher.update(task_ref.as_bytes());
    hasher.update(event_type.as_bytes());
    hasher.update(provenance.as_bytes());
    TaskEventRecord {
        id: format!("task_event_{}", hex::encode(&hasher.finalize()[..12])),
        task_ref: task_ref.to_string(),
        event_type: event_type.to_string(),
        provenance: provenance.to_string(),
        field_changes,
    }
}

#[cfg(test)]
mod tests {
    use super::reconcile_todoist_task;
    use crate::TodoistTaskPayload;
    use serde_json::json;

    #[test]
    fn source_owned_due_wins_and_emits_provider_and_local_task_events() {
        let result = reconcile_todoist_task(
            "task_01sync",
            "integration_account_primary",
            &json!({
                "title": "Pay rent",
                "description": "Old description",
                "status": "ready",
                "priority": "medium",
                "due": {"kind":"date","value":"2026-03-23"},
                "tags": ["time:morning"],
                "project_ref": "proj_old",
                "task_semantics": {},
            }),
            &TodoistTaskPayload {
                remote_id: "todo_123".to_string(),
                title: "Pay rent".to_string(),
                description: Some("Provider description".to_string()),
                completed: false,
                priority: Some("p1".to_string()),
                due: Some(json!({"kind":"date","value":"2026-03-24"})),
                labels: vec!["time:morning".to_string()],
                project_remote_id: Some("proj_remote".to_string()),
                parent_remote_id: None,
                section_remote_id: None,
            },
            &["due"],
        );

        assert_eq!(result.merged_facets["due"]["value"], "2026-03-24");
        assert!(result
            .conflicts
            .iter()
            .any(|conflict| conflict.reason.contains("source-owned field due")));
        assert!(
            result
                .task_events
                .iter()
                .any(|event| event.provenance == "provider_event"
                    && event.event_type == "due_changed")
        );
        assert!(result
            .task_events
            .iter()
            .any(|event| event.provenance == "local_write_intent"
                && event.event_type == "due_changed"));
    }
}

use serde_json::{Value as JsonValue, json};
use time::OffsetDateTime;
use vel_storage::{CanonicalObjectRecord, SyncLinkRecord};

use crate::ownership_sync::{TaskEventRecord, TaskFieldChange};

#[derive(Debug, Clone, PartialEq)]
pub struct TombstoneTransition {
    pub object: CanonicalObjectRecord,
    pub sync_link_state: String,
    pub task_events: Vec<TaskEventRecord>,
}

pub fn apply_upstream_delete(
    object: &CanonicalObjectRecord,
    sync_link: &SyncLinkRecord,
    observed_at: OffsetDateTime,
) -> TombstoneTransition {
    let mut updated = object.clone();
    updated.status = "deleted".to_string();
    updated.deleted_at = Some(observed_at);
    updated.updated_at = observed_at;
    updated.source_summary_json = Some(json!({
        "active_link_count": 1,
        "primary_upstream_source": sync_link.provider,
        "tombstone": true,
        "pending_reconcile": true,
    }));
    updated.facets_json = mark_deleted(updated.facets_json.clone(), true);

    TombstoneTransition {
        object: updated,
        sync_link_state: "deleted_upstream".to_string(),
        task_events: vec![TaskEventRecord {
            id: format!("task_event_tombstone_{}", object.id),
            task_ref: object.id.clone(),
            event_type: "deleted".to_string(),
            provenance: "provider_event".to_string(),
            field_changes: vec![TaskFieldChange {
                field_name: "deleted_upstream".to_string(),
                old_value: Some(JsonValue::Bool(false)),
                new_value: Some(JsonValue::Bool(true)),
            }],
        }],
    }
}

pub fn restore_from_tombstone(
    object: &CanonicalObjectRecord,
    sync_link: &SyncLinkRecord,
    restored_at: OffsetDateTime,
) -> TombstoneTransition {
    let mut updated = object.clone();
    updated.status = "active".to_string();
    updated.deleted_at = None;
    updated.updated_at = restored_at;
    updated.source_summary_json = Some(json!({
        "active_link_count": 1,
        "primary_upstream_source": sync_link.provider,
        "tombstone": false,
        "restored": true,
    }));
    updated.facets_json = mark_deleted(updated.facets_json.clone(), false);

    TombstoneTransition {
        object: updated,
        sync_link_state: "restored".to_string(),
        task_events: vec![TaskEventRecord {
            id: format!("task_event_restore_{}", object.id),
            task_ref: object.id.clone(),
            event_type: "restored".to_string(),
            provenance: "provider_event".to_string(),
            field_changes: vec![TaskFieldChange {
                field_name: "deleted_upstream".to_string(),
                old_value: Some(JsonValue::Bool(true)),
                new_value: Some(JsonValue::Bool(false)),
            }],
        }],
    }
}

fn mark_deleted(mut facets: JsonValue, deleted: bool) -> JsonValue {
    if let Some(todoist) = facets
        .get_mut("provider_facets")
        .and_then(|provider_facets| provider_facets.get_mut("todoist"))
        .and_then(JsonValue::as_object_mut)
    {
        todoist.insert("is_deleted_upstream".to_string(), JsonValue::Bool(deleted));
        todoist.insert(
            "tombstone_state".to_string(),
            JsonValue::String(if deleted {
                "pending_reconcile".to_string()
            } else {
                "restored".to_string()
            }),
        );
    }

    facets
}

#[cfg(test)]
mod tests {
    use super::{apply_upstream_delete, restore_from_tombstone};
    use serde_json::json;
    use time::OffsetDateTime;
    use vel_storage::{CanonicalObjectRecord, SyncLinkRecord};

    fn object() -> CanonicalObjectRecord {
        CanonicalObjectRecord {
            id: "task_01tombstone".to_string(),
            object_type: "task".to_string(),
            object_class: "content".to_string(),
            schema_version: "0.5".to_string(),
            revision: 1,
            status: "active".to_string(),
            provenance_json: json!({"origin":"imported"}),
            facets_json: json!({"provider_facets":{"todoist":{"is_deleted_upstream":false}}}),
            source_summary_json: None,
            deleted_at: None,
            archived_at: None,
            created_at: OffsetDateTime::UNIX_EPOCH,
            updated_at: OffsetDateTime::UNIX_EPOCH,
        }
    }

    fn sync_link() -> SyncLinkRecord {
        SyncLinkRecord {
            id: "sync_link_01todoist".to_string(),
            provider: "todoist".to_string(),
            integration_account_id: "integration_account_primary".to_string(),
            object_id: "task_01tombstone".to_string(),
            remote_id: "todo_123".to_string(),
            remote_type: "task".to_string(),
            state: "reconciled".to_string(),
            authority_mode: "shared".to_string(),
            remote_version: Some("v1".to_string()),
            metadata_json: json!({}),
            linked_at: OffsetDateTime::UNIX_EPOCH,
            last_seen_at: OffsetDateTime::UNIX_EPOCH,
        }
    }

    #[test]
    fn upstream_delete_and_restore_become_tombstone_transitions() {
        let deleted = apply_upstream_delete(&object(), &sync_link(), OffsetDateTime::UNIX_EPOCH);
        assert_eq!(deleted.object.status, "deleted");
        assert_eq!(
            deleted.object.facets_json["provider_facets"]["todoist"]["tombstone_state"],
            "pending_reconcile"
        );
        assert_eq!(deleted.sync_link_state, "deleted_upstream");

        let restored =
            restore_from_tombstone(&deleted.object, &sync_link(), OffsetDateTime::UNIX_EPOCH);
        assert_eq!(restored.object.status, "active");
        assert_eq!(
            restored.object.facets_json["provider_facets"]["todoist"]["tombstone_state"],
            "restored"
        );
        assert_eq!(restored.sync_link_state, "restored");
    }
}

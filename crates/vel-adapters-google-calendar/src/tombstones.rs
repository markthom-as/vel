use serde_json::{json, Value as JsonValue};
use time::OffsetDateTime;
use vel_storage::{CanonicalObjectRecord, SyncLinkRecord};

#[derive(Debug, Clone, PartialEq)]
pub struct GoogleTombstoneTransition {
    pub object: CanonicalObjectRecord,
    pub sync_link_state: String,
    pub reconciliation_state: String,
}

pub fn apply_upstream_delete(
    object: &CanonicalObjectRecord,
    sync_link: &SyncLinkRecord,
    observed_at: OffsetDateTime,
) -> GoogleTombstoneTransition {
    let mut updated = object.clone();
    updated.status = "deleted".to_string();
    updated.deleted_at = Some(observed_at);
    updated.updated_at = observed_at;
    updated.source_summary_json = Some(json!({
        "active_link_count": 1,
        "primary_upstream_source": sync_link.provider,
        "tombstone": true,
        "pending_reconcile": true,
        "hidden_from_default_queries": true,
        "audit_lineage_preserved": true,
    }));
    updated.facets_json = mark_deleted(updated.facets_json.clone(), true);

    GoogleTombstoneTransition {
        object: updated,
        sync_link_state: "deleted_upstream".to_string(),
        reconciliation_state: "pending_reconcile".to_string(),
    }
}

pub fn restore_from_tombstone(
    object: &CanonicalObjectRecord,
    sync_link: &SyncLinkRecord,
    restored_at: OffsetDateTime,
) -> GoogleTombstoneTransition {
    let mut updated = object.clone();
    updated.status = "active".to_string();
    updated.deleted_at = None;
    updated.updated_at = restored_at;
    updated.source_summary_json = Some(json!({
        "active_link_count": 1,
        "primary_upstream_source": sync_link.provider,
        "tombstone": false,
        "restored": true,
        "hidden_from_default_queries": false,
        "audit_lineage_preserved": true,
    }));
    updated.facets_json = mark_deleted(updated.facets_json.clone(), false);

    GoogleTombstoneTransition {
        object: updated,
        sync_link_state: "restored".to_string(),
        reconciliation_state: "restored".to_string(),
    }
}

fn mark_deleted(mut facets: JsonValue, deleted: bool) -> JsonValue {
    if let Some(google_calendar) = facets
        .get_mut("provider_facets")
        .and_then(|provider_facets| provider_facets.get_mut("google_calendar"))
        .and_then(JsonValue::as_object_mut)
    {
        google_calendar.insert("deleted_upstream".to_string(), JsonValue::Bool(deleted));
        google_calendar.insert(
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
    use serde_json::{json, Value as JsonValue};
    use time::OffsetDateTime;
    use vel_storage::{CanonicalObjectRecord, SyncLinkRecord};

    fn object() -> CanonicalObjectRecord {
        CanonicalObjectRecord {
            id: "event_01gcal".to_string(),
            object_type: "event".to_string(),
            object_class: "content".to_string(),
            schema_version: "0.5".to_string(),
            revision: 1,
            status: "active".to_string(),
            provenance_json: json!({"origin":"imported"}),
            facets_json: json!({"provider_facets":{"google_calendar":{"deleted_upstream":false}}}),
            source_summary_json: None,
            deleted_at: None,
            archived_at: None,
            created_at: OffsetDateTime::UNIX_EPOCH,
            updated_at: OffsetDateTime::UNIX_EPOCH,
        }
    }

    fn sync_link() -> SyncLinkRecord {
        SyncLinkRecord {
            id: "sync_link_01gcal".to_string(),
            provider: "google-calendar".to_string(),
            integration_account_id: "integration_account_google".to_string(),
            object_id: "event_01gcal".to_string(),
            remote_id: "evt_123".to_string(),
            remote_type: "event".to_string(),
            state: "reconciled".to_string(),
            authority_mode: "shared".to_string(),
            remote_version: Some("etag-1".to_string()),
            metadata_json: json!({}),
            linked_at: OffsetDateTime::UNIX_EPOCH,
            last_seen_at: OffsetDateTime::UNIX_EPOCH,
        }
    }

    #[test]
    fn google_delete_and_restore_preserve_hidden_tombstone_and_audit_lineage() {
        let deleted = apply_upstream_delete(&object(), &sync_link(), OffsetDateTime::UNIX_EPOCH);
        assert_eq!(deleted.object.status, "deleted");
        assert_eq!(
            deleted.object.source_summary_json.as_ref().unwrap()["hidden_from_default_queries"],
            JsonValue::Bool(true)
        );
        assert_eq!(deleted.reconciliation_state, "pending_reconcile");

        let restored =
            restore_from_tombstone(&deleted.object, &sync_link(), OffsetDateTime::UNIX_EPOCH);
        assert_eq!(restored.object.status, "active");
        assert_eq!(
            restored.object.source_summary_json.as_ref().unwrap()["audit_lineage_preserved"],
            JsonValue::Bool(true)
        );
        assert_eq!(restored.reconciliation_state, "restored");
    }
}

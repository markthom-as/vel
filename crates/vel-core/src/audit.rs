use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventKind {
    Allowed,
    Denied,
    DryRun,
    ApprovalRequired,
    DispatchStarted,
    DispatchSucceeded,
    DispatchFailed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditBeforeAfter {
    pub before: Option<JsonValue>,
    pub after: Option<JsonValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditFieldCapture {
    pub field: String,
    pub before_after: Option<AuditBeforeAfter>,
    pub diff: Option<JsonValue>,
    pub reference: Option<String>,
    pub redacted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditRecord {
    pub action_name: String,
    pub target_object_refs: Vec<String>,
    pub dry_run: bool,
    pub approval_required: bool,
    pub outcome: AuditEventKind,
    pub reason: String,
    pub field_captures: Vec<AuditFieldCapture>,
    pub write_intent_ref: Option<String>,
    pub downstream_operation_ref: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{AuditBeforeAfter, AuditEventKind, AuditFieldCapture, AuditRecord};
    use serde_json::json;

    #[test]
    fn audit_records_support_before_after_diff_reference_and_redacted_fields() {
        let record = AuditRecord {
            action_name: "object.update".to_string(),
            target_object_refs: vec!["task_01audit".to_string()],
            dry_run: true,
            approval_required: true,
            outcome: AuditEventKind::DryRun,
            reason: "dry run still records audit intent".to_string(),
            field_captures: vec![AuditFieldCapture {
                field: "due".to_string(),
                before_after: Some(AuditBeforeAfter {
                    before: Some(json!("2026-03-22")),
                    after: Some(json!("2026-03-23")),
                }),
                diff: Some(json!({"op":"replace","path":"/due","value":"2026-03-23"})),
                reference: Some("sync_link_01audit".to_string()),
                redacted: false,
            }],
            write_intent_ref: Some("write_intent_01audit".to_string()),
            downstream_operation_ref: None,
        };

        assert!(record.dry_run);
        assert!(record.approval_required);
        assert!(matches!(record.outcome, AuditEventKind::DryRun));
        assert_eq!(record.field_captures[0].reference.as_deref(), Some("sync_link_01audit"));
    }
}

use sqlx::SqlitePool;
use time::OffsetDateTime;
use uuid::Uuid;
use vel_core::AuditRecord;
use vel_storage::{insert_runtime_record, RuntimeRecord};

use crate::errors::AppError;

#[derive(Debug, Default)]
pub struct AuditEmitter;

impl AuditEmitter {
    pub async fn emit(
        &self,
        pool: &SqlitePool,
        audit: &AuditRecord,
    ) -> Result<RuntimeRecord, AppError> {
        let now = OffsetDateTime::now_utc();
        let record = RuntimeRecord {
            id: format!("audit_{}", Uuid::new_v4().simple()),
            record_type: "audit".to_string(),
            object_ref: audit.target_object_refs.first().cloned(),
            status: audit_status(&audit.outcome).to_string(),
            payload_json: serde_json::to_value(audit)
                .map_err(|error| AppError::internal(error.to_string()))?,
            created_at: now,
            updated_at: now,
        };

        insert_runtime_record(pool, &record).await?;
        Ok(record)
    }
}

fn audit_status(outcome: &vel_core::AuditEventKind) -> &'static str {
    match outcome {
        vel_core::AuditEventKind::Allowed => "allowed",
        vel_core::AuditEventKind::Denied => "denied",
        vel_core::AuditEventKind::DryRun => "dry_run",
        vel_core::AuditEventKind::ApprovalRequired => "approval_required",
        vel_core::AuditEventKind::DispatchStarted => "dispatch_started",
        vel_core::AuditEventKind::DispatchSucceeded => "dispatch_succeeded",
        vel_core::AuditEventKind::DispatchFailed => "dispatch_failed",
    }
}

#[cfg(test)]
mod tests {
    use super::AuditEmitter;
    use serde_json::json;
    use sqlx::SqlitePool;
    use vel_core::{AuditBeforeAfter, AuditEventKind, AuditFieldCapture, AuditRecord};
    use vel_storage::{list_runtime_records, migrate_storage};

    async fn test_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        migrate_storage(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn audit_emitter_records_denied_dry_run_and_approval_paths() {
        let pool = test_pool().await;
        let emitter = AuditEmitter;

        for audit in [
            AuditRecord {
                action_name: "object.update".to_string(),
                target_object_refs: vec!["task_01auditdeny".to_string()],
                dry_run: false,
                approval_required: false,
                outcome: AuditEventKind::Denied,
                reason: "denied by policy".to_string(),
                field_captures: vec![],
                write_intent_ref: None,
                downstream_operation_ref: None,
            },
            AuditRecord {
                action_name: "object.update".to_string(),
                target_object_refs: vec!["task_01auditdry".to_string()],
                dry_run: true,
                approval_required: true,
                outcome: AuditEventKind::DryRun,
                reason: "dry_run preview with approval".to_string(),
                field_captures: vec![AuditFieldCapture {
                    field: "due".to_string(),
                    before_after: Some(AuditBeforeAfter {
                        before: Some(json!("2026-03-22")),
                        after: Some(json!("2026-03-23")),
                    }),
                    diff: Some(json!({"to":"2026-03-23"})),
                    reference: Some("write_intent_01dry".to_string()),
                    redacted: false,
                }],
                write_intent_ref: Some("write_intent_01dry".to_string()),
                downstream_operation_ref: None,
            },
            AuditRecord {
                action_name: "object.delete".to_string(),
                target_object_refs: vec!["task_01approval".to_string()],
                dry_run: false,
                approval_required: true,
                outcome: AuditEventKind::ApprovalRequired,
                reason: "approval gate before destructive change".to_string(),
                field_captures: vec![],
                write_intent_ref: Some("write_intent_01approval".to_string()),
                downstream_operation_ref: None,
            },
        ] {
            emitter.emit(&pool, &audit).await.unwrap();
        }

        let records = list_runtime_records(&pool, "audit").await.unwrap();
        assert_eq!(records.len(), 3);
        assert!(records.iter().any(|record| record.status == "denied"));
        assert!(records.iter().any(|record| record.payload_json["dry_run"] == json!(true)));
        assert!(records
            .iter()
            .any(|record| record.payload_json["approval_required"] == json!(true)));
    }
}

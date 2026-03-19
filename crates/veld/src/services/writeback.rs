use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::{
    ConflictCaseRecord, ConflictCaseStatus, OrderingStamp, WritebackOperationId,
    WritebackOperationKind, WritebackOperationRecord, WritebackRisk, WritebackStatus,
};
use vel_storage::Storage;

use crate::{errors::AppError, services::integrations_todoist};

pub async fn queue_writeback_operation(
    storage: &Storage,
    record: WritebackOperationRecord,
    ordering_stamp: OrderingStamp,
) -> Result<WritebackOperationRecord, AppError> {
    storage
        .insert_writeback_operation(&record, &ordering_stamp)
        .await
        .map_err(Into::into)
}

pub async fn mark_writeback_applied(
    storage: &Storage,
    id: &WritebackOperationId,
    result_payload: Option<JsonValue>,
    applied_at: OffsetDateTime,
) -> Result<WritebackOperationRecord, AppError> {
    let mut record = storage
        .get_writeback_operation(id.as_ref())
        .await?
        .ok_or_else(|| AppError::not_found(format!("writeback operation {} not found", id)))?;
    record.status = WritebackStatus::Applied;
    record.result_payload = result_payload;
    record.applied_at = Some(applied_at);
    record.updated_at = applied_at;
    storage.update_writeback_operation(&record).await?;
    Ok(record)
}

pub async fn mark_writeback_failed(
    storage: &Storage,
    id: &WritebackOperationId,
    result_payload: Option<JsonValue>,
    updated_at: OffsetDateTime,
) -> Result<WritebackOperationRecord, AppError> {
    let mut record = storage
        .get_writeback_operation(id.as_ref())
        .await?
        .ok_or_else(|| AppError::not_found(format!("writeback operation {} not found", id)))?;
    record.status = WritebackStatus::Failed;
    record.result_payload = result_payload;
    record.updated_at = updated_at;
    storage.update_writeback_operation(&record).await?;
    Ok(record)
}

pub async fn open_conflict_case(
    storage: &Storage,
    record: ConflictCaseRecord,
    writeback_id: Option<&WritebackOperationId>,
) -> Result<ConflictCaseRecord, AppError> {
    let record = storage.insert_conflict_case(&record).await?;
    if let Some(writeback_id) = writeback_id {
        let mut operation = storage
            .get_writeback_operation(writeback_id.as_ref())
            .await?
            .ok_or_else(|| {
                AppError::not_found(format!("writeback operation {} not found", writeback_id))
            })?;
        operation.status = WritebackStatus::Conflicted;
        operation.conflict_case_id = Some(record.id.to_string());
        operation.updated_at = record.updated_at;
        storage.update_writeback_operation(&operation).await?;
    }
    Ok(record)
}

pub async fn list_pending_writebacks(
    storage: &Storage,
    limit: u32,
) -> Result<Vec<WritebackOperationRecord>, AppError> {
    let queued = storage
        .list_writeback_operations(Some(WritebackStatus::Queued), limit)
        .await?;
    let in_progress = storage
        .list_writeback_operations(Some(WritebackStatus::InProgress), limit)
        .await?;
    let conflicted = storage
        .list_writeback_operations(Some(WritebackStatus::Conflicted), limit)
        .await?;
    let mut combined = Vec::new();
    combined.extend(queued);
    combined.extend(in_progress);
    combined.extend(conflicted);
    combined.sort_by(|left, right| {
        right
            .updated_at
            .cmp(&left.updated_at)
            .then_with(|| left.id.as_ref().cmp(right.id.as_ref()))
    });
    combined.truncate(limit as usize);
    Ok(combined)
}

pub async fn list_open_conflicts(
    storage: &Storage,
    limit: u32,
) -> Result<Vec<ConflictCaseRecord>, AppError> {
    storage
        .list_open_conflict_cases(limit)
        .await
        .map_err(Into::into)
}

pub(crate) async fn todoist_create_task(
    storage: &Storage,
    requested_by_node_id: &str,
    mutation: integrations_todoist::TodoistTaskMutation,
) -> Result<WritebackOperationRecord, AppError> {
    let plan =
        integrations_todoist::plan_todoist_create_task(storage, requested_by_node_id, mutation)
            .await?;
    execute_todoist_writeback(storage, plan, WritebackOperationKind::TodoistCreateTask).await
}

pub(crate) async fn todoist_update_task(
    storage: &Storage,
    requested_by_node_id: &str,
    commitment_id: &str,
    mutation: integrations_todoist::TodoistTaskMutation,
) -> Result<WritebackOperationRecord, AppError> {
    let plan = integrations_todoist::plan_todoist_update_task(
        storage,
        requested_by_node_id,
        commitment_id,
        mutation,
    )
    .await?;
    execute_todoist_writeback(storage, plan, WritebackOperationKind::TodoistUpdateTask).await
}

pub(crate) async fn todoist_complete_task(
    storage: &Storage,
    requested_by_node_id: &str,
    commitment_id: &str,
) -> Result<WritebackOperationRecord, AppError> {
    let plan = integrations_todoist::plan_todoist_complete_task(
        storage,
        requested_by_node_id,
        commitment_id,
    )
    .await?;
    execute_todoist_writeback(storage, plan, WritebackOperationKind::TodoistCompleteTask).await
}

pub(crate) async fn todoist_reopen_task(
    storage: &Storage,
    requested_by_node_id: &str,
    commitment_id: &str,
) -> Result<WritebackOperationRecord, AppError> {
    let plan = integrations_todoist::plan_todoist_reopen_task(
        storage,
        requested_by_node_id,
        commitment_id,
    )
    .await?;
    execute_todoist_writeback(storage, plan, WritebackOperationKind::TodoistReopenTask).await
}

async fn execute_todoist_writeback(
    storage: &Storage,
    plan: integrations_todoist::TodoistWritePlan,
    kind: WritebackOperationKind,
) -> Result<WritebackOperationRecord, AppError> {
    let now = OffsetDateTime::now_utc();
    let ordering_stamp = OrderingStamp::new(
        now.unix_timestamp(),
        0,
        vel_core::NodeIdentity::from(plan.requested_by_node_id.clone()),
    );
    let queued = queue_writeback_operation(
        storage,
        WritebackOperationRecord {
            id: WritebackOperationId::new(),
            kind,
            risk: WritebackRisk::Safe,
            status: WritebackStatus::Queued,
            target: plan.target.clone(),
            requested_payload: plan.requested_payload.clone(),
            result_payload: None,
            provenance: plan.provenance.clone(),
            conflict_case_id: None,
            requested_by_node_id: plan.requested_by_node_id.clone(),
            requested_at: now,
            applied_at: None,
            updated_at: now,
        },
        ordering_stamp,
    )
    .await?;

    match integrations_todoist::execute_todoist_write_plan(storage, &plan).await? {
        integrations_todoist::TodoistWriteExecutionResult::Applied {
            result_payload,
            target_external_id,
            provenance,
        } => {
            let mut stored = storage
                .get_writeback_operation(queued.id.as_ref())
                .await?
                .ok_or_else(|| {
                    AppError::not_found(format!(
                        "writeback operation {} not found after queue",
                        queued.id
                    ))
                })?;
            stored.status = WritebackStatus::Applied;
            stored.result_payload = Some(result_payload);
            stored.applied_at = Some(OffsetDateTime::now_utc());
            stored.updated_at = stored.applied_at.unwrap_or(now);
            if let Some(external_id) = target_external_id {
                stored.target.external_id = Some(external_id);
            }
            if !provenance.is_empty() {
                stored.provenance = provenance;
            }
            storage.update_writeback_operation(&stored).await?;
            Ok(stored)
        }
        integrations_todoist::TodoistWriteExecutionResult::Conflict {
            kind,
            summary,
            upstream_payload,
        } => {
            let now = OffsetDateTime::now_utc();
            let conflict = open_conflict_case(
                storage,
                ConflictCaseRecord {
                    id: vel_core::ConflictCaseId::new(),
                    kind,
                    status: ConflictCaseStatus::Open,
                    target: plan.target,
                    summary,
                    local_payload: plan.requested_payload,
                    upstream_payload,
                    resolution_payload: None,
                    opened_at: now,
                    resolved_at: None,
                    updated_at: now,
                },
                Some(&queued.id),
            )
            .await?;
            let mut stored = storage
                .get_writeback_operation(queued.id.as_ref())
                .await?
                .ok_or_else(|| {
                    AppError::not_found(format!(
                        "writeback operation {} not found after conflict",
                        queued.id
                    ))
                })?;
            stored.conflict_case_id = Some(conflict.id.to_string());
            Ok(stored)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Duration;
    use vel_core::{
        ConflictCaseKind, ConflictCaseStatus, IntegrationFamily, NodeIdentity,
        WritebackOperationKind, WritebackRisk, WritebackTargetRef,
    };
    use vel_storage::Storage;

    fn sample_writeback(now: OffsetDateTime) -> (WritebackOperationRecord, OrderingStamp) {
        (
            WritebackOperationRecord {
                id: "wb_service_1".to_string().into(),
                kind: WritebackOperationKind::TodoistCreateTask,
                risk: WritebackRisk::Safe,
                status: WritebackStatus::Queued,
                target: WritebackTargetRef {
                    family: IntegrationFamily::Tasks,
                    provider_key: "todoist".to_string(),
                    project_id: Some("proj_service".to_string().into()),
                    connection_id: Some("icn_service".to_string().into()),
                    external_id: Some("todo_1".to_string()),
                },
                requested_payload: serde_json::json!({"content": "follow up"}),
                result_payload: None,
                provenance: vec![],
                conflict_case_id: None,
                requested_by_node_id: "node-alpha".to_string(),
                requested_at: now,
                applied_at: None,
                updated_at: now,
            },
            OrderingStamp::new(
                now.unix_timestamp(),
                1,
                NodeIdentity::from("123e4567-e89b-12d3-a456-426614174000".to_string()),
            ),
        )
    }

    #[tokio::test]
    async fn open_conflict_case_marks_writeback_conflicted() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = OffsetDateTime::now_utc();
        let (record, ordering_stamp) = sample_writeback(now);
        queue_writeback_operation(&storage, record.clone(), ordering_stamp)
            .await
            .unwrap();

        let conflict = ConflictCaseRecord {
            id: "conf_service_1".to_string().into(),
            kind: ConflictCaseKind::UpstreamVsLocal,
            status: ConflictCaseStatus::Open,
            target: record.target.clone(),
            summary: "upstream changed".to_string(),
            local_payload: serde_json::json!({"content": "follow up"}),
            upstream_payload: Some(serde_json::json!({"content": "other"})),
            resolution_payload: None,
            opened_at: now,
            resolved_at: None,
            updated_at: now + Duration::minutes(1),
        };

        open_conflict_case(&storage, conflict.clone(), Some(&record.id))
            .await
            .unwrap();

        let stored = storage
            .get_writeback_operation(record.id.as_ref())
            .await
            .unwrap()
            .expect("writeback should exist");
        assert_eq!(stored.status, WritebackStatus::Conflicted);
        assert_eq!(stored.conflict_case_id.as_deref(), Some("conf_service_1"));
    }
}

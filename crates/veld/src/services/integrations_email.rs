#![allow(dead_code)] // Email writeback helpers are staged until supervised mutation entry points land.

use serde_json::{json, Value as JsonValue};
use time::OffsetDateTime;
use uuid::Uuid;
use vel_core::{
    IntegrationConnectionId, IntegrationConnectionStatus, IntegrationFamily, IntegrationProvider,
    IntegrationSourceRef, NodeIdentity, OrderingStamp, PersonAlias, PersonRecord, ProjectId,
    WritebackOperationId, WritebackOperationKind, WritebackOperationRecord, WritebackRisk,
    WritebackStatus, WritebackTargetRef,
};
use vel_storage::{
    IntegrationConnectionFilters, IntegrationConnectionInsert, Storage, UpstreamObjectRefRecord,
};

use crate::{errors::AppError, services::writeback::queue_writeback_operation};

pub(crate) const EMAIL_PROVIDER_KEY: &str = "email";

#[derive(Debug, Clone)]
pub(crate) struct EmailCreateDraftReplyRequest {
    pub thread_id: String,
    pub subject: Option<String>,
    pub body: String,
    pub sender: Option<String>,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub project_id: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct EmailSendDraftRequest {
    pub draft_id: String,
    pub sender: Option<String>,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub project_id: Option<String>,
    pub confirm: bool,
}

pub(crate) async fn email_create_draft_reply(
    storage: &Storage,
    requested_by_node_id: &str,
    request: EmailCreateDraftReplyRequest,
) -> Result<WritebackOperationRecord, AppError> {
    let thread_id = normalize_required(&request.thread_id, "thread_id")?;
    let body = normalize_required(&request.body, "body")?;
    let requested_by_node_id = normalize_requested_by_node_id(requested_by_node_id);
    let connection_id = ensure_email_connection(storage).await?;
    let project = resolve_project(storage, request.project_id.as_deref()).await?;
    let participants =
        resolve_people_links(storage, &request.sender, &request.to, &request.cc).await?;
    let normalized_sender = normalize_optional(request.sender.clone());
    let draft_id = format!("draft_{}", Uuid::new_v4().simple());
    let requested_payload = json!({
        "operation": "email_create_draft_reply",
        "thread_id": thread_id,
        "subject": normalize_optional(request.subject),
        "body": body,
        "sender": normalized_sender,
        "to": request.to,
        "cc": request.cc,
        "project_id": project.as_ref().map(|value| value.as_ref()),
        "people": participants,
    });
    let stored = queue_writeback_operation(
        storage,
        terminal_record(
            WritebackOperationKind::EmailCreateDraftReply,
            WritebackRisk::Safe,
            WritebackStatus::Applied,
            WritebackTargetRef {
                family: IntegrationFamily::Messaging,
                provider_key: EMAIL_PROVIDER_KEY.to_string(),
                project_id: project.clone(),
                connection_id: Some(connection_id.clone()),
                external_id: Some(draft_id.clone()),
            },
            requested_payload,
            Some(json!({
                "state": "applied",
                "draft_state": "draft-first",
                "thread_id": thread_id,
                "draft_id": draft_id,
                "project_id": project.as_ref().map(|value| value.as_ref()),
                "people": participants,
            })),
            vec![IntegrationSourceRef {
                family: IntegrationFamily::Messaging,
                provider_key: EMAIL_PROVIDER_KEY.to_string(),
                connection_id: connection_id.clone(),
                external_id: draft_id.clone(),
            }],
            requested_by_node_id.clone(),
        ),
        ordering_stamp_for(&requested_by_node_id),
    )
    .await?;

    if let Some(sender) = normalized_sender {
        remember_email_sender(storage, &connection_id, &sender).await?;
    }
    upsert_operation_ref(
        storage,
        &stored.id,
        &project,
        &draft_id,
        Some(&thread_id),
        "draft",
        &requested_by_node_id,
        json!({}),
    )
    .await?;

    Ok(stored)
}

pub(crate) async fn email_send_draft(
    storage: &Storage,
    requested_by_node_id: &str,
    request: EmailSendDraftRequest,
) -> Result<WritebackOperationRecord, AppError> {
    let draft_id = normalize_required(&request.draft_id, "draft_id")?;
    let requested_by_node_id = normalize_requested_by_node_id(requested_by_node_id);
    let connection_id = ensure_email_connection(storage).await?;
    let project = resolve_project(storage, request.project_id.as_deref()).await?;
    let participants =
        resolve_people_links(storage, &request.sender, &request.to, &request.cc).await?;
    let normalized_sender = normalize_optional(request.sender.clone());
    let status = if request.confirm {
        WritebackStatus::Applied
    } else {
        WritebackStatus::Denied
    };
    let result_state = if request.confirm {
        "applied"
    } else {
        "confirm_required"
    };
    let stored = queue_writeback_operation(
        storage,
        terminal_record(
            WritebackOperationKind::EmailSendDraft,
            WritebackRisk::ConfirmRequired,
            status,
            WritebackTargetRef {
                family: IntegrationFamily::Messaging,
                provider_key: EMAIL_PROVIDER_KEY.to_string(),
                project_id: project.clone(),
                connection_id: Some(connection_id.clone()),
                external_id: Some(draft_id.clone()),
            },
            json!({
                "operation": "email_send_draft",
                "draft_id": draft_id,
                "sender": normalized_sender,
                "to": request.to,
                "cc": request.cc,
                "project_id": project.as_ref().map(|value| value.as_ref()),
                "confirm": request.confirm,
                "people": participants,
            }),
            Some(json!({
                "state": result_state,
                "draft_id": draft_id,
                "confirm_required": true,
                "project_id": project.as_ref().map(|value| value.as_ref()),
                "people": participants,
            })),
            vec![IntegrationSourceRef {
                family: IntegrationFamily::Messaging,
                provider_key: EMAIL_PROVIDER_KEY.to_string(),
                connection_id: connection_id.clone(),
                external_id: draft_id.clone(),
            }],
            requested_by_node_id.clone(),
        ),
        ordering_stamp_for(&requested_by_node_id),
    )
    .await?;

    if request.confirm {
        if let Some(sender) = normalized_sender {
            remember_email_sender(storage, &connection_id, &sender).await?;
        }
        upsert_operation_ref(
            storage,
            &stored.id,
            &project,
            &draft_id,
            None,
            "send",
            &requested_by_node_id,
            json!({ "confirm_required": true }),
        )
        .await?;
    }

    Ok(stored)
}

fn terminal_record(
    kind: WritebackOperationKind,
    risk: WritebackRisk,
    status: WritebackStatus,
    target: WritebackTargetRef,
    requested_payload: JsonValue,
    result_payload: Option<JsonValue>,
    provenance: Vec<IntegrationSourceRef>,
    requested_by_node_id: String,
) -> WritebackOperationRecord {
    let now = OffsetDateTime::now_utc();
    WritebackOperationRecord {
        id: WritebackOperationId::new(),
        kind,
        risk,
        status,
        target,
        requested_payload,
        result_payload,
        provenance,
        conflict_case_id: None,
        requested_by_node_id,
        requested_at: now,
        applied_at: (status == WritebackStatus::Applied).then_some(now),
        updated_at: now,
    }
}

async fn ensure_email_connection(storage: &Storage) -> Result<IntegrationConnectionId, AppError> {
    let mut existing = storage
        .list_integration_connections(IntegrationConnectionFilters {
            family: Some(IntegrationFamily::Messaging),
            provider_key: Some(EMAIL_PROVIDER_KEY.to_string()),
            status: None,
            include_disabled: true,
        })
        .await?;
    if let Some(connection) = existing.pop() {
        return Ok(connection.id);
    }

    let provider = IntegrationProvider::new(IntegrationFamily::Messaging, EMAIL_PROVIDER_KEY)
        .map_err(|error| AppError::internal(format!("email provider: {error}")))?;
    storage
        .insert_integration_connection(IntegrationConnectionInsert {
            family: IntegrationFamily::Messaging,
            provider,
            status: IntegrationConnectionStatus::Connected,
            display_name: "Email".to_string(),
            account_ref: None,
            metadata_json: json!({
                "foundation": true,
                "write_surface": [
                    "email_create_draft_reply",
                    "email_send_draft"
                ],
                "draft-first": true
            }),
        })
        .await
        .map_err(Into::into)
}

async fn remember_email_sender(
    storage: &Storage,
    connection_id: &IntegrationConnectionId,
    sender: &str,
) -> Result<(), AppError> {
    storage
        .upsert_integration_connection_setting_ref(connection_id.as_ref(), "email_sender", sender)
        .await?;
    Ok(())
}

async fn resolve_project(
    storage: &Storage,
    project_id: Option<&str>,
) -> Result<Option<ProjectId>, AppError> {
    if let Some(project_id) = project_id.map(str::trim).filter(|value| !value.is_empty()) {
        return Ok(storage
            .get_project(project_id)
            .await?
            .map(|project| project.id));
    }
    Ok(None)
}

async fn resolve_people_links(
    storage: &Storage,
    sender: &Option<String>,
    to: &[String],
    cc: &[String],
) -> Result<Vec<JsonValue>, AppError> {
    let mut handles = Vec::new();
    if let Some(sender) = sender.as_deref() {
        let sender = sender.trim();
        if !sender.is_empty() {
            handles.push(sender.to_string());
        }
    }
    handles.extend(
        to.iter()
            .chain(cc.iter())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty()),
    );
    if handles.is_empty() {
        return Ok(Vec::new());
    }

    let people = storage.list_people().await?;
    Ok(handles
        .into_iter()
        .map(|handle| match_person_alias(&people, "email", &handle))
        .collect())
}

fn match_person_alias(people: &[PersonRecord], platform: &str, handle: &str) -> JsonValue {
    let normalized_handle = handle.to_ascii_lowercase();
    let matched = people.iter().find_map(|person| {
        person.aliases.iter().find_map(|alias| {
            alias_matches(alias, platform, &normalized_handle).then(|| {
                json!({
                    "person_id": person.id.as_ref(),
                    "platform": alias.platform,
                    "handle": alias.handle,
                    "display": alias.display,
                })
            })
        })
    });
    matched.unwrap_or_else(|| {
        json!({
            "person_id": JsonValue::Null,
            "platform": platform,
            "handle": handle,
            "display": handle,
        })
    })
}

fn alias_matches(alias: &PersonAlias, platform: &str, handle: &str) -> bool {
    alias.platform.eq_ignore_ascii_case(platform) && alias.handle.eq_ignore_ascii_case(handle)
}

async fn upsert_operation_ref(
    storage: &Storage,
    writeback_id: &WritebackOperationId,
    project_id: &Option<ProjectId>,
    external_id: &str,
    external_parent_id: Option<&str>,
    provider_object: &str,
    requested_by_node_id: &str,
    metadata_json: JsonValue,
) -> Result<(), AppError> {
    storage
        .upsert_upstream_object_ref(&UpstreamObjectRefRecord {
            id: format!("uor_email_writeback_{}", Uuid::new_v4().simple()),
            family: IntegrationFamily::Messaging,
            provider_key: EMAIL_PROVIDER_KEY.to_string(),
            project_id: project_id.clone(),
            local_object_kind: "writeback_operation".to_string(),
            local_object_id: writeback_id.as_ref().to_string(),
            external_id: external_id.to_string(),
            external_parent_id: external_parent_id.map(ToOwned::to_owned),
            ordering_stamp: ordering_stamp_for(requested_by_node_id),
            last_seen_at: OffsetDateTime::now_utc(),
            metadata_json: json!({
                "provider_object": provider_object,
                "metadata": metadata_json,
            }),
        })
        .await?;
    Ok(())
}

fn ordering_stamp_for(requested_by_node_id: &str) -> OrderingStamp {
    OrderingStamp::new(
        OffsetDateTime::now_utc().unix_timestamp(),
        0,
        NodeIdentity::from(normalize_requested_by_node_id(requested_by_node_id)),
    )
}

fn normalize_requested_by_node_id(node_id: &str) -> String {
    let trimmed = node_id.trim();
    if trimmed.is_empty() {
        "vel-local".to_string()
    } else {
        trimmed.to_string()
    }
}

fn normalize_required(value: &str, field: &str) -> Result<String, AppError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AppError::bad_request(format!("{field} must not be empty")))
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use vel_core::{PersonId, PersonRecord};

    #[tokio::test]
    async fn email_send_draft_requires_confirm_required_before_apply() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .create_person(PersonRecord {
                id: PersonId::from("per_email".to_string()),
                display_name: "Sam".to_string(),
                given_name: None,
                family_name: None,
                relationship_context: None,
                birthday: None,
                last_contacted_at: None,
                aliases: vec![PersonAlias {
                    platform: "email".to_string(),
                    handle: "sam@example.com".to_string(),
                    display: "Sam".to_string(),
                    source_ref: None,
                }],
                links: vec![],
            })
            .await
            .unwrap();

        let denied = email_send_draft(
            &storage,
            "node-local",
            EmailSendDraftRequest {
                draft_id: "draft_123".to_string(),
                sender: Some("sam@example.com".to_string()),
                to: vec!["sam@example.com".to_string()],
                cc: vec![],
                project_id: None,
                confirm: false,
            },
        )
        .await
        .unwrap();

        assert_eq!(denied.kind, WritebackOperationKind::EmailSendDraft);
        assert_eq!(denied.risk, WritebackRisk::ConfirmRequired);
        assert_eq!(denied.status, WritebackStatus::Denied);
        assert_eq!(
            denied.result_payload.as_ref().unwrap()["state"],
            "confirm_required"
        );
        assert_eq!(
            denied.result_payload.as_ref().unwrap()["people"][0]["person_id"],
            "per_email"
        );

        let applied = email_send_draft(
            &storage,
            "node-local",
            EmailSendDraftRequest {
                draft_id: "draft_123".to_string(),
                sender: Some("sam@example.com".to_string()),
                to: vec!["sam@example.com".to_string()],
                cc: vec![],
                project_id: None,
                confirm: true,
            },
        )
        .await
        .unwrap();

        assert_eq!(applied.status, WritebackStatus::Applied);
        let upstream_ref = storage
            .get_upstream_object_ref("writeback_operation", applied.id.as_ref())
            .await
            .unwrap()
            .expect("email send upstream ref should exist");
        assert_eq!(upstream_ref.provider_key, EMAIL_PROVIDER_KEY);
    }
}

#![allow(dead_code)] // GitHub writeback helpers are staged until supervised mutation entry points land.

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

pub(crate) const GITHUB_PROVIDER_KEY: &str = "github";

#[derive(Debug, Clone)]
pub(crate) struct GithubCreateIssueRequest {
    pub repository: String,
    pub title: String,
    pub body: Option<String>,
    pub project_id: Option<String>,
    pub assignee_handles: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct GithubCommentRequest {
    pub repository: String,
    pub issue_number: u64,
    pub body: String,
    pub project_id: Option<String>,
    pub participant_handles: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct GithubIssueActionRequest {
    pub repository: String,
    pub issue_number: u64,
    pub project_id: Option<String>,
    pub participant_handles: Vec<String>,
}

pub(crate) async fn github_create_issue(
    storage: &Storage,
    requested_by_node_id: &str,
    request: GithubCreateIssueRequest,
) -> Result<WritebackOperationRecord, AppError> {
    let repository = normalize_required(&request.repository, "repository")?;
    let title = normalize_required(&request.title, "title")?;
    let body = normalize_optional(request.body);
    let requested_by_node_id = normalize_requested_by_node_id(requested_by_node_id);
    let connection_id = ensure_github_connection(storage).await?;
    let project =
        resolve_project(storage, request.project_id.as_deref(), Some(&repository)).await?;
    let people = resolve_people_links(storage, "github", &request.assignee_handles).await?;
    let issue_number = synthetic_issue_number();
    let issue_ref = format!("{repository}#{issue_number}");
    let requested_payload = json!({
        "operation": "github_create_issue",
        "repository": repository,
        "title": title,
        "body": body,
        "project_id": project.as_ref().map(|value| value.as_ref()),
        "people": people,
    });
    let record = terminal_record(
        WritebackOperationKind::GithubCreateIssue,
        WritebackRisk::Safe,
        WritebackStatus::Applied,
        WritebackTargetRef {
            family: IntegrationFamily::Git,
            provider_key: GITHUB_PROVIDER_KEY.to_string(),
            project_id: project.clone(),
            connection_id: Some(connection_id.clone()),
            external_id: Some(issue_ref.clone()),
        },
        requested_payload,
        Some(json!({
            "state": "applied",
            "repository": repository,
            "issue_number": issue_number,
            "issue_ref": issue_ref,
            "project_id": project.as_ref().map(|value| value.as_ref()),
            "people": people,
        })),
        vec![IntegrationSourceRef {
            family: IntegrationFamily::Git,
            provider_key: GITHUB_PROVIDER_KEY.to_string(),
            connection_id: connection_id.clone(),
            external_id: issue_ref.clone(),
        }],
        requested_by_node_id.clone(),
    );
    let stored =
        queue_writeback_operation(storage, record, ordering_stamp_for(&requested_by_node_id))
            .await?;

    remember_repository_scope(storage, &connection_id, &repository).await?;
    upsert_repository_ref(storage, &project, &repository, &requested_by_node_id).await?;
    upsert_operation_ref(
        storage,
        &stored.id,
        &project,
        &repository,
        &issue_ref,
        "issue",
        &requested_by_node_id,
        json!({ "title": title }),
    )
    .await?;

    Ok(stored)
}

pub(crate) async fn github_add_comment(
    storage: &Storage,
    requested_by_node_id: &str,
    request: GithubCommentRequest,
) -> Result<WritebackOperationRecord, AppError> {
    let repository = normalize_required(&request.repository, "repository")?;
    let body = normalize_required(&request.body, "body")?;
    let requested_by_node_id = normalize_requested_by_node_id(requested_by_node_id);
    let connection_id = ensure_github_connection(storage).await?;
    let project =
        resolve_project(storage, request.project_id.as_deref(), Some(&repository)).await?;
    let people = resolve_people_links(storage, "github", &request.participant_handles).await?;
    let issue_ref = format!("{repository}#{}", request.issue_number);
    let comment_ref = format!("{issue_ref}/comment/{}", Uuid::new_v4().simple());
    let requested_payload = json!({
        "operation": "github_add_comment",
        "repository": repository,
        "issue_number": request.issue_number,
        "body": body,
        "project_id": project.as_ref().map(|value| value.as_ref()),
        "people": people,
    });
    let stored = queue_writeback_operation(
        storage,
        terminal_record(
            WritebackOperationKind::GithubAddComment,
            WritebackRisk::Safe,
            WritebackStatus::Applied,
            WritebackTargetRef {
                family: IntegrationFamily::Git,
                provider_key: GITHUB_PROVIDER_KEY.to_string(),
                project_id: project.clone(),
                connection_id: Some(connection_id.clone()),
                external_id: Some(issue_ref.clone()),
            },
            requested_payload,
            Some(json!({
                "state": "applied",
                "repository": repository,
                "issue_number": request.issue_number,
                "issue_ref": issue_ref,
                "comment_ref": comment_ref,
                "project_id": project.as_ref().map(|value| value.as_ref()),
                "people": people,
            })),
            vec![IntegrationSourceRef {
                family: IntegrationFamily::Git,
                provider_key: GITHUB_PROVIDER_KEY.to_string(),
                connection_id: connection_id.clone(),
                external_id: comment_ref.clone(),
            }],
            requested_by_node_id.clone(),
        ),
        ordering_stamp_for(&requested_by_node_id),
    )
    .await?;

    remember_repository_scope(storage, &connection_id, &repository).await?;
    upsert_repository_ref(storage, &project, &repository, &requested_by_node_id).await?;
    upsert_operation_ref(
        storage,
        &stored.id,
        &project,
        &repository,
        &comment_ref,
        "comment",
        &requested_by_node_id,
        json!({ "issue_ref": issue_ref }),
    )
    .await?;

    Ok(stored)
}

pub(crate) async fn github_close_issue(
    storage: &Storage,
    requested_by_node_id: &str,
    request: GithubIssueActionRequest,
) -> Result<WritebackOperationRecord, AppError> {
    github_issue_state_change(
        storage,
        requested_by_node_id,
        request,
        WritebackOperationKind::GithubCloseIssue,
        "closed",
    )
    .await
}

pub(crate) async fn github_reopen_issue(
    storage: &Storage,
    requested_by_node_id: &str,
    request: GithubIssueActionRequest,
) -> Result<WritebackOperationRecord, AppError> {
    github_issue_state_change(
        storage,
        requested_by_node_id,
        request,
        WritebackOperationKind::GithubReopenIssue,
        "open",
    )
    .await
}

async fn github_issue_state_change(
    storage: &Storage,
    requested_by_node_id: &str,
    request: GithubIssueActionRequest,
    kind: WritebackOperationKind,
    state: &str,
) -> Result<WritebackOperationRecord, AppError> {
    let repository = normalize_required(&request.repository, "repository")?;
    let requested_by_node_id = normalize_requested_by_node_id(requested_by_node_id);
    let connection_id = ensure_github_connection(storage).await?;
    let project =
        resolve_project(storage, request.project_id.as_deref(), Some(&repository)).await?;
    let people = resolve_people_links(storage, "github", &request.participant_handles).await?;
    let issue_ref = format!("{repository}#{}", request.issue_number);
    let requested_payload = json!({
        "operation": kind.to_string(),
        "repository": repository,
        "issue_number": request.issue_number,
        "project_id": project.as_ref().map(|value| value.as_ref()),
        "people": people,
    });
    let stored = queue_writeback_operation(
        storage,
        terminal_record(
            kind,
            WritebackRisk::Safe,
            WritebackStatus::Applied,
            WritebackTargetRef {
                family: IntegrationFamily::Git,
                provider_key: GITHUB_PROVIDER_KEY.to_string(),
                project_id: project.clone(),
                connection_id: Some(connection_id.clone()),
                external_id: Some(issue_ref.clone()),
            },
            requested_payload,
            Some(json!({
                "state": "applied",
                "issue_state": state,
                "repository": repository,
                "issue_number": request.issue_number,
                "issue_ref": issue_ref,
                "project_id": project.as_ref().map(|value| value.as_ref()),
                "people": people,
            })),
            vec![IntegrationSourceRef {
                family: IntegrationFamily::Git,
                provider_key: GITHUB_PROVIDER_KEY.to_string(),
                connection_id: connection_id.clone(),
                external_id: issue_ref.clone(),
            }],
            requested_by_node_id.clone(),
        ),
        ordering_stamp_for(&requested_by_node_id),
    )
    .await?;

    remember_repository_scope(storage, &connection_id, &repository).await?;
    upsert_repository_ref(storage, &project, &repository, &requested_by_node_id).await?;
    upsert_operation_ref(
        storage,
        &stored.id,
        &project,
        &repository,
        &issue_ref,
        "issue_state",
        &requested_by_node_id,
        json!({ "issue_state": state }),
    )
    .await?;

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
        applied_at: Some(now),
        updated_at: now,
    }
}

async fn ensure_github_connection(storage: &Storage) -> Result<IntegrationConnectionId, AppError> {
    let mut existing = storage
        .list_integration_connections(IntegrationConnectionFilters {
            family: Some(IntegrationFamily::Git),
            provider_key: Some(GITHUB_PROVIDER_KEY.to_string()),
            status: None,
            include_disabled: true,
        })
        .await?;
    if let Some(connection) = existing.pop() {
        return Ok(connection.id);
    }

    let provider = IntegrationProvider::new(IntegrationFamily::Git, GITHUB_PROVIDER_KEY)
        .map_err(|error| AppError::internal(format!("github provider: {error}")))?;
    storage
        .insert_integration_connection(IntegrationConnectionInsert {
            family: IntegrationFamily::Git,
            provider,
            status: IntegrationConnectionStatus::Connected,
            display_name: "GitHub".to_string(),
            account_ref: None,
            metadata_json: json!({
                "foundation": true,
                "write_surface": [
                    "github_create_issue",
                    "github_add_comment",
                    "github_close_issue",
                    "github_reopen_issue"
                ]
            }),
        })
        .await
        .map_err(Into::into)
}

async fn remember_repository_scope(
    storage: &Storage,
    connection_id: &IntegrationConnectionId,
    repository: &str,
) -> Result<(), AppError> {
    storage
        .upsert_integration_connection_setting_ref(
            connection_id.as_ref(),
            "github_repository",
            repository,
        )
        .await?;
    Ok(())
}

async fn resolve_project(
    storage: &Storage,
    project_id: Option<&str>,
    repository: Option<&str>,
) -> Result<Option<ProjectId>, AppError> {
    if let Some(project_id) = project_id.map(str::trim).filter(|value| !value.is_empty()) {
        return Ok(storage
            .get_project(project_id)
            .await?
            .map(|project| project.id));
    }
    if let Some(repository) = repository.map(str::trim).filter(|value| !value.is_empty()) {
        return Ok(storage
            .get_project_by_upstream_id(GITHUB_PROVIDER_KEY, repository)
            .await?
            .map(|project| project.id));
    }
    Ok(None)
}

async fn resolve_people_links(
    storage: &Storage,
    platform: &str,
    handles: &[String],
) -> Result<Vec<JsonValue>, AppError> {
    let trimmed_handles: Vec<String> = handles
        .iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect();
    if trimmed_handles.is_empty() {
        return Ok(Vec::new());
    }
    let people = storage.list_people().await?;
    Ok(trimmed_handles
        .into_iter()
        .map(|handle| match_person_alias(&people, platform, &handle))
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

async fn upsert_repository_ref(
    storage: &Storage,
    project_id: &Option<ProjectId>,
    repository: &str,
    requested_by_node_id: &str,
) -> Result<(), AppError> {
    let Some(project_id) = project_id else {
        return Ok(());
    };
    storage
        .upsert_upstream_object_ref(&UpstreamObjectRefRecord {
            id: format!("uor_github_project_{}", Uuid::new_v4().simple()),
            family: IntegrationFamily::Git,
            provider_key: GITHUB_PROVIDER_KEY.to_string(),
            project_id: Some(project_id.clone()),
            local_object_kind: "project".to_string(),
            local_object_id: project_id.as_ref().to_string(),
            external_id: repository.to_string(),
            external_parent_id: None,
            ordering_stamp: ordering_stamp_for(requested_by_node_id),
            last_seen_at: OffsetDateTime::now_utc(),
            metadata_json: json!({
                "provider_object": "repository",
                "repository": repository,
            }),
        })
        .await?;
    Ok(())
}

async fn upsert_operation_ref(
    storage: &Storage,
    writeback_id: &WritebackOperationId,
    project_id: &Option<ProjectId>,
    repository: &str,
    external_id: &str,
    provider_object: &str,
    requested_by_node_id: &str,
    metadata_json: JsonValue,
) -> Result<(), AppError> {
    storage
        .upsert_upstream_object_ref(&UpstreamObjectRefRecord {
            id: format!("uor_github_writeback_{}", Uuid::new_v4().simple()),
            family: IntegrationFamily::Git,
            provider_key: GITHUB_PROVIDER_KEY.to_string(),
            project_id: project_id.clone(),
            local_object_kind: "writeback_operation".to_string(),
            local_object_id: writeback_id.as_ref().to_string(),
            external_id: external_id.to_string(),
            external_parent_id: Some(repository.to_string()),
            ordering_stamp: ordering_stamp_for(requested_by_node_id),
            last_seen_at: OffsetDateTime::now_utc(),
            metadata_json: json!({
                "provider_object": provider_object,
                "repository": repository,
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

fn synthetic_issue_number() -> u64 {
    let bytes = *Uuid::new_v4().as_bytes();
    u64::from_be_bytes(bytes[..8].try_into().expect("uuid slice should fit"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use vel_core::{
        PersonId, PersonRecord, ProjectFamily, ProjectProvisionRequest, ProjectRecord,
        ProjectRootRef, ProjectStatus,
    };

    #[tokio::test]
    async fn github_create_issue_links_project_and_people_with_provenance() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = OffsetDateTime::now_utc();
        storage
            .create_project(ProjectRecord {
                id: "proj_github".to_string().into(),
                slug: "runtime".to_string(),
                name: "Runtime".to_string(),
                family: ProjectFamily::Work,
                status: ProjectStatus::Active,
                primary_repo: ProjectRootRef {
                    path: "/tmp/runtime".to_string(),
                    label: "runtime".to_string(),
                    kind: "repo".to_string(),
                },
                primary_notes_root: ProjectRootRef {
                    path: "/tmp/runtime-notes".to_string(),
                    label: "runtime-notes".to_string(),
                    kind: "notes_root".to_string(),
                },
                secondary_repos: vec![],
                secondary_notes_roots: vec![],
                upstream_ids: BTreeMap::from([(
                    GITHUB_PROVIDER_KEY.to_string(),
                    "vel/runtime".to_string(),
                )]),
                pending_provision: ProjectProvisionRequest::default(),
                created_at: now,
                updated_at: now,
                archived_at: None,
            })
            .await
            .unwrap();
        storage
            .create_person(PersonRecord {
                id: PersonId::from("per_github".to_string()),
                display_name: "Alex".to_string(),
                given_name: None,
                family_name: None,
                relationship_context: None,
                birthday: None,
                last_contacted_at: None,
                aliases: vec![PersonAlias {
                    platform: "github".to_string(),
                    handle: "alex".to_string(),
                    display: "Alex".to_string(),
                    source_ref: None,
                }],
                links: vec![],
            })
            .await
            .unwrap();

        let record = github_create_issue(
            &storage,
            "node-local",
            GithubCreateIssueRequest {
                repository: "vel/runtime".to_string(),
                title: "Tighten writeback scope".to_string(),
                body: Some("Use typed writeback boundaries.".to_string()),
                project_id: None,
                assignee_handles: vec!["alex".to_string()],
            },
        )
        .await
        .unwrap();

        assert_eq!(record.kind, WritebackOperationKind::GithubCreateIssue);
        assert_eq!(
            record
                .target
                .project_id
                .as_ref()
                .map(|value| value.as_ref()),
            Some("proj_github")
        );
        assert_eq!(record.target.provider_key, GITHUB_PROVIDER_KEY);
        assert_eq!(record.provenance.len(), 1);
        assert_eq!(record.provenance[0].provider_key, GITHUB_PROVIDER_KEY);
        assert_eq!(
            record.result_payload.as_ref().unwrap()["people"][0]["person_id"],
            "per_github"
        );

        let upstream_ref = storage
            .get_upstream_object_ref("writeback_operation", record.id.as_ref())
            .await
            .unwrap()
            .expect("github writeback upstream ref should exist");
        assert_eq!(upstream_ref.provider_key, GITHUB_PROVIDER_KEY);
        assert_eq!(
            upstream_ref.external_parent_id.as_deref(),
            Some("vel/runtime")
        );
    }
}

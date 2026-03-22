use serde_json::{json, Value as JsonValue};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;

use crate::todoist_ids::{todoist_provider_object_ref, TODOIST_MODULE_ID};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoistCommentAuthorStub {
    pub remote_id: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoistCommentPayload {
    pub remote_id: String,
    pub parent_remote_task_id: String,
    pub body: String,
    pub author: TodoistCommentAuthorStub,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttachedCommentRecord {
    pub id: String,
    pub parent_object_ref: String,
    pub body: String,
    pub author_ref: Option<String>,
    pub author_stub: Option<TodoistCommentAuthorStub>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub source_refs: Vec<String>,
    pub visibility_or_scope: Option<String>,
    pub provider_facets: JsonValue,
}

pub fn map_todoist_comment(
    integration_account_id: &str,
    parent_object_ref: &str,
    payload: &TodoistCommentPayload,
) -> AttachedCommentRecord {
    AttachedCommentRecord {
        id: todoist_comment_record_id(integration_account_id, &payload.remote_id),
        parent_object_ref: parent_object_ref.to_string(),
        body: payload.body.clone(),
        author_ref: None,
        author_stub: Some(payload.author.clone()),
        created_at: payload.created_at,
        updated_at: payload.updated_at,
        source_refs: vec![
            TODOIST_MODULE_ID.to_string(),
            todoist_provider_object_ref("comment", &payload.remote_id),
        ],
        visibility_or_scope: None,
        provider_facets: json!({
            "todoist": {
                "comment_id": payload.remote_id,
                "parent_task_id": payload.parent_remote_task_id,
                "author": {
                    "remote_id": payload.author.remote_id,
                    "display_name": payload.author.display_name,
                }
            }
        }),
    }
}

fn todoist_comment_record_id(integration_account_id: &str, remote_comment_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher
        .update(format!("{integration_account_id}:todoist:comment:{remote_comment_id}").as_bytes());
    let digest = hasher.finalize();
    format!("attached_comment_{}", hex::encode(&digest[..12]))
}

#[cfg(test)]
mod tests {
    use super::{
        map_todoist_comment, AttachedCommentRecord, TodoistCommentAuthorStub, TodoistCommentPayload,
    };
    use time::OffsetDateTime;

    #[test]
    fn todoist_comments_map_to_attached_comment_records() {
        let comment = map_todoist_comment(
            "integration_account_primary",
            "task_01mapped",
            &TodoistCommentPayload {
                remote_id: "comment_123".to_string(),
                parent_remote_task_id: "todo_123".to_string(),
                body: "Need to reschedule this".to_string(),
                author: TodoistCommentAuthorStub {
                    remote_id: Some("user_123".to_string()),
                    display_name: Some("Jove".to_string()),
                },
                created_at: OffsetDateTime::UNIX_EPOCH,
                updated_at: OffsetDateTime::UNIX_EPOCH,
            },
        );

        assert!(comment.id.starts_with("attached_comment_"));
        assert_eq!(comment.parent_object_ref, "task_01mapped");
        assert_eq!(comment.body, "Need to reschedule this");
        assert_eq!(
            comment.provider_facets["todoist"]["parent_task_id"],
            "todo_123"
        );

        let _: AttachedCommentRecord = comment;
    }
}

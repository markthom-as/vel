use serde_json::Value;
use vel_api_types::{
    ConversationContinuationData, NowHeaderBucketKindData, ThreadContinuationData,
};

use crate::errors::AppError;

fn string_field(metadata: &Value, key: &str) -> Option<String> {
    metadata
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn bool_field(metadata: &Value, key: &str) -> Option<bool> {
    metadata.get(key).and_then(Value::as_bool)
}

pub(crate) fn parse_thread_metadata(metadata_json: &str) -> Option<Value> {
    serde_json::from_str::<Value>(metadata_json)
        .ok()
        .filter(|value| value.is_object())
}

pub(crate) fn proposal_thread_lifecycle_stage(
    thread_type: &str,
    metadata: Option<&Value>,
) -> Option<String> {
    match thread_type {
        "assistant_proposal" | "planning_profile_edit" | "reflow_edit" | "day_plan_apply" => {
            metadata
                .and_then(|value| value.get("proposal_state"))
                .and_then(Value::as_str)
                .map(ToString::to_string)
        }
        _ => None,
    }
}

pub(crate) fn thread_continuation_data(
    thread_type: &str,
    metadata: Option<&Value>,
) -> Option<ThreadContinuationData> {
    let metadata = metadata?;

    match thread_type {
        "assistant_proposal" => {
            let mut review_requirements = Vec::new();
            if string_field(metadata, "permission_mode").as_deref() == Some("user_confirm") {
                review_requirements.push(
                    "Operator confirmation is required before the proposal can be applied."
                        .to_string(),
                );
            }
            if matches!(
                string_field(metadata, "proposal_state").as_deref(),
                Some("staged" | "approved")
            ) {
                review_requirements.push(
                    "The proposal stays review-gated until it is explicitly applied, dismissed, or reversed through an existing operator lane."
                        .to_string(),
                );
            }
            Some(ThreadContinuationData {
                escalation_reason:
                    "This assistant proposal became multi-step and remains in Threads for explicit follow-through."
                        .to_string(),
                continuation_context: metadata
                    .get("lineage")
                    .cloned()
                    .unwrap_or_else(|| Value::Object(Default::default())),
                review_requirements,
                bounded_capability_state: "proposal_review_gated".to_string(),
                continuation_category: NowHeaderBucketKindData::ReviewApply,
                open_target: thread_open_target(thread_type).to_string(),
            })
        }
        "planning_profile_edit" => {
            let mut review_requirements = Vec::new();
            if bool_field(metadata, "requires_confirmation").unwrap_or(false) {
                review_requirements.push(
                    "Planning-profile edits require explicit approval before the backend saves them."
                        .to_string(),
                );
            }
            Some(ThreadContinuationData {
                escalation_reason:
                    "This planning-profile change remains in Threads until the bounded edit is approved or rejected."
                        .to_string(),
                continuation_context: metadata
                    .get("lineage")
                    .cloned()
                    .unwrap_or_else(|| Value::Object(Default::default())),
                review_requirements,
                bounded_capability_state: "planning_profile_review_gated".to_string(),
                continuation_category: NowHeaderBucketKindData::ReviewApply,
                open_target: thread_open_target(thread_type).to_string(),
            })
        }
        "reflow_edit" | "day_plan_apply" => {
            let mut review_requirements = Vec::new();
            if matches!(
                string_field(metadata, "proposal_state").as_deref(),
                Some("staged" | "approved")
            ) {
                review_requirements.push(
                    "Schedule changes remain review-gated until the bounded proposal is explicitly applied."
                        .to_string(),
                );
            }
            let mut continuation_context = serde_json::Map::new();
            for key in [
                "source",
                "trigger",
                "severity",
                "summary",
                "context_computed_at",
            ] {
                if let Some(value) = metadata.get(key).cloned() {
                    continuation_context.insert(key.to_string(), value);
                }
            }
            if let Some(value) = metadata.get("preview_lines").cloned() {
                continuation_context.insert("preview_lines".to_string(), value);
            }
            Some(ThreadContinuationData {
                escalation_reason: if thread_type == "reflow_edit" {
                    "This reflow needs bounded manual shaping or explicit review in Threads."
                        .to_string()
                } else {
                    "This day-plan change remains in Threads until the bounded proposal is reviewed."
                        .to_string()
                },
                continuation_context: Value::Object(continuation_context),
                review_requirements,
                bounded_capability_state: "schedule_review_gated".to_string(),
                continuation_category: thread_category(thread_type),
                open_target: thread_open_target(thread_type).to_string(),
            })
        }
        _ => None,
    }
}

pub(crate) fn thread_category(thread_type: &str) -> NowHeaderBucketKindData {
    match thread_type {
        "assistant_proposal" | "planning_profile_edit" => NowHeaderBucketKindData::ReviewApply,
        "reflow_edit" | "day_plan_apply" => NowHeaderBucketKindData::Reflow,
        "raw_capture" | "day_thread" => NowHeaderBucketKindData::FollowUp,
        _ => NowHeaderBucketKindData::FollowUp,
    }
}

pub(crate) fn thread_open_target(thread_type: &str) -> &'static str {
    match thread_type {
        "raw_capture" => "raw_capture",
        "day_thread" => "day_thread",
        _ => "thread",
    }
}

pub(crate) async fn conversation_continuation_data(
    storage: &vel_storage::Storage,
    conversation_id: &str,
) -> Result<Option<ConversationContinuationData>, AppError> {
    let thread_ids = storage
        .list_threads_linking_entity("conversation", conversation_id, "continues")
        .await?;
    let Some(thread_id) = thread_ids.last() else {
        return Ok(None);
    };
    let Some((thread_id, thread_type, _title, _status, metadata_json, _created_at, _updated_at)) =
        storage.get_thread_by_id(thread_id).await?
    else {
        return Ok(None);
    };
    let metadata = parse_thread_metadata(&metadata_json);
    let Some(continuation) = thread_continuation_data(&thread_type, metadata.as_ref()) else {
        return Ok(None);
    };
    Ok(Some(ConversationContinuationData {
        thread_id,
        thread_type: thread_type.clone(),
        lifecycle_stage: proposal_thread_lifecycle_stage(&thread_type, metadata.as_ref()),
        continuation,
    }))
}

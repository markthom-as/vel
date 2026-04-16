use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;

use crate::{
    AssistantActionProposalData, CommitmentData, ConversationData, DailyLoopSessionData,
    EndOfDayData, MessageData, NowDockedInputIntentData, NowHeaderBucketKindData,
    PlanningProfileEditProposalData, UnixSeconds,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssistantEntryRouteTargetData {
    Inbox,
    Threads,
    Inline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantEntryVoiceProvenanceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_device: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript_origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recorded_at: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_captured_at: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queued_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantEntryAttachmentKindData {
    File,
    Image,
    Video,
    Audio,
    Link,
    Markdown,
    Person,
    Event,
    Task,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantEntryAttachmentData {
    pub kind: AssistantEntryAttachmentKindData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantEntryRequest {
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<NowDockedInputIntentData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AssistantEntryAttachmentData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<AssistantEntryVoiceProvenanceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantEntryResponse {
    pub route_target: AssistantEntryRouteTargetData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_intent: Option<NowDockedInputIntentData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation_category: Option<NowHeaderBucketKindData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow_up: Option<AssistantEntryFollowUpData>,
    pub user_message: MessageData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assistant_message: Option<MessageData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assistant_error: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub assistant_error_retryable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assistant_context: Option<AssistantContextData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proposal: Option<AssistantActionProposalData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planning_profile_proposal: Option<PlanningProfileEditProposalData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daily_loop_session: Option<DailyLoopSessionData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_of_day: Option<EndOfDayData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantEntryFollowUpData {
    pub intervention_id: String,
    pub message_id: String,
    pub conversation_id: String,
    pub kind: String,
    pub state: String,
    pub surfaced_at: UnixSeconds,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snoozed_until: Option<UnixSeconds>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallContextSourceCountData {
    pub source_kind: vel_core::SemanticSourceKind,
    pub count: u32,
}

impl From<vel_core::RecallContextSourceCount> for RecallContextSourceCountData {
    fn from(value: vel_core::RecallContextSourceCount) -> Self {
        Self {
            source_kind: value.source_kind,
            count: value.count,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallContextHitData {
    pub record_id: vel_core::SemanticRecordId,
    pub source_kind: vel_core::SemanticSourceKind,
    pub source_id: String,
    pub snippet: String,
    pub lexical_score: f32,
    pub semantic_score: f32,
    pub combined_score: f32,
    pub provenance: vel_core::SemanticProvenance,
}

impl From<vel_core::RecallContextHit> for RecallContextHitData {
    fn from(value: vel_core::RecallContextHit) -> Self {
        Self {
            record_id: value.record_id,
            source_kind: value.source_kind,
            source_id: value.source_id,
            snippet: value.snippet,
            lexical_score: value.lexical_score,
            semantic_score: value.semantic_score,
            combined_score: value.combined_score,
            provenance: value.provenance,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallContextData {
    pub query_text: String,
    pub hit_count: u32,
    #[serde(default)]
    pub source_counts: Vec<RecallContextSourceCountData>,
    #[serde(default)]
    pub hits: Vec<RecallContextHitData>,
}

impl From<vel_core::RecallContextPack> for RecallContextData {
    fn from(value: vel_core::RecallContextPack) -> Self {
        Self {
            query_text: value.query_text,
            hit_count: value.hit_count,
            source_counts: value.source_counts.into_iter().map(Into::into).collect(),
            hits: value.hits.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantContextData {
    pub query_text: String,
    pub summary: String,
    #[serde(default)]
    pub focus_lines: Vec<String>,
    #[serde(default)]
    pub commitments: Vec<CommitmentData>,
    pub recall: RecallContextData,
}

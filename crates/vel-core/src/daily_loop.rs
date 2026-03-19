use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;

pub const DAILY_LOOP_MAX_QUESTIONS: u8 = 3;
pub const DAILY_LOOP_MAX_COMMITMENTS: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DailyLoopSessionId(pub(crate) String);

impl DailyLoopSessionId {
    pub fn new() -> Self {
        Self(format!("dls_{}", Uuid::new_v4().simple()))
    }
}

impl Default for DailyLoopSessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for DailyLoopSessionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for DailyLoopSessionId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for DailyLoopSessionId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopPhase {
    MorningOverview,
    Standup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopStatus {
    Active,
    WaitingForInput,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopStartSource {
    Manual,
    Automatic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopSurface {
    Cli,
    Web,
    AppleVoice,
    AppleText,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopTurnAction {
    Submit,
    Skip,
    Resume,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopTurnState {
    InProgress,
    WaitingForInput,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopQuestionBudget(u8);

impl DailyLoopQuestionBudget {
    pub fn new(value: u8) -> Result<Self, crate::VelCoreError> {
        if value > DAILY_LOOP_MAX_QUESTIONS {
            return Err(crate::VelCoreError::Validation(format!(
                "daily loop question budget must be <= {}",
                DAILY_LOOP_MAX_QUESTIONS
            )));
        }
        Ok(Self(value))
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

impl Default for DailyLoopQuestionBudget {
    fn default() -> Self {
        Self(DAILY_LOOP_MAX_QUESTIONS)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopCommitmentLimit(usize);

impl DailyLoopCommitmentLimit {
    pub fn new(value: usize) -> Result<Self, crate::VelCoreError> {
        if value == 0 || value > DAILY_LOOP_MAX_COMMITMENTS {
            return Err(crate::VelCoreError::Validation(format!(
                "daily loop commitment limit must be between 1 and {}",
                DAILY_LOOP_MAX_COMMITMENTS
            )));
        }
        Ok(Self(value))
    }

    pub fn get(self) -> usize {
        self.0
    }
}

impl Default for DailyLoopCommitmentLimit {
    fn default() -> Self {
        Self(DAILY_LOOP_MAX_COMMITMENTS)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopStartMetadata {
    pub source: DailyLoopStartSource,
    pub surface: DailyLoopSurface,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopStartRequest {
    pub phase: DailyLoopPhase,
    pub session_date: String,
    pub start: DailyLoopStartMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopTurnRequest {
    pub session_id: DailyLoopSessionId,
    pub action: DailyLoopTurnAction,
    pub response_text: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopPromptKind {
    IntentQuestion,
    CommitmentReduction,
    ConstraintCheck,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopPrompt {
    pub prompt_id: String,
    pub kind: DailyLoopPromptKind,
    pub text: String,
    pub ordinal: u8,
    pub allow_skip: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MorningFrictionCallout {
    pub label: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MorningIntentSignal {
    MustDoHint { text: String },
    FocusIntent { text: String },
    MeetingDoubt { text: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MorningOverviewState {
    pub snapshot: String,
    pub friction_callouts: Vec<MorningFrictionCallout>,
    pub signals: Vec<MorningIntentSignal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyStandupBucket {
    Must,
    Should,
    Stretch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyCommitmentDraft {
    pub title: String,
    pub bucket: DailyStandupBucket,
    pub source_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyDeferredTask {
    pub title: String,
    pub source_ref: Option<String>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyFocusBlockProposal {
    pub label: String,
    pub start_at: OffsetDateTime,
    pub end_at: OffsetDateTime,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyStandupOutcome {
    pub commitments: Vec<DailyCommitmentDraft>,
    pub deferred_tasks: Vec<DailyDeferredTask>,
    pub confirmed_calendar: Vec<String>,
    pub focus_blocks: Vec<DailyFocusBlockProposal>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case")]
pub enum DailyLoopSessionState {
    MorningOverview(MorningOverviewState),
    Standup(DailyStandupOutcome),
}

impl From<MorningOverviewState> for DailyLoopSessionState {
    fn from(value: MorningOverviewState) -> Self {
        Self::MorningOverview(value)
    }
}

impl From<DailyStandupOutcome> for DailyLoopSessionState {
    fn from(value: DailyStandupOutcome) -> Self {
        Self::Standup(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case")]
pub enum DailyLoopSessionOutcome {
    MorningOverview { signals: Vec<MorningIntentSignal> },
    Standup(DailyStandupOutcome),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyLoopSession {
    pub id: DailyLoopSessionId,
    pub session_date: String,
    pub phase: DailyLoopPhase,
    pub status: DailyLoopStatus,
    pub start: DailyLoopStartMetadata,
    pub turn_state: DailyLoopTurnState,
    pub current_prompt: Option<DailyLoopPrompt>,
    pub state: DailyLoopSessionState,
    pub outcome: Option<DailyLoopSessionOutcome>,
}

#[cfg(test)]
mod tests {
    use super::{
        DailyLoopCommitmentLimit, DailyLoopQuestionBudget, DAILY_LOOP_MAX_COMMITMENTS,
        DAILY_LOOP_MAX_QUESTIONS,
    };

    #[test]
    fn question_budget_rejects_more_than_three_questions() {
        assert_eq!(
            DailyLoopQuestionBudget::new(DAILY_LOOP_MAX_QUESTIONS)
                .expect("max budget should be valid")
                .get(),
            3
        );
        assert!(DailyLoopQuestionBudget::new(DAILY_LOOP_MAX_QUESTIONS + 1).is_err());
    }

    #[test]
    fn commitment_limit_rejects_zero_or_more_than_three() {
        assert!(DailyLoopCommitmentLimit::new(0).is_err());
        assert_eq!(
            DailyLoopCommitmentLimit::new(DAILY_LOOP_MAX_COMMITMENTS)
                .expect("max limit should be valid")
                .get(),
            3
        );
        assert!(DailyLoopCommitmentLimit::new(DAILY_LOOP_MAX_COMMITMENTS + 1).is_err());
    }
}

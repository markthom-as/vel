use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrivacyClass {
    Private,
    Work,
    Sensitive,
    DoNotRecord,
}

impl Default for PrivacyClass {
    fn default() -> Self {
        Self::Private
    }
}

impl Display for PrivacyClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Private => "private",
            Self::Work => "work",
            Self::Sensitive => "sensitive",
            Self::DoNotRecord => "do_not_record",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncClass {
    Hot,
    Warm,
    Cold,
}

impl Default for SyncClass {
    fn default() -> Self {
        Self::Warm
    }
}

impl Display for SyncClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Hot => "hot",
            Self::Warm => "warm",
            Self::Cold => "cold",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
}

impl Display for JobStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionFeedback {
    Dismiss,
    Correct,
    NeverShowAgain,
    Train,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CaptureId(String);

impl CaptureId {
    pub fn new() -> Self {
        Self(format!("cap_{}", Uuid::new_v4().simple()))
    }
}

impl Default for CaptureId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for CaptureId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for CaptureId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for CaptureId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JobId(String);

impl JobId {
    pub fn new() -> Self {
        Self(format!("job_{}", Uuid::new_v4().simple()))
    }
}

impl Default for JobId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for JobId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for JobId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtifactId(String);

impl ArtifactId {
    pub fn new() -> Self {
        Self(format!("art_{}", Uuid::new_v4().simple()))
    }
}

impl Default for ArtifactId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ArtifactId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for ArtifactId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for ArtifactId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureRecord {
    pub capture_id: CaptureId,
    pub capture_type: String,
    pub content_text: String,
    pub occurred_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub source_device: Option<String>,
    pub privacy_class: PrivacyClass,
    pub metadata_json: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
pub enum VelCoreError {
    #[error("validation error: {0}")]
    Validation(String),
}

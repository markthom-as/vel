use crate::command::{DomainOperation, TypedTarget};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParseMode {
    Deterministic,
    Repaired,
}

impl Display for ParseMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Deterministic => "deterministic",
            Self::Repaired => "repaired",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandConfidenceBand {
    Low,
    Medium,
    High,
}

impl Display for CommandConfidenceBand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionConfidence {
    pub field: String,
    pub band: CommandConfidenceBand,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentResolution {
    #[serde(default = "default_json_object")]
    pub explicit: Value,
    #[serde(default = "default_json_object")]
    pub inferred: Value,
    #[serde(default)]
    pub assumptions: Vec<String>,
    #[serde(default)]
    pub confidence: Vec<ResolutionConfidence>,
    #[serde(default)]
    pub requires_confirmation: bool,
}

impl Default for IntentResolution {
    fn default() -> Self {
        Self {
            explicit: default_json_object(),
            inferred: default_json_object(),
            assumptions: Vec::new(),
            confidence: Vec::new(),
            requires_confirmation: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionMeta {
    pub parser: ParseMode,
    #[serde(default)]
    pub model_assisted: bool,
    #[serde(default)]
    pub confirmation_required: bool,
}

impl Default for ResolutionMeta {
    fn default() -> Self {
        Self {
            parser: ParseMode::Deterministic,
            model_assisted: false,
            confirmation_required: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedCommand {
    pub operation: DomainOperation,
    #[serde(default)]
    pub targets: Vec<TypedTarget>,
    #[serde(default = "default_json_object")]
    pub inferred: Value,
    #[serde(default)]
    pub assumptions: Vec<String>,
    #[serde(default)]
    pub resolution: ResolutionMeta,
}

impl Default for ResolvedCommand {
    fn default() -> Self {
        Self {
            operation: DomainOperation::Inspect,
            targets: Vec::new(),
            inferred: default_json_object(),
            assumptions: Vec::new(),
            resolution: ResolutionMeta::default(),
        }
    }
}

fn default_json_object() -> Value {
    Value::Object(Default::default())
}

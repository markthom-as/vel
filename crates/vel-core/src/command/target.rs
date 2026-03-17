use crate::command::DomainKind;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

/// Selector variants for resolving a command target without a raw ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum TargetSelector {
    Id(String),
    Alias(String),
    Latest,
    Open,
    DueToday,
    Custom(String),
}

impl Display for TargetSelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(value) => write!(f, "id:{value}"),
            Self::Alias(value) => write!(f, "alias:{value}"),
            Self::Latest => f.write_str("latest"),
            Self::Open => f.write_str("open"),
            Self::DueToday => f.write_str("due_today"),
            Self::Custom(value) => f.write_str(value),
        }
    }
}

/// A typed target for command execution and explainability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedTarget {
    pub kind: DomainKind,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub selector: Option<TargetSelector>,
    #[serde(default = "default_attributes")]
    pub attributes: Value,
}

impl TypedTarget {
    pub fn new(kind: DomainKind) -> Self {
        Self {
            kind,
            id: None,
            selector: None,
            attributes: default_attributes(),
        }
    }
}

impl Default for TypedTarget {
    fn default() -> Self {
        Self::new(DomainKind::Context)
    }
}

fn default_attributes() -> Value {
    Value::Object(Default::default())
}

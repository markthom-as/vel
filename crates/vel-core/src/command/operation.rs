use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// High-level operation families for resolved command execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainOperation {
    Create,
    Inspect,
    List,
    Update,
    Link,
    Explain,
    Execute,
}

impl Display for DomainOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Create => "create",
            Self::Inspect => "inspect",
            Self::List => "list",
            Self::Update => "update",
            Self::Link => "link",
            Self::Explain => "explain",
            Self::Execute => "execute",
        };
        f.write_str(value)
    }
}

/// Explicit relation operation semantics for cross-type linking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationOperation {
    Link,
    Attach,
    Detach,
}

impl Display for RelationOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Link => "link",
            Self::Attach => "attach",
            Self::Detach => "detach",
        };
        f.write_str(value)
    }
}

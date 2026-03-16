//! Provenance / refs: relation types for run→artifact, artifact→capture, etc.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefRelationType {
    GeneratedFrom,
    DerivedFrom,
    AttachedTo,
}

impl Display for RefRelationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::GeneratedFrom => "generated_from",
            Self::DerivedFrom => "derived_from",
            Self::AttachedTo => "attached_to",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for RefRelationType {
    type Err = crate::VelCoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "generated_from" => Ok(Self::GeneratedFrom),
            "derived_from" => Ok(Self::DerivedFrom),
            "attached_to" => Ok(Self::AttachedTo),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown relation type: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ref {
    pub id: String,
    pub from_type: String,
    pub from_id: String,
    pub to_type: String,
    pub to_id: String,
    pub relation_type: RefRelationType,
    pub created_at: OffsetDateTime,
}

impl Ref {
    pub fn new(
        from_type: impl Into<String>,
        from_id: impl Into<String>,
        to_type: impl Into<String>,
        to_id: impl Into<String>,
        relation_type: RefRelationType,
    ) -> Self {
        Self {
            id: format!("ref_{}", Uuid::new_v4().simple()),
            from_type: from_type.into(),
            from_id: from_id.into(),
            to_type: to_type.into(),
            to_id: to_id.into(),
            relation_type,
            created_at: time::OffsetDateTime::now_utc(),
        }
    }
}

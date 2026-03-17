use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionMode {
    Proceed,
    AskUser,
    AskAgent,
    Defer,
    SilentHold,
}

impl Display for ResolutionMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Proceed => "proceed",
            Self::AskUser => "ask_user",
            Self::AskAgent => "ask_agent",
            Self::Defer => "defer",
            Self::SilentHold => "silent_hold",
        };
        f.write_str(value)
    }
}

impl FromStr for ResolutionMode {
    type Err = crate::VelCoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "proceed" => Ok(Self::Proceed),
            "ask_user" => Ok(Self::AskUser),
            "ask_agent" => Ok(Self::AskAgent),
            "defer" => Ok(Self::Defer),
            "silent_hold" => Ok(Self::SilentHold),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown resolution mode: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UncertaintyStatus {
    Open,
    Resolved,
}

impl Display for UncertaintyStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Open => "open",
            Self::Resolved => "resolved",
        };
        f.write_str(value)
    }
}

impl FromStr for UncertaintyStatus {
    type Err = crate::VelCoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "open" => Ok(Self::Open),
            "resolved" => Ok(Self::Resolved),
            _ => Err(crate::VelCoreError::Validation(format!(
                "unknown uncertainty status: {}",
                value
            ))),
        }
    }
}

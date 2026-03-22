use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

pub use crate::types::EventId;

macro_rules! prefixed_id_type {
    ($name:ident, $prefix:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            pub fn new() -> Self {
                Self(format!("{}_{}", $prefix, Uuid::new_v4().simple()))
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}

prefixed_id_type!(TaskId, "task");
prefixed_id_type!(WorkflowId, "workflow");
prefixed_id_type!(ModuleId, "module");
prefixed_id_type!(SkillId, "skill");
prefixed_id_type!(ToolId, "tool");
prefixed_id_type!(IntegrationAccountId, "integration_account");
prefixed_id_type!(SyncLinkId, "sync_link");
prefixed_id_type!(WriteIntentId, "write_intent");

#[cfg(test)]
mod tests {
    use super::{
        IntegrationAccountId, ModuleId, SkillId, SyncLinkId, TaskId, ToolId, WorkflowId,
        WriteIntentId,
    };

    #[test]
    fn canonical_ids_use_expected_prefixes() {
        let ids = [
            TaskId::new().to_string(),
            WorkflowId::new().to_string(),
            ModuleId::new().to_string(),
            SkillId::new().to_string(),
            ToolId::new().to_string(),
            IntegrationAccountId::new().to_string(),
            SyncLinkId::new().to_string(),
            WriteIntentId::new().to_string(),
        ];

        assert!(ids[0].starts_with("task_"));
        assert!(ids[1].starts_with("workflow_"));
        assert!(ids[2].starts_with("module_"));
        assert!(ids[3].starts_with("skill_"));
        assert!(ids[4].starts_with("tool_"));
        assert!(ids[5].starts_with("integration_account_"));
        assert!(ids[6].starts_with("sync_link_"));
        assert!(ids[7].starts_with("write_intent_"));
    }
}

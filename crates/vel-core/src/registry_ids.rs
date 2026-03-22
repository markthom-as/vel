use serde::{Deserialize, Serialize};

pub const MODULE_INTEGRATION_TODOIST: &str = "module.integration.todoist";
pub const MODULE_INTEGRATION_GOOGLE_CALENDAR: &str = "module.integration.google-calendar";
pub const SKILL_CORE_DAILY_BRIEF: &str = "skill.core.daily-brief";
pub const TOOL_OBJECT_GET: &str = "tool.object.get";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryKind {
    Module,
    Skill,
    Tool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRegistryId {
    pub kind: RegistryKind,
    pub namespace: String,
    pub slug: String,
}

impl SemanticRegistryId {
    pub fn new(kind: RegistryKind, namespace: impl Into<String>, slug: impl Into<String>) -> Self {
        Self {
            kind,
            namespace: namespace.into(),
            slug: slug.into(),
        }
    }

    pub fn as_string(&self) -> String {
        let prefix = match self.kind {
            RegistryKind::Module => "module",
            RegistryKind::Skill => "skill",
            RegistryKind::Tool => "tool",
        };
        format!("{prefix}.{}.{}", self.namespace, self.slug)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RegistryKind, SemanticRegistryId, MODULE_INTEGRATION_GOOGLE_CALENDAR,
        MODULE_INTEGRATION_TODOIST, SKILL_CORE_DAILY_BRIEF, TOOL_OBJECT_GET,
    };

    #[test]
    fn semantic_registry_ids_keep_canonical_module_skill_and_tool_namespaces() {
        assert_eq!(
            SemanticRegistryId::new(RegistryKind::Module, "integration", "todoist").as_string(),
            MODULE_INTEGRATION_TODOIST
        );
        assert_eq!(
            SemanticRegistryId::new(RegistryKind::Module, "integration", "google-calendar")
                .as_string(),
            MODULE_INTEGRATION_GOOGLE_CALENDAR
        );
        assert_eq!(
            SemanticRegistryId::new(RegistryKind::Skill, "core", "daily-brief").as_string(),
            SKILL_CORE_DAILY_BRIEF
        );
        assert_eq!(
            SemanticRegistryId::new(RegistryKind::Tool, "object", "get").as_string(),
            TOOL_OBJECT_GET
        );
    }
}

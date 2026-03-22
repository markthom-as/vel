use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantScope {
    Workspace,
    Module(String),
    IntegrationAccount(String),
    Object(String),
    Action(String),
    Execution(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Grant {
    pub id: String,
    pub scope: Vec<GrantScope>,
    pub capabilities: Vec<String>,
    pub durable: bool,
    pub run_scoped: bool,
    pub read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrantRequest {
    pub action_name: String,
    pub capability: String,
    pub object_ids: Vec<String>,
    pub durable: bool,
    pub run_scoped: bool,
}

#[cfg(test)]
mod tests {
    use super::{Grant, GrantRequest, GrantScope};

    #[test]
    fn grant_model_tracks_scope_and_lifetime_posture() {
        let grant = Grant {
            id: "grant_01".to_string(),
            scope: vec![GrantScope::Workspace, GrantScope::Action("object.update".to_string())],
            capabilities: vec!["object.write".to_string()],
            durable: false,
            run_scoped: true,
            read_only: false,
        };
        let request = GrantRequest {
            action_name: "object.update".to_string(),
            capability: "object.write".to_string(),
            object_ids: vec!["task_01".to_string()],
            durable: false,
            run_scoped: true,
        };

        assert!(grant.scope.len() >= 2);
        assert_eq!(request.capability, "object.write");
    }
}


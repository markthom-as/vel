use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowContextValueKind {
    CanonicalObject,
    RuntimeValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowObjectRef {
    pub object_ref: String,
    pub object_type: String,
    pub expected_revision: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRuntimeValue {
    pub value_type: String,
    pub value: JsonValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WorkflowContextValue {
    CanonicalObject(WorkflowObjectRef),
    RuntimeValue(WorkflowRuntimeValue),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowBinding {
    pub binding_name: String,
    pub value: WorkflowContextValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowContext {
    pub workflow_id: String,
    pub bindings: Vec<WorkflowBinding>,
}

impl WorkflowContext {
    pub fn binding(&self, binding_name: &str) -> Option<&WorkflowBinding> {
        self.bindings
            .iter()
            .find(|binding| binding.binding_name == binding_name)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.workflow_id.trim().is_empty() {
            return Err("workflow context missing workflow_id".to_string());
        }
        if self.bindings.is_empty() {
            return Err("workflow context requires at least one context binding".to_string());
        }

        for binding in &self.bindings {
            if binding.binding_name.trim().is_empty() {
                return Err("workflow context binding_name must not be empty".to_string());
            }

            match &binding.value {
                WorkflowContextValue::CanonicalObject(object) => {
                    if object.object_ref.trim().is_empty() {
                        return Err(format!(
                            "workflow context binding {} missing object_ref",
                            binding.binding_name
                        ));
                    }
                    if object.object_type.trim().is_empty() {
                        return Err(format!(
                            "workflow context binding {} missing canonical object type",
                            binding.binding_name
                        ));
                    }
                }
                WorkflowContextValue::RuntimeValue(runtime) => {
                    if runtime.value_type.trim().is_empty() {
                        return Err(format!(
                            "workflow context binding {} missing runtime value_type",
                            binding.binding_name
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        WorkflowBinding, WorkflowContext, WorkflowContextValue, WorkflowObjectRef,
        WorkflowRuntimeValue,
    };
    use serde_json::json;

    #[test]
    fn workflow_context_validates_canonical_object_and_runtime_bindings() {
        let context = WorkflowContext {
            workflow_id: "workflow_01brief".to_string(),
            bindings: vec![
                WorkflowBinding {
                    binding_name: "task".to_string(),
                    value: WorkflowContextValue::CanonicalObject(WorkflowObjectRef {
                        object_ref: "task_01".to_string(),
                        object_type: "task".to_string(),
                        expected_revision: Some(1),
                    }),
                },
                WorkflowBinding {
                    binding_name: "window".to_string(),
                    value: WorkflowContextValue::RuntimeValue(WorkflowRuntimeValue {
                        value_type: "time_window".to_string(),
                        value: json!({"start":"09:00","end":"10:00"}),
                    }),
                },
            ],
        };

        assert!(context.validate().is_ok());
        assert_eq!(context.binding("task").unwrap().binding_name, "task");
    }

    #[test]
    fn workflow_context_rejects_empty_object_ref() {
        let context = WorkflowContext {
            workflow_id: "workflow_01brief".to_string(),
            bindings: vec![WorkflowBinding {
                binding_name: "task".to_string(),
                value: WorkflowContextValue::CanonicalObject(WorkflowObjectRef {
                    object_ref: String::new(),
                    object_type: "task".to_string(),
                    expected_revision: None,
                }),
            }],
        };

        assert!(context
            .validate()
            .unwrap_err()
            .contains("missing object_ref"));
    }
}

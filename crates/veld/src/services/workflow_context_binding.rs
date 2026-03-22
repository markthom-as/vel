use std::collections::BTreeMap;

use sqlx::SqlitePool;
use vel_core::{WorkflowContext, WorkflowContextValue};
use vel_storage::{get_canonical_object, CanonicalObjectRecord};

use crate::errors::AppError;

#[derive(Debug, Clone, PartialEq)]
pub struct BoundWorkflowContext {
    pub workflow_id: String,
    pub canonical_objects: BTreeMap<String, CanonicalObjectRecord>,
    pub runtime_values: BTreeMap<String, serde_json::Value>,
}

pub struct ContextBinding;

impl ContextBinding {
    pub async fn bind(
        pool: &SqlitePool,
        context: &WorkflowContext,
    ) -> Result<BoundWorkflowContext, AppError> {
        context.validate().map_err(AppError::bad_request)?;

        let mut canonical_objects = BTreeMap::new();
        let mut runtime_values = BTreeMap::new();

        for binding in &context.bindings {
            match &binding.value {
                WorkflowContextValue::CanonicalObject(object) => {
                    let record = get_canonical_object(pool, &object.object_ref)
                        .await
                        .map_err(|error| AppError::internal(error.to_string()))?
                        .ok_or_else(|| {
                            AppError::not_found(format!(
                                "canonical object {} missing for context binding {}",
                                object.object_ref, binding.binding_name
                            ))
                        })?;

                    if record.object_type != object.object_type {
                        return Err(AppError::bad_request(format!(
                            "canonical object {} for context binding {} has type {} but expected {}",
                            object.object_ref, binding.binding_name, record.object_type, object.object_type
                        )));
                    }

                    if let Some(expected_revision) = object.expected_revision {
                        if record.revision != expected_revision {
                            return Err(AppError::bad_request(format!(
                                "canonical object {} for context binding {} has revision {} but expected {}",
                                object.object_ref, binding.binding_name, record.revision, expected_revision
                            )));
                        }
                    }

                    canonical_objects.insert(binding.binding_name.clone(), record);
                }
                WorkflowContextValue::RuntimeValue(runtime) => {
                    runtime_values.insert(binding.binding_name.clone(), runtime.value.clone());
                }
            }
        }

        Ok(BoundWorkflowContext {
            workflow_id: context.workflow_id.clone(),
            canonical_objects,
            runtime_values,
        })
    }
}

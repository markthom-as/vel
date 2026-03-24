use vel_core::{Grant, GrantRequest, GrantScope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrantResolverError {
    GrantMissing(String),
}

#[derive(Debug, Default)]
pub struct GrantResolver;

impl GrantResolver {
    pub fn resolve(
        &self,
        grant: &Grant,
        request: &GrantRequest,
    ) -> Result<Grant, GrantResolverError> {
        if !grant
            .capabilities
            .iter()
            .any(|capability| capability == &request.capability)
        {
            return Err(GrantResolverError::GrantMissing(format!(
                "GrantMissing capability {}",
                request.capability
            )));
        }

        if request.durable && !grant.durable {
            return Err(GrantResolverError::GrantMissing(
                "GrantMissing durable escalation".to_string(),
            ));
        }

        if request.run_scoped && !grant.run_scoped && !grant.durable {
            return Err(GrantResolverError::GrantMissing(
                "GrantMissing run-scoped escalation".to_string(),
            ));
        }

        if !grant.scope.iter().any(
            |scope| matches!(scope, GrantScope::Action(action) if action == &request.action_name),
        ) {
            return Err(GrantResolverError::GrantMissing(format!(
                "GrantMissing action {}",
                request.action_name
            )));
        }

        let mut narrowed_scope = vec![GrantScope::Action(request.action_name.clone())];
        narrowed_scope.extend(request.object_ids.iter().cloned().map(GrantScope::Object));

        Ok(Grant {
            id: format!("{}_narrow", grant.id),
            scope: narrowed_scope,
            capabilities: vec![request.capability.clone()],
            durable: request.durable,
            run_scoped: request.run_scoped,
            read_only: grant.read_only,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{GrantResolver, GrantResolverError};
    use vel_core::{Grant, GrantRequest, GrantScope};

    #[test]
    fn grant_resolver_narrows_scope_without_widening_authority() {
        let resolver = GrantResolver;
        let grant = Grant {
            id: "grant_01".to_string(),
            scope: vec![
                GrantScope::Workspace,
                GrantScope::Action("object.update".to_string()),
            ],
            capabilities: vec!["object.write".to_string()],
            durable: false,
            run_scoped: true,
            read_only: false,
        };

        let narrowed = resolver
            .resolve(
                &grant,
                &GrantRequest {
                    action_name: "object.update".to_string(),
                    capability: "object.write".to_string(),
                    object_ids: vec!["task_01".to_string()],
                    durable: false,
                    run_scoped: true,
                },
            )
            .unwrap();

        assert_eq!(narrowed.capabilities, vec!["object.write".to_string()]);
        assert!(narrowed
            .scope
            .iter()
            .any(|scope| matches!(scope, GrantScope::Object(id) if id == "task_01")));
        assert!(!narrowed.durable);
    }

    #[test]
    fn grant_resolver_rejects_attempts_to_widen_to_durable_scope() {
        let resolver = GrantResolver;
        let grant = Grant {
            id: "grant_02".to_string(),
            scope: vec![GrantScope::Action("object.update".to_string())],
            capabilities: vec!["object.write".to_string()],
            durable: false,
            run_scoped: true,
            read_only: false,
        };

        let result = resolver.resolve(
            &grant,
            &GrantRequest {
                action_name: "object.update".to_string(),
                capability: "object.write".to_string(),
                object_ids: vec!["task_02".to_string()],
                durable: true,
                run_scoped: true,
            },
        );

        assert!(matches!(result, Err(GrantResolverError::GrantMissing(_))));
    }
}

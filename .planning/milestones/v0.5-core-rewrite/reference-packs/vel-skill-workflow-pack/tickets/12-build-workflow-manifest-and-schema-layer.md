# Build workflow manifest and schema layer

## Scope
- Add `workflow.yaml` parsing
- Add `workflow.schema.json`
- Validate manifests at load time
- Expose normalized model in runtime crate

## Acceptance criteria
- invalid manifests fail with actionable errors
- runtime can inspect name, version, triggers, context binding, and steps

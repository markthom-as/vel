# 11. Data Storage and Runtime

## 11.1 Runtime recommendation

A strong fit is:

- **Rust** for core runtime, schema validation, action dispatch, policy, and module loading
- optional hook languages for skills/workflows/modules:
  - TypeScript/Node
  - Python
  - shell
  - later WASM

## 11.2 Why Rust is a good core fit

- strong typing for schema/action/policy systems
- predictable CLI embedding
- good serialization support
- good performance for validation/query/sync paths
- future portability across desktop/native/Tauri/web-ish surfaces

## 11.3 Suggested runtime components

- `object_registry`
- `schema_engine`
- `relation_graph`
- `action_dispatcher`
- `policy_engine`
- `audit_log`
- `template_resolver`
- `workflow_runtime`
- `skill_runtime`
- `module_loader`
- `adapter_runtime`
- `query_engine`

## 11.4 Storage suggestions

At minimum, support:

- canonical object storage
- relation storage
- audit/event log
- workflow run logs
- module registry state
- source mapping state
- config/policy state

Could be backed by SQLite initially with careful schema design.
That is more than enough to start and much less annoying than prematurely cosplaying distributed systems adulthood.

## 11.5 Suggested storage partitions

- `objects`
- `object_versions` or mutation snapshots
- `relations`
- `source_mappings`
- `audit_events`
- `workflow_runs`
- `skill_runs`
- `policies`
- `grants`
- `modules`
- `templates`

## 11.6 IDs and versioning

Use stable object IDs and versioned schemas.
Need:

- object IDs
- schema version IDs
- action IDs
- run IDs
- source mapping IDs
- audit event IDs

## 11.7 Migration discipline

Schemas, templates, module assets, and adapter mappings will evolve.
Need explicit migration support, not wishful thinking.

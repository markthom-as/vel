---
title: Typed CurrentContext & Schema-on-Write Transition
status: in-progress
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
labels:
  - vel-core
  - veld
  - type-safety
---

Transition `CurrentContext` from a free-form `serde_json::Value` to a versioned, strictly typed Rust struct in `vel-core`, enforcing a schema-on-write boundary.

## Technical Details
- **Core Struct**: Define `CurrentContextV1` in `vel-core/src/context.rs`.
- **Fields**: Include `mode`, `morning_state`, `meds_status`, `attention_state`, `drift_type`, etc.
- **Migration**: Implement a `ContextMigrator` that converts old JSON data to the new struct format upon reading from SQLite.
- **Enforcement**: Update all inference logic to write to the struct first, which is then serialized to JSON for storage.

## Acceptance Criteria
- `veld` uses a typed struct for all current context logic.
- Compiler errors occur if a non-existent context field is accessed.
- Migration logic is tested and handles legacy JSON formats.
- All inference-related tests pass.

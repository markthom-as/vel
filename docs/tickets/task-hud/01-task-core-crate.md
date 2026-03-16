---
title: Create vel-task-core crate
status: ready
owner: agent
priority: P0
area: vel-task-hud
---

# Goal
Create the canonical task domain crate for Vel.

## Scope
Implement:
- `Task`
- enums for status, kind, source, priority, urgency, decay, visibility
- serde support
- DB-facing types if needed
- conversion helpers from storage rows to domain objects

## Suggested paths
```text
crates/vel-task-core/src/
  lib.rs
  task.rs
  task_status.rs
  task_kind.rs
  task_source.rs
  priority.rs
  urgency.rs
  decay_state.rs
  visibility_mode.rs
```

## Requirements
- Keep the model minimal but extensible.
- Avoid UI-specific fields in the domain model except policy-facing visibility hints.
- Document invariants in code comments.
- Prefer explicit enums over ad hoc strings.

## Tests
- round-trip serialization tests
- enum parse/format tests
- task constructor / validation tests if constructors exist

## Done when
- crate builds
- types are documented
- tests pass


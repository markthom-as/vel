---
title: Create vel-task-core crate
status: ready
owner: agent
priority: P0
area: vel-task-hud
---

# Goal
Decide whether Vel actually needs a new durable task domain, and only then create the narrowest core crate that fits that decision.

## Boundary rule

Do **not** assume this ticket automatically authorizes a parallel replacement for:

- commitments,
- nudges,
- threads,
- risk.

Preferred default:

- keep **commitments** as the durable actionable object,
- build HUD ranking/view-model logic on top,
- introduce a new `Task` domain only if commitment-backed HUD semantics prove insufficient.

## Scope
First:
- write down the boundary decision in the crate/module docs or ticket notes:
  - why commitments are sufficient, or
  - why they are not
- if commitments are sufficient, implement a thin commitment-backed HUD core instead of a new durable task model

Only if a new domain is justified, implement:
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
- Do not duplicate fields or semantics already canonically owned by commitments/risk unless the boundary note explains why duplication is necessary.
- Document invariants in code comments.
- Prefer explicit enums over ad hoc strings.

## Tests
- boundary tests or assertions for any conversion layer from commitments -> HUD/task objects
- round-trip serialization tests
- enum parse/format tests
- task constructor / validation tests if constructors exist

## Done when
- the boundary decision is explicit and documented
- crate builds
- types are documented
- tests pass

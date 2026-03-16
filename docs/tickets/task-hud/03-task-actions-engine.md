---
title: Implement task actions engine
status: ready
owner: agent
priority: P0
area: vel-task-hud
---

# Goal
Provide canonical mutations on task state through a shared actions layer.

## Scope
Create `vel-task-actions` with functions like:
- `complete_task`
- `snooze_task`
- `start_task`
- `block_task`
- `unblock_task`
- `defer_task`
- `break_down_task`
- `pin_task`
- `hide_task`

## Requirements
- Every mutation should emit a task event.
- Mutations should update `last_touched_at`.
- Avoid embedding UI assumptions in action semantics.
- Prefer explicit typed inputs over freeform maps.

## Suggested API shape
```rust
pub trait TaskActionService {
    fn complete_task(&self, input: CompleteTaskInput) -> Result<Task>;
    fn snooze_task(&self, input: SnoozeTaskInput) -> Result<Task>;
    // ...
}
```

## Policy notes
- `snooze_task` should require an until-time or named duration.
- `break_down_task` may create children/subtasks later; for now it can emit a suggestion or draft children.
- `hide_task` is not delete.

## Tests
- state transition tests
- event emission tests
- invalid transition tests

## Done when
- actions crate exists
- all mutations are test-covered
- event log integration works


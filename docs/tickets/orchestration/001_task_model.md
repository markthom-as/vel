# Ticket 001 — Task Model

## Purpose
Vel must operate on explicit tasks rather than freeform prompts.

## Deliverables
Rust structs:
- Task
- TaskId
- TaskState
- TaskPriority
- IntentClass
- OutputSpec

State machine:
Received → Planned → Routed → Executing → Integrating → Completed → Failed → Blocked

## Acceptance Criteria
Tasks support:
- parent task
- dependency graph
- capability requirements
- deadline
- priority
- status transitions

## Implementation Notes
File: src/orchestrator/task.rs


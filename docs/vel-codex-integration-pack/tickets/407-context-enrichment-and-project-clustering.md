---
title: Ticket 407 - Enrich current context with project clusters, task pressure, and calendar windows
status: proposed
owner: codex
priority: critical
---

# Goal

Use normalized external state to make `current_context` actually project-aware and schedule-aware.

# Files

## Changed
- `crates/veld/src/services/inference.rs`
- `crates/vel-core/src/current_context.rs` (new if not yet created)
- `crates/vel-api-types/src/lib.rs`

# Concrete shape changes

Add sections:
- `projects.active`
- `projects.blocked`
- `projects.drifting`
- `tasks.overdue`
- `tasks.due_today`
- `tasks.unscheduled`
- `calendar.next_event`
- `calendar.prep_windows`
- `calendar.free_windows`
- `sync.sources`

## Project clustering algorithm
For each project slug:
- count open commitments
- count overdue commitments
- count upcoming events in next 7 days
- count recent signals in last 7 days

Use these to classify:
- active
- blocked
- drifting
- dormant

Keep heuristics deterministic and documented.

# Acceptance criteria

- `GET /v1/context` can answer project-aware and schedule-aware questions without additional ad hoc recomputation
- the shape is stable and typed

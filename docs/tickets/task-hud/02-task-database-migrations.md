---
title: Add task database tables and indices
status: ready
owner: agent
priority: P0
area: vel-task-hud
---

# Goal
Persist tasks and their lifecycle in Vel's primary DB.

## Scope
Add migrations for:
- `tasks`
- `task_dependencies`
- `task_context_refs`
- `task_events`

## Suggested columns
### tasks
- id
- title
- description
- status
- kind
- source
- priority
- urgency
- estimated_duration_secs
- energy_required
- deadline_at
- scheduled_for
- snoozed_until
- last_touched_at
- attention_score
- commitment_score
- lateness_risk
- decay_state
- visibility_mode
- created_at
- updated_at
- completed_at
- hidden_at
- pinned_to_hud

### task_dependencies
- task_id
- depends_on_task_id
- relation_type

### task_context_refs
- task_id
- context_type
- context_id

### task_events
- id
- task_id
- event_type
- payload_json
- created_at

## Indices
Add useful indices on:
- status
- deadline_at
- scheduled_for
- snoozed_until
- attention_score
- pinned_to_hud
- updated_at

## Notes
- Use event log as append-only lifecycle trace.
- Avoid over-normalizing too early.
- Keep migration idempotence and rollback story clean.

## Tests
- migration up/down test if the repo supports it
- repository smoke tests for insert/read/update

## Done when
- schema exists
- repositories compile
- task inserts and reads work


---
title: Ticket 408 - Represent external mutations as explicit sync proposals and operations
status: proposed
owner: codex
priority: high
---

# Goal

Prevent hidden side effects by making all external changes proposal-driven.

# Files

## New
- `crates/veld/src/services/sync_proposals.rs`
- `crates/veld/src/routes/sync.rs` (extend existing route)

# Implementation

## Proposal kinds
Support at least:
- `todoist_relabel_task`
- `todoist_move_task_project`
- `todoist_reorder_today`
- `calendar_move_event`
- `calendar_add_prep_block`
- `calendar_mark_transparent`
- `project_registry_alias_add`

## Proposal flow
1. suggestion / loop creates proposal
2. proposal stored in `sync_operations` as `pending`
3. user or policy approves
4. adapter-specific apply path executes
5. result written back as `applied` or `failed`

Do not let evaluation services call Todoist or Google Calendar mutation APIs directly.

# Acceptance criteria

- all external writes are inspectable and replay-safe
- proposals can be approved or rejected without code branching inside evaluation logic

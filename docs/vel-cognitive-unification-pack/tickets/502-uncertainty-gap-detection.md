---
title: Ticket 502 - Detect and persist uncertainty gaps from external awareness
status: proposed
owner: codex
priority: critical
---

# Goal

When Vel lacks enough information, record uncertainty explicitly and route it properly.

# Files

## New
- `crates/vel-core/src/uncertainty.rs`
- `crates/veld/src/services/uncertainty.rs`
- `migrations/0029_uncertainties.sql`

# Gap types
Implement at least:
- `project_mapping_missing`
- `event_project_ambiguous`
- `task_priority_ambiguous`
- `schedule_conflict_unresolved`
- `insufficient_evidence_for_writeback`
- `competing_now_candidates`

# Detection sources
- todoist adapter
- calendar adapter
- project registry sync
- suggestion engine
- scheduling loops

# Resolution model
States:
- `open`
- `asked_user`
- `resolved`
- `dismissed`

Resolution may happen by:
- user answer
- new external evidence
- explicit dismissal

# Acceptance criteria

- the repo has a durable uncertainty table and service
- unresolved gaps appear in context and can suppress unsafe proposals

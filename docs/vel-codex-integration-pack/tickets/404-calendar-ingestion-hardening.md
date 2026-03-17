---
title: Ticket 404 - Rework calendar ingestion around canonical event identity and free/busy semantics
status: proposed
owner: codex
priority: high
---

# Goal

Make calendar awareness good enough to support real schedule pressure reasoning and proposal generation.

# Current issue

The existing calendar adapter parses ICS lines and emits `calendar_event` signals, but it does not provide:
- durable normalized event cache
- stable external IDs across sync runs
- event/project linkage
- consistent busy vs free semantics from Codex Workspace calendar schema

# Grounding

Codex Workspace canonical calendar schema:
- `title`
- `start`
- `end`
- `location`
- `notes`
- `visibility`
- `calendar_id`

# Files

## Changed
- `crates/veld/src/adapters/calendar.rs`
- `crates/veld/src/services/external_sync.rs`
- `crates/vel-storage/src/db.rs`

# Concrete changes

## Event identity
Prefer source-stable IDs in this order:
1. Google Calendar event id from snapshot/API
2. ICS `UID`
3. fallback deterministic hash over title + start + end + calendar id

## Normalize visibility
Map source visibility to:
- `busy`
- `free`
- `transparent`
- `unknown`

Store raw source value in `payload_json`.

## Project linkage
Project resolution heuristics, in order:
1. explicit metadata field if present
2. project name prefix / bracket convention if already used in titles
3. alias match against known projects

If unresolved:
- store null
- do not hallucinate

## Signals to emit
- `calendar_event_seen`
- `calendar_event_changed`
- `calendar_busy_window_seen`
- `calendar_free_window_seen`
- `calendar_event_project_unresolved`

## Commitment linkage
Create/update commitments only for event types that imply an obligation:
- meetings
- appointments
- deadlines
- travel
Do not turn every free/busy block into a commitment.

# Acceptance criteria

- upcoming events can be queried from normalized cache
- event/project linkage is stable when metadata exists
- free/busy windows are available to context and suggestion services

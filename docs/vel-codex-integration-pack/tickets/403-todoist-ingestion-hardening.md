---
title: Ticket 403 - Rework Todoist adapter around normalized external items
status: proposed
owner: codex
priority: high
---

# Goal

Refactor `crates/veld/src/adapters/todoist.rs` so it stops jumping straight from snapshot item -> commitment only.

# Current issue

Today the adapter:
- loads a snapshot
- emits a signal
- creates/updates a commitment

Missing layers:
- normalized external item cache
- stable Todoist project/name mapping to internal project slug
- richer lifecycle detection
- durable linkage between task and commitment

# Files

## Changed
- `crates/veld/src/adapters/todoist.rs`
- `crates/veld/src/services/external_sync.rs` (new)
- `crates/vel-storage/src/db.rs`

# Concrete changes

## Snapshot parsing
Keep current JSON support, but normalize into `ExternalItemKind::Task`.

## Fingerprint
Compute a fingerprint from:
- content
- checked/completed state
- due
- labels
- project id
- parent/section if available

Use SHA-256 of canonical JSON serialization.

## Project resolution
Map Todoist project ID or name to `project_slug` using imported project registry.
If unresolved:
- leave `project_slug` null
- emit an uncertainty item later; do not invent a slug

## Commitment rules
Create or update a commitment when:
- task is actionable
- task is not purely archival/completed history beyond retention window

Preserve external linkage:
- create/update `external_items`
- create `external_item_links` to commitment and signal

## Signals to emit
At minimum:
- `external_task_seen`
- `external_task_completed`
- `external_task_due_changed`
- `external_task_project_unresolved`

# Optional enhancement

Import Todoist activity exports from Codex Workspace when available and use them to backfill historical signals rather than only current snapshots.

# Acceptance criteria

- every imported Todoist task has a normalized external-item record
- linked commitments can be traced back to canonical Todoist identity
- project resolution is deterministic and observable

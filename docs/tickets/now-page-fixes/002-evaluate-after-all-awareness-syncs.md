---
id: NOW-002
status: proposed
title: Run evaluate after every sync that affects awareness
owner: backend
priority: P0
---

## Goal

Ensure awareness-affecting sync operations actually update current context.

## Why

Calendar and Todoist sync call evaluate. Activity/git/messaging/notes/transcripts do not. That creates stale awareness after successful ingestion.

## Files likely touched

- `crates/veld/src/routes/sync.rs`
- `crates/veld/src/app.rs`
- relevant tests in `crates/veld/src/app.rs`

## Requirements

1. After successful sync for these sources, call `services::evaluate::run(...)`:
   - activity
   - git
   - messaging
   - notes
   - transcripts
2. Keep the route read/write boundary clean:
   - sync route ingests
   - then explicitly evaluates
3. Preserve existing sync success/error reporting behavior.
4. If evaluate fails after a successful sync, log it clearly and decide one policy:
   - either fail the route, or
   - return success with an explicit degraded warning

### Recommended policy

Fail the route. A sync that does not refresh awareness is not actually successful for this UI.

## Tests

Add/extend tests so that after each sync route:

- `current_context` is updated
- the inferred state reflects newly ingested signals where applicable

## Acceptance criteria

- Syncing activity/git/messaging/notes/transcripts updates current context in the same request path.
- No awareness-affecting sync route leaves current context stale.

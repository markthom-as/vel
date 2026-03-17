---
id: NOW-007
status: proposed
title: Prioritize Todoist and open tasks for the Now surface
owner: backend
priority: P1
---

## Goal

Make the task list reflect urgency and relevance, not insertion chronology.

## Why

`created_at DESC` is fine for a raw list endpoint. It is bad for “what matters right now.”

## Files likely touched

- `crates/vel-storage/src/db.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/services/integrations.rs`
- `crates/vel-api-types/src/lib.rs`
- tests

## Requirements

1. Add a backend prioritization path for Now tasks.
2. Prioritize by:
   - pending medication today
   - overdue
   - due today with time
   - due today without time
   - high priority/recently updated
   - everything else
3. Enrich Todoist-backed commitments with whatever metadata is needed:
   - priority
   - labels
   - updated_at if available
4. Do not bury prioritization inside React.

## Implementation options

### Option A
Add a storage query purpose-built for Now.

### Option B
Load open commitments and sort in service layer.

Recommended: service-layer sort first, move into SQL only if needed.

## Tests

- overdue task outranks newly created no-due task
- pending meds outrank ordinary tasks
- due-soon tasks sort ahead of no-due tasks

## Acceptance criteria

- Todoist backlog on Now matches urgency/actionability.

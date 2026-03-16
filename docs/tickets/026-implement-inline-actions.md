---
title: "Implement Inline Actions"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 025-implement-card-renderer
  - 017-implement-intervention-actions-api
labels:
  - vel
  - chat-interface
---
Add inline actions to structured cards.

## Supported Actions

- `mark_done`
- `snooze`
- `resolve`
- `dismiss`
- `show_why`

## Acceptance Criteria

- actions call server endpoints
- UI state updates after actions
- duplicate actions do not create broken state

## Notes for Agent

Cards should be operational, not decorative.

---
title: "Add Frontend Tests"
status: todo
owner: agent
type: implementation
priority: medium
created: 2026-03-15
depends_on:
  - 022-implement-conversation-list
  - 023-implement-thread-view
  - 025-implement-card-renderer
  - 032-implement-settings-ui
labels:
  - vel
  - chat-interface
---
Add frontend tests.

## Cover

- message renderer
- card actions
- thread loading
- settings save

## Acceptance Criteria

- critical rendering paths are tested
- action wiring is tested
- regressions in structured card rendering are catchable

## Notes for Agent

The renderer and action plumbing are the parts most likely to quietly rot.

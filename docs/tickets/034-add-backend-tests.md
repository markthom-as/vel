---
title: "Add Backend Tests"
status: todo
owner: agent
type: implementation
priority: medium
created: 2026-03-15
depends_on:
  - 014-implement-conversation-api
  - 015-implement-message-api
  - 017-implement-intervention-actions-api
  - 018-implement-websocket-server
labels:
  - vel
  - chat-interface
---
Expand backend test coverage.

## Cover

- repositories
- state transitions
- API endpoints
- websocket events

## Acceptance Criteria

- core mutation flows are tested
- websocket behavior is at least smoke-tested
- invalid transitions are covered

## Notes for Agent

Test what mutates, not just what compiles.

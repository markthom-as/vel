---
title: "Implement Initial Database Schema"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 007-create-sqlite-migration-system
  - 005-implement-message-domain-model
  - 006-implement-intervention-model
labels:
  - vel
  - chat-interface
---
Create migration `0001_init.sql`.

## Tables

- `conversations`
- `messages`
- `interventions`
- `event_log`

## Acceptance Criteria

- database initializes with all required tables
- foreign keys are valid
- schema supports current-state queries plus audit log

## Notes for Agent

Store structured payloads as JSON where the schema is still in motion, but keep table naming and relations disciplined.

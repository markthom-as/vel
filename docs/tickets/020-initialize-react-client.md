---
title: "Initialize React Client"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 001-initialize-monorepo
  - 002-configure-tooling
labels:
  - vel
  - chat-interface
---
Scaffold the web client.

## Location

`clients/web`

## Stack

- React
- TypeScript
- Vite
- Tailwind

## Acceptance Criteria

- dev server runs
- base styling works
- workspace scripts can start the client

## Notes for Agent

Keep the shell lean. The UI will get complicated soon enough all by itself.

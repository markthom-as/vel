---
title: "Create SQLite Migration System"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 003-create-rust-crates
labels:
  - vel
  - chat-interface
---
Set up migrations in `vel-store`.

## Goal

Enable database migrations for local development and app boot.

## Tasks

- Add migration tooling
- Create migrations directory
- Ensure migrations run automatically on server boot

## Acceptance Criteria

- migrations can run from CLI or startup
- server boot applies unapplied migrations
- SQLite opens with WAL enabled

## Notes for Agent

Keep startup predictable. No mystical bootstrap behavior.

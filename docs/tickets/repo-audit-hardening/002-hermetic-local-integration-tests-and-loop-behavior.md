---
title: Hermetic local-integration tests and loop behavior
status: in_progress
owner: agent
type: implementation
priority: high
created: 2026-03-17
depends_on:
  - 001-docs-truth-repair-and-entrypoint-alignment.md
labels:
  - vel
  - tests
  - integrations
  - reliability
---

Make optional local snapshot inputs degrade cleanly so tests and runtime loops do not fail on missing ambient files.

## Scope

- background loop behavior for missing optional local inputs
- bootstrap local-context sync behavior
- adapter handling for absent optional snapshot files and notes roots
- `cargo test` reliability from a clean checkout

## Acceptance criteria

- missing optional snapshot paths return zero ingest instead of internal errors
- malformed existing files still fail explicitly
- `cargo test` passes without requiring developer-created local snapshot files

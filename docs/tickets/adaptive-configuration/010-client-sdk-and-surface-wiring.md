---
id: vel-adaptive-config-010
title: Wire surfaces and clients to consume effective config
status: proposed
priority: P2
owner: platform
---

## Summary
Ensure desktop/mobile/watch/voice/CLI surfaces and internal components consume effective config instead of each inventing their own shadow settings logic.

## Scope
- define a lightweight client contract for fetching effective config
- wire relevant surfaces to pass normalized signals
- ensure components read resolved values from one source of truth
- add caching/invalidation semantics for config hash changes

## Acceptance Criteria
- at least one surface and one backend component consume effective config end-to-end
- signal submission and config retrieval are traced by session/task
- config changes propagate predictably without restart

## Tests
- end-to-end scenario test from signal -> policy -> effective config -> client behavior

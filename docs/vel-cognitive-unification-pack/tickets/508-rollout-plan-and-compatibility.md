---
title: Ticket 508 - Roll out cognitive unification without breaking the current repo
status: proposed
owner: codex
priority: medium
---

# Goal

Sequence the work so existing routes and flows keep functioning.

# Rollout plan

## Phase 1
- add migrations
- add domain types
- keep existing adapters working

## Phase 2
- dual-write external items while preserving current commitment behavior
- compute new context sections behind a feature flag

## Phase 3
- upgrade suggestion engine to evidence-based writes
- enable uncertainties in context

## Phase 4
- enable loop registry and review loops
- migrate UI to project-aware surfaces

# Compatibility requirements
- existing `GET /v1/context` should remain valid JSON
- existing Todoist/calendar settings pages should still work
- current simple suggestion rows can coexist during migration

# Acceptance criteria

- upgrade path is incremental
- no forced greenfield rewrite
- operator can inspect mixed-mode state during transition

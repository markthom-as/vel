---
title: Normalize global page-state, freshness, degraded-state, and recovery UX
status: todo
owner: agent
priority: P1
area: web-ui
created: 2026-03-17
depends_on:
  - WUI-002-transport-decoder-and-query-boundaries.md
labels:
  - web
  - ux
  - freshness
---

# Goal

Make loading, empty, error, stale, and recovery behavior consistent across the web shell.

## Scope

- shared page-state components and conventions
- freshness badge semantics
- degraded-state copy and recovery affordances

## Requirements

1. Standardize `fresh`, `aging`, `stale`, `error`, and `disconnected` semantics.
2. Distinguish empty data from missing configuration and failed sync.
3. Reuse common loading/error/empty components instead of per-page reinvention.
4. Provide the next useful operator action when a page is degraded.

## Write scope

- shared UI primitives and page-state helpers
- cross-page freshness/degraded-state rendering
- copy and tests

## Acceptance criteria

- similar failures look and behave similarly across pages
- stale data is visible but clearly marked
- operators are not left guessing whether a page is empty, broken, or simply disconnected

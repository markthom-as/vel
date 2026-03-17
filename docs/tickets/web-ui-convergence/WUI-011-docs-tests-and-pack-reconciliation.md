---
title: Reconcile docs, regression coverage, and historical packet references
status: todo
owner: agent
priority: P0
area: web-ui
created: 2026-03-17
depends_on:
  - WUI-004-now-context-stats-contracts.md
  - WUI-005-settings-and-integrations-ia.md
  - WUI-007-inbox-threads-suggestions-role-cleanup.md
  - WUI-010-projects-web-workspaces.md
labels:
  - docs
  - tests
  - rollout
---

# Goal

Keep the new web/operator surface architecture durable after implementation starts.

## Scope

- docs/status and ticket-index reconciliation
- regression tests for cross-surface behavior
- historical packet cross-links and demotion notes

## Requirements

1. Update relevant docs to point to the canonical web operator surface spec and this pack.
2. Add regression coverage for shared freshness, page-state, realtime, and cross-surface handoff behavior.
3. Ensure older packets remain usable as historical inputs without competing as default execution docs.
4. Keep implementation truth in `docs/status.md`.

## Write scope

- docs/specs and ticket indexes
- tests across backend and web where shared contracts matter
- residual packet READMEs or cross-links as needed

## Acceptance criteria

- contributors have one obvious web/UI execution entrypoint
- the repo has regression coverage for the most failure-prone cross-surface behavior
- doc drift risk is materially lower than before the consolidation

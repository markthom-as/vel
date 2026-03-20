---
phase: 12-operator-shell-onboarding-and-connector-ergonomics
plan: 01
subsystem: docs
tags: [phase-12, shell, onboarding, docs, runtime-api]
requires:
  - phase: 10-daily-loop-morning-overview-and-standup-commitment-engine
    provides: daily-loop authority over Now, web, CLI, and Apple surfaces
  - phase: 11-agent-grounding-and-operator-relevant-data-tool-awareness
    provides: backend-owned trust posture for operator-facing shell surfaces
provides:
  - explicit Phase 12 decision to reuse existing typed runtime routes and user docs for contextual help/setup routing
  - clearer operator guidance for Now, Settings, Todoist, linking, and Apple/local-source setup paths
  - runtime API guardrail against client-only contextual-help state
affects: [phase-12, docs, runtime-api, web-shell, onboarding]
tech-stack:
  added: []
  patterns: [docs-first contract clarification, contextual operator routing, backend-owned help boundary]
key-files:
  created:
    - .planning/phases/12-operator-shell-onboarding-and-connector-ergonomics/12-01-SUMMARY.md
  modified:
    - docs/api/runtime.md
    - docs/user/daily-use.md
    - docs/user/setup.md
    - docs/user/integrations/README.md
key-decisions:
  - "Phase 12 does not add a dedicated backend contextual-help payload in 12-01; the shell should route from existing typed runtime state to existing user docs."
  - "Settings remains the primary operator surface for linking, writeback trust, Todoist, and Apple/local-source setup guidance."
  - "If later shell slices need help metadata, they must publish a typed contract first rather than storing that behavior in client-only state."
patterns-established:
  - "When a phase mainly clarifies shell and onboarding authority, first decide whether existing typed routes are sufficient before adding new payloads."
requirements-completed: [DOCS-01, INTEGR-UX-01]
duration: 8m
completed: 2026-03-19
---

# Phase 12-01 Summary

**Phase 12 now has an explicit contract-first help boundary: existing typed runtime routes remain the source of truth, and the shell should route operators to the correct setup and troubleshooting docs instead of inventing hidden help state.**

## Accomplishments

- Added a Phase 12 shell/help contract note to [runtime.md](/home/jove/code/vel/docs/api/runtime.md) clarifying that there is no standalone contextual-help payload yet and that later help metadata must land as a typed contract first.
- Updated [daily-use.md](/home/jove/code/vel/docs/user/daily-use.md) so the operator shell has a clearer authority split between `Now`, `Settings`, and `Threads`, including where to go for Apple and local-source setup help.
- Updated [setup.md](/home/jove/code/vel/docs/user/setup.md) with a direct shell-to-guide routing section and an explicit decision note that Phase 12 currently reuses existing typed routes and user docs.
- Updated [integrations/README.md](/home/jove/code/vel/docs/user/integrations/README.md) so stale Todoist, local-source, and Apple/macOS issues map to the right guide instead of a generic integration index.

## Verification

- `npm --prefix clients/web test -- --run src/types.test.ts`
- `rg -n "help|setup|linking|Todoist|Apple|local-source" docs/api/runtime.md docs/user/daily-use.md docs/user/setup.md docs/user/integrations/README.md`

Both passed.

## Next Phase Readiness

- `12-02` can now tighten shell navigation and freshness UX without inventing a new backend help payload first.
- If later Phase 12 slices discover a real need for shell-help metadata, they now have an explicit rule: add the typed contract before UI behavior depends on it.

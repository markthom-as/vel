---
phase: 55-outmoded-ui-path-cleanup-and-seam-hardening
plan: 01
subsystem: ui
tags: [react, web, cleanup, routing, settings]
requires:
  - phase: 54-final-ui-cleanup-and-polish-pass
    provides: final polished shell surfaces ready for cleanup
provides:
  - reduced route taxonomy with one real MVP shell lane
  - removal of dead Suggestions/Stats web surface files
  - removal of placeholder main-panel fallback routing
  - removal of legacy settings tab compatibility
affects: [phase-56-verification]
tech-stack:
  added: []
  patterns: [route-lane pruning, compatibility removal]
key-files:
  modified:
    [
      clients/web/src/data/operatorSurfaces.ts,
      clients/web/src/shell/MainPanel/MainPanel.tsx,
      clients/web/src/shell/MainPanel/MainPanel.test.tsx,
      clients/web/src/views/settings/SettingsPage.tsx,
      clients/web/src/data/context.ts,
      clients/web/src/App.tsx,
      clients/web/src/README.md
    ]
  removed:
    [
      clients/web/src/views/suggestions/SuggestionsView.tsx,
      clients/web/src/views/suggestions/SuggestionsView.test.tsx,
      clients/web/src/views/suggestions/index.ts,
      clients/web/src/views/stats/StatsView.tsx,
      clients/web/src/views/stats/StatsView.test.tsx,
      clients/web/src/views/stats/index.ts
    ]
key-decisions:
  - "Suggestions and Stats are no longer carried as dormant shell routes in the web client."
  - "Settings tab compatibility now reflects only the current `general` / `integrations` / `runtime` contract."
patterns-established:
  - "When a surface is explicitly demoted out of the MVP shell, remove its route lane and dead file tree instead of leaving a placeholder branch behind."
requirements-completed: [CLEANUP-01, CLEANUP-02, CLEANUP-03, CLEANUP-04]
duration: session
completed: 2026-03-22
---

# Phase 55: Outmoded UI path cleanup and seam hardening Summary

**The web client now carries one honest MVP shell lane: dead `Suggestions`/`Stats` routes are gone, `MainPanel` no longer pretends to support them, and `SettingsPage` dropped its obsolete legacy-tab shim.**

## Performance

- **Duration:** session slice
- **Started:** 2026-03-22
- **Completed:** 2026-03-22
- **Tasks:** 4
- **Files modified:** 7
- **Files removed:** 6

## Accomplishments

- Reduced the operator surface registry to the still-supported route set and left `Projects` as the only hidden detail surface.
- Removed the placeholder route branch from `MainPanel`, so the shell no longer has a second dormant behavior lane.
- Removed legacy `SettingsPage` tab compatibility and simplified the tab-to-section mapping to the current contract only.
- Deleted the unused web-only `Suggestions` and `Stats` view trees and updated the local source README to reflect the actual view map.
- Kept focused shell/settings/projects tests green after the cleanup.

## Decisions Made

- Superseded lanes should be deleted, not kept as shell placeholders “just in case.”
- Cleanup stayed strictly inside the web-client shell and did not widen into backend or milestone-closeout work.

## Deviations from Plan

None.

## Issues Encountered

- `npm run build` still fails due to a large pre-existing strict TypeScript error block in `SettingsPage.tsx`; the cleanup slice removed its own introduced errors, but the broader file still needs dedicated closeout work.

## Next Phase Readiness

- Phase 56 can now verify the shell against one real route model rather than a mixed active-plus-placeholder posture.

---
*Phase: 55-outmoded-ui-path-cleanup-and-seam-hardening*
*Completed: 2026-03-22*

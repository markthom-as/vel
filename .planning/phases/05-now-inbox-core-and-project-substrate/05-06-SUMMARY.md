---
phase: 05-now-inbox-core-and-project-substrate
plan: 06
subsystem: ui
tags: [phase-5, web, typescript, now, inbox, projects, linking]
requires:
  - phase: 05-now-inbox-core-and-project-substrate
    provides: typed backend project, linking, and action/intervention contracts from 05-01 through 05-05
provides:
  - explicit web DTOs and decoders for project, linking, action-item, and review-snapshot payloads
  - web data helpers for projects, pairing tokens, linking status, and inbox triage mutations
  - deterministic thread-route reuse helper for open-thread inbox actions
affects: [phase-05, web, now, inbox, projects, linking, continuity]
tech-stack:
  added: []
  patterns: [explicit DTO decoders, thin data-layer API helpers, query invalidation for shared phase state]
key-files:
  created:
    - clients/web/src/data/chat.test.ts
  modified:
    - clients/web/src/types.ts
    - clients/web/src/types.test.ts
    - clients/web/src/data/chat.ts
    - clients/web/src/data/context.ts
    - clients/web/src/data/operator.ts
    - clients/web/src/data/ws-sync.ts
key-decisions:
  - "The web layer decodes Phase 05 payloads explicitly at the boundary instead of relying on unchecked casts or UI-local inference."
  - "Project and linking API access stays in narrow data helpers so React views remain read-oriented shells over server truth."
  - "Inbox open-thread remains a route helper over conversation_id plus available_actions, not a new browser-owned workflow."
patterns-established:
  - "When backend DTOs expand, update TypeScript interfaces, decoders, and decoder tests in the same slice."
  - "Phase-state websocket refreshes should invalidate Now, sync bootstrap, project, and linking caches together when they share backend-owned truth."
requirements-completed: [NOW-01, NOW-02, INBOX-01, INBOX-02, CONTINUITY-01, CONTINUITY-02, PROJ-03]
duration: 9m
completed: 2026-03-19
---

# Phase 05-06 Summary

**Typed web decoders and data helpers now cover Phase 05 projects, linking, Now review state, and Inbox triage mutations**

## Performance

- **Duration:** 9 min
- **Started:** 2026-03-19T02:35:00Z
- **Completed:** 2026-03-19T02:44:05Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- Added explicit TypeScript contracts and decoders for project records, linking payloads, action items, review snapshots, expanded Inbox rows, expanded Now payloads, and sync/bootstrap state.
- Added web data-layer helpers for loading and creating projects, issuing and redeeming pairing tokens, loading linking status, and mutating inbox items through acknowledge, snooze, and dismiss actions.
- Added deterministic tests for decoder coverage, triage mutation routes, and `getInboxThreadPath`, while also fixing the pre-existing optional `reminders` decode failure in the integrations transport test.

## Task Commits

No task commits were created. This slice was executed inline in the current Phase 05 worktree and left uncommitted for review.

## Files Created/Modified

- `clients/web/src/types.ts` - Adds Phase 05 DTOs and explicit decoders for project, linking, action-item, review-snapshot, inbox, now, and sync/bootstrap payloads.
- `clients/web/src/types.test.ts` - Covers the expanded Now, Inbox, project, linking, and sync bootstrap transport contracts.
- `clients/web/src/data/chat.ts` - Adds inbox mutation helpers and the thread-route reuse helper.
- `clients/web/src/data/chat.test.ts` - Verifies acknowledge, snooze, dismiss, and open-thread route reuse behavior.
- `clients/web/src/data/context.ts` - Adds sync bootstrap loading alongside the expanded Now decode path.
- `clients/web/src/data/operator.ts` - Adds project and linking API helpers for the Phase 05 web boundary.
- `clients/web/src/data/ws-sync.ts` - Invalidates Now, sync bootstrap, project, linking, and inbox caches together on Phase 05 websocket updates.

## Decisions Made

- Project and linking request payloads stay typed in the data layer, but UI policy remains backend-owned.
- `SyncBootstrapData` decoding lives with other context-facing loaders because it is part of the shared runtime state hydration path.
- The optional `reminders` integration section is decoded only when present so older or partial payloads remain compatible.

## Deviations from Plan

None - plan executed within the intended scope.

## Issues Encountered

- The baseline web test suite started with one failing decoder assertion because `decodeIntegrationsData` treated the optional `reminders` section as required. That was corrected while expanding the Phase 05 transport layer so the focused web verification could return to green.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The web client now has typed access to the Phase 05 backend contracts and mutation routes needed for UI work.
- The next dependent slice is `05-07`, which can build the actual web Now, Inbox, Projects, and linking views on top of these helpers without adding browser-owned policy.

---
*Phase: 05-now-inbox-core-and-project-substrate*
*Completed: 2026-03-19*

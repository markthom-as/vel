---
phase: 05-now-inbox-core-and-project-substrate
plan: 07
subsystem: ui
tags: [phase-5, web, typescript, now, inbox, projects, linking]
requires:
  - phase: 05-now-inbox-core-and-project-substrate
    provides: typed Phase 05 web contracts and mutation helpers from 05-06
provides:
  - ranked Now action stack rendered ahead of supporting schedule and source panels
  - dense Inbox triage rows with explicit acknowledge, snooze, dismiss, and open-thread controls
  - real Projects and linked-device support surfaces wired into the web shell
affects: [phase-05, web, now, inbox, projects, linking, continuity]
tech-stack:
  added: []
  patterns: [thin React views over shared data helpers, optimistic Inbox triage state, local-first project drafting]
key-files:
  created:
    - clients/web/src/components/ProjectsView.tsx
    - clients/web/src/components/ProjectsView.test.tsx
  modified:
    - clients/web/src/App.tsx
    - clients/web/src/components/NowView.tsx
    - clients/web/src/components/InboxView.tsx
    - clients/web/src/components/MainPanel.tsx
    - clients/web/src/components/Sidebar.tsx
    - clients/web/src/components/SettingsPage.tsx
    - clients/web/src/components/NowView.test.tsx
    - clients/web/src/components/InboxView.test.tsx
    - clients/web/src/components/MainPanel.test.tsx
    - clients/web/src/components/Sidebar.test.tsx
    - clients/web/src/components/SettingsPage.test.tsx
key-decisions:
  - "Inbox open-thread routing reuses the canonical Threads surface by setting conversation state in App instead of inventing an Inbox-specific thread screen."
  - "Projects stay local-first in the browser: the create flow captures durable local roots and explicit deferred provisioning flags, while backend policy remains authoritative."
  - "Guided linking in Settings exposes scope disclosure and token issuance, but trust state still comes from cluster bootstrap and backend linking contracts."
patterns-established:
  - "When a Phase 05 surface exists in multiple clients, keep the browser view read-oriented and push mutations through typed helpers plus shared query invalidation."
  - "Dense triage rows should surface action labels, timestamps, project context, and evidence without expanding into a second thread UI."
requirements-completed: [NOW-01, NOW-02, INBOX-01, INBOX-02, REVIEW-01, CONTINUITY-01, CONTINUITY-02, PROJ-03]
duration: 16m
completed: 2026-03-19
---

# Phase 05-07 Summary

**The web shell now ships the real Phase 05 Now, Inbox, Projects, and guided linking views**

## Performance

- **Duration:** 16 min
- **Started:** 2026-03-19T02:44:30Z
- **Completed:** 2026-03-19T03:00:55Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Rebuilt `Now` around a ranked action stack sourced from `data.action_items`, keeping freshness and supporting schedule/source panels visible below it.
- Replaced the generic Inbox list with dense triage rows that show evidence, project context, and explicit `Acknowledge`, `Snooze 10m`, `Dismiss`, and `Open thread` actions.
- Replaced the Projects placeholder with a real grouped project registry, detail pane, and local-first `Create project` flow, while also adding linked-device status cards and pairing-token issuance to Settings.

## Task Commits

No task commits were created. This slice was executed inline in the current Phase 05 worktree and left uncommitted for review.

## Files Created/Modified

- `clients/web/src/App.tsx` - Reuses the canonical Threads surface for Inbox open-thread navigation.
- `clients/web/src/components/NowView.tsx` - Adds the ranked action stack and review snapshot chips ahead of supporting panels.
- `clients/web/src/components/InboxView.tsx` - Adds dense triage rows, optimistic action handling, and open-thread routing.
- `clients/web/src/components/MainPanel.tsx` - Wires the real Projects surface and Inbox thread handoff.
- `clients/web/src/components/Sidebar.tsx` - Keeps `Now`, `Inbox`, then `Projects` at the front of navigation.
- `clients/web/src/components/ProjectsView.tsx` - Adds grouped project registry, detail pane, and local-first project draft form.
- `clients/web/src/components/SettingsPage.tsx` - Adds linked-node status cards, scope disclosure, pairing-token issuance, and CLI fallback copy.
- `clients/web/src/components/NowView.test.tsx` - Verifies action-stack ordering and evidence rendering.
- `clients/web/src/components/InboxView.test.tsx` - Verifies empty-state copy, explicit triage buttons, and open-thread reuse.
- `clients/web/src/components/MainPanel.test.tsx` - Verifies the Projects placeholder is gone.
- `clients/web/src/components/Sidebar.test.tsx` - Verifies nav ordering remains `Now`, `Inbox`, then `Projects`.
- `clients/web/src/components/ProjectsView.test.tsx` - Verifies grouped families and the explicit local-first create flow.
- `clients/web/src/components/SettingsPage.test.tsx` - Verifies linked-device status and pairing-token issuance.

## Decisions Made

- The web shell remains a thin operator surface: it renders backend-owned action, project, and linking state directly instead of inferring new browser-side policy.
- Inbox actions use optimistic pending state in the shared query cache so rows disappear immediately but still reconcile correctly with websocket updates.
- Guided linking is disclosed in explicit steps and keeps the CLI path visible as the fallback for node pairing.

## Deviations from Plan

None - plan executed within the intended scope.

## Issues Encountered

- Existing `NowView` tests mocked raw transport payloads without the new `action_items` and `review_snapshot` fields. The view was hardened to tolerate partial fixtures while the tests were updated to assert the new Phase 05 contract.

## User Setup Required

None - the new web surfaces use existing backend endpoints and local settings state.

## Next Phase Readiness

- The web client now exposes the Phase 05 project, triage, and linking surfaces needed for multi-client continuity work.
- The next dependent slice is `05-08`, which can bring the Apple client to parity with the typed Phase 05 continuity model.

---
*Phase: 05-now-inbox-core-and-project-substrate*
*Completed: 2026-03-19*

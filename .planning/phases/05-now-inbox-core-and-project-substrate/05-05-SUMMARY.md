---
phase: 05-now-inbox-core-and-project-substrate
plan: 05
subsystem: api
tags: [phase-5, now, inbox, sync, projects, interventions, action-queue]
requires:
  - phase: 05-now-inbox-core-and-project-substrate
    provides: typed project substrate and local-first project records from 05-02
  - phase: 05-now-inbox-core-and-project-substrate
    provides: durable linking state and scoped node trust records from 05-03
provides:
  - backend-owned ranked action/intervention synthesis for Now, Inbox, and sync/bootstrap
  - typed Now review counts and top-ranked action stack from persisted evidence
  - Inbox triage rows plus acknowledge, snooze, dismiss, and open-thread affordances backed by server state
  - shared sync/bootstrap hydration for linked nodes, projects, and action items
affects: [phase-05, now, inbox, sync, web, apple, continuity]
tech-stack:
  added: []
  patterns: [backend-owned action queue, shared typed hydration across Now and sync, server-backed inbox triage mutations]
key-files:
  created:
    - crates/veld/src/services/operator_queue.rs
  modified:
    - crates/vel-storage/src/db.rs
    - crates/vel-storage/src/repositories/chat_repo.rs
    - crates/veld/src/services/now.rs
    - crates/veld/src/services/client_sync.rs
    - crates/veld/src/services/chat/interventions.rs
    - crates/veld/src/services/chat/mapping.rs
    - crates/veld/src/services/chat/reads.rs
    - crates/veld/src/routes/now.rs
    - crates/veld/src/routes/chat.rs
    - crates/veld/src/routes/sync.rs
    - crates/veld/src/app.rs
key-decisions:
  - "The action/intervention stack is synthesized once in the backend and then reused by Now, Inbox enrichment, and sync/bootstrap instead of letting each surface rank locally."
  - "Inbox triage keeps thread reuse explicit by exposing conversation_id and open_thread as a client hint instead of adding another thread-opening endpoint."
  - "Acknowledge is a first-class persisted intervention state, separate from snooze, dismiss, and resolve, so thin clients can reflect triage without inventing local state."
patterns-established:
  - "Operator-facing ranking logic should live in a dedicated backend service that returns evidence-bearing ActionItem records."
  - "When sync/bootstrap gains a new typed contract, the cluster payload and top-level bootstrap payload should ship the same fields in the same slice."
requirements-completed: [NOW-01, NOW-02, INBOX-01, INBOX-02, ACTION-01, CONTINUITY-01]
duration: 25m
completed: 2026-03-19
---

# Phase 05-05 Summary

**Backend-ranked action/intervention state now drives Now, Inbox triage, and sync/bootstrap with typed evidence and explicit acknowledge flow**

## Performance

- **Duration:** 25 min
- **Started:** 2026-03-19T02:09:00Z
- **Completed:** 2026-03-19T02:34:05Z
- **Tasks:** 2
- **Files modified:** 13

## Accomplishments

- Added a dedicated backend `operator_queue` service that ranks freshness, linking, intervention, project, and commitment work into evidence-bearing `ActionItem` records.
- Extended `Now` and sync/bootstrap to expose typed `action_items`, plus typed `review_snapshot` on `Now`, without falling back to untyped JSON.
- Enriched Inbox rows with conversation/thread reuse metadata, project context, available triage actions, and persisted evidence, and added an explicit acknowledge mutation path.

## Task Commits

No task commits were created. This slice was executed inline in an already-dirty Phase 05 worktree and left uncommitted for review.

## Files Created/Modified

- `crates/veld/src/services/operator_queue.rs` - Synthesizes ranked action/intervention items and review counts from persisted runtime state.
- `crates/veld/src/services/now.rs` - Reuses the ranked action queue for top-5 Now action items and typed review snapshot counts.
- `crates/veld/src/services/client_sync.rs` - Hydrates linked nodes, projects, and ranked action items into sync/bootstrap state.
- `crates/veld/src/services/chat/reads.rs` - Enriches Inbox and conversation intervention rows from ranked action items and persisted message context.
- `crates/veld/src/services/chat/mapping.rs` - Defines typed Inbox row fields and exact available-action hints.
- `crates/veld/src/services/chat/interventions.rs` - Adds persisted acknowledge handling alongside existing snooze, dismiss, and resolve mutations.
- `crates/veld/src/routes/chat.rs` - Maps enriched Inbox rows and exposes `POST /api/interventions/:id/acknowledge`.
- `crates/veld/src/routes/now.rs` - Maps typed `action_items` and `review_snapshot` into `NowData`.
- `crates/veld/src/routes/sync.rs` - Maps typed linked nodes, projects, and action items for both top-level bootstrap and cluster payloads.
- `crates/vel-storage/src/repositories/chat_repo.rs` - Persists acknowledged intervention state.
- `crates/vel-storage/src/db.rs` - Exposes acknowledge storage support through the storage facade.
- `crates/veld/src/app.rs` - Adds app-level verification for Now, sync/bootstrap, enriched Inbox rows, and acknowledge mutation behavior.

## Decisions Made

- Ranking bands remain concrete and explainable in code: freshness 90, linking 85, intervention 80, blocked project 75, due commitment 70, and review 60.
- Inbox action affordances stay limited to `acknowledge`, `snooze`, `dismiss`, and conditional `open_thread`; project-opening and thread-opening remain separate client concerns.
- Sync/bootstrap carries the same action-item vocabulary as Now so thin clients hydrate server truth instead of re-deriving urgency locally.

## Deviations from Plan

None - plan executed within the intended scope.

## Issues Encountered

- The first compile pass hit a Rust type-inference/ownership issue around linked-node IDs in the new action queue; this was resolved by tightening the local `node_id` handling before wider verification.
- The existing app tests compiled but did not prove the new contract fields, so targeted assertions were added for sync/bootstrap hydration, Now review state, Inbox row enrichment, and acknowledge persistence.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `Now`, `Inbox`, and sync/bootstrap now share one backend-owned action/intervention vocabulary with typed evidence and persisted triage state.
- The next dependent slice is `05-06`, which can add web data loaders and mutation helpers on top of these stable backend contracts.

---
*Phase: 05-now-inbox-core-and-project-substrate*
*Completed: 2026-03-19*

---
phase: 08-coding-centric-supervised-execution-with-gsd-and-local-agents
plan: 04
subsystem: execution-routing
tags: [phase-08, execution, routing, handoff, review, cli, web, veld]
requires:
  - phase: 08-coding-centric-supervised-execution-with-gsd-and-local-agents
    provides: execution context sidecar and local runtime seams from 08-02 and 08-03
provides:
  - persisted execution handoff storage plus explicit review state transitions
  - typed routing decisions surfaced through execution routes, CLI, and operator web views
  - pending execution review projected into Now and runtime Settings surfaces
affects: [phase-08, execution, operator-surfaces, cli, web]
tech-stack:
  added: []
  patterns: [typed routing reasons, persisted operator review queue, route-thin service orchestration]
key-files:
  created:
    - migrations/0043_phase8_execution_handoffs.sql
    - crates/vel-storage/src/repositories/execution_handoffs_repo.rs
    - crates/veld/src/services/execution_routing.rs
    - crates/veld/tests/execution_routing.rs
    - .planning/phases/08-coding-centric-supervised-execution-with-gsd-and-local-agents/08-04-SUMMARY.md
  modified:
    - crates/vel-storage/src/repositories/mod.rs
    - crates/vel-storage/src/db.rs
    - crates/veld/src/services/mod.rs
    - crates/veld/src/services/operator_queue.rs
    - crates/veld/src/services/client_sync.rs
    - crates/veld/src/routes/execution.rs
    - crates/veld/src/app.rs
    - crates/vel-cli/src/client.rs
    - crates/vel-cli/src/commands/exec.rs
    - crates/vel-cli/src/main.rs
    - clients/web/src/types.ts
    - clients/web/src/data/operator.ts
    - clients/web/src/components/NowView.tsx
    - clients/web/src/components/SettingsPage.tsx
    - docs/user/daily-use.md
key-decisions:
  - "Persisted routing decisions and review state as explicit records instead of inferring launchability from transient queue state."
  - "Projected pending execution handoffs into existing operator surfaces rather than introducing a separate review UI."
  - "Kept shared-file edits minimal: one route mount export path, one storage façade, and one unrelated compile fix needed to execute the owned test target."
patterns-established:
  - "Execution handoffs stay trace-linked, scope-bounded, and review-gated before supervised launch."
  - "Routing reasons remain typed and operator-visible across transport, CLI, and web boundaries."
requirements-completed: [EXEC-02, GSD-02, HANDOFF-01, HANDOFF-02, POLICY-01]
duration: 37m
completed: 2026-03-19
---

# Phase 08 Plan 04: Execution Routing And Handoff Review Summary

**Execution handoffs are now persisted, explicitly reviewable, and visible from Now, Settings, and `vel exec` instead of hiding behind backend-only routing logic.**

## Performance

- **Duration:** 37 min
- **Completed:** 2026-03-19
- **Tasks:** 2

## Accomplishments

- Added red integration tests first for invalid handoffs, persisted typed routing reasons, launch-preview blockers, and approve/reject review state changes.
- Implemented `execution_handoffs` persistence, typed routing policy selection, and explicit `pending_review` / `approved` / `rejected` transitions behind thin execution routes.
- Projected pending execution review into `Now` action items, added `vel exec review|launch-preview|approve|reject`, and surfaced the same queue in web `Now` and runtime `Settings`.

## Files Created/Modified

- `migrations/0043_phase8_execution_handoffs.sql` - added durable handoff storage for explicit review state and launch gating.
- `crates/vel-storage/src/repositories/execution_handoffs_repo.rs` - added the storage boundary for create/list/review/launch-preview handoff flows.
- `crates/veld/src/services/execution_routing.rs` and `crates/veld/tests/execution_routing.rs` - implemented routing policy orchestration and integration coverage for persisted review flows.
- `crates/veld/src/routes/execution.rs`, `crates/veld/src/app.rs`, and `crates/veld/src/services/operator_queue.rs` - mounted the execution handoff routes and projected pending review into operator-facing status.
- `crates/vel-cli/src/client.rs`, `crates/vel-cli/src/commands/exec.rs`, and `crates/vel-cli/src/main.rs` - added typed handoff review, launch-preview, approve, and reject CLI flows.
- `clients/web/src/types.ts`, `clients/web/src/data/operator.ts`, `clients/web/src/components/NowView.tsx`, and `clients/web/src/components/SettingsPage.tsx` - surfaced typed handoff review state in the web operator shell.
- `docs/user/daily-use.md` - documented the shipped repo-local execution review commands.

## Verification

- `cargo test -p veld execution_routing -- --nocapture`
- `cargo test -p vel-cli exec -- --nocapture`
- `npm --prefix clients/web test -- --run src/types.test.ts src/data/operator.test.ts src/components/NowView.test.tsx src/components/SettingsPage.test.tsx`

## Deviations

- I made three minimal shared-file deviations to wire the owned lane cleanly:
  - [db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs) to expose the new handoff repository through `Storage`
  - [mod.rs](/home/jove/code/vel/crates/veld/src/services/mod.rs) to export the new routing service
  - [app.rs](/home/jove/code/vel/crates/veld/src/app.rs) to mount the new execution handoff routes
- I also applied one unrelated but necessary compile repair in [client_sync.rs](/home/jove/code/vel/crates/veld/src/services/client_sync.rs) so the owned `veld` test target could build; it fixes a temporary URL borrow in node-id inference and does not widen the execution-routing scope.

## Next Phase Readiness

- Phase 08 runtime execution can now consume persisted handoff review state instead of inventing its own launch gating.
- Operator review is visible on the same surfaces already used for writeback, conflict, and intervention oversight.

## Self-Check

PASSED

- FOUND: `.planning/phases/08-coding-centric-supervised-execution-with-gsd-and-local-agents/08-04-SUMMARY.md`
- FOUND: `migrations/0043_phase8_execution_handoffs.sql`
- FOUND: `crates/veld/tests/execution_routing.rs`

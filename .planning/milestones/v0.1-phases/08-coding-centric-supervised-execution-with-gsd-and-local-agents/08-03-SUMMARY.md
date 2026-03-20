---
phase: 08-coding-centric-supervised-execution-with-gsd-and-local-agents
plan: 03
subsystem: api
tags: [phase-08, connect, runtime, cli, local-command, supervision, leases, veld]
requires:
  - phase: 08-coding-centric-supervised-execution-with-gsd-and-local-agents
    provides: connect runtime requirements and route/auth direction from 08-01
provides:
  - operator-authenticated `/v1/connect/instances*` lifecycle routes
  - supervised local-command runtime launch, heartbeat, expiry, inspect, and terminate behavior
  - live `vel connect instances` and `vel connect inspect` CLI behavior backed by the runtime API
affects: [phase-08, connect, runtime, cli, local-supervision]
tech-stack:
  added: []
  patterns: [thin connect routes, service-owned runtime supervision, persisted lease plus backing run linkage]
key-files:
  created:
    - crates/veld/src/routes/connect.rs
    - crates/veld/src/services/connect_runtime.rs
    - crates/veld/tests/connect_runtime.rs
    - .planning/phases/08-coding-centric-supervised-execution-with-gsd-and-local-agents/08-03-SUMMARY.md
  modified:
    - crates/veld/src/app.rs
    - crates/veld/src/routes/mod.rs
    - crates/veld/src/services/agent_protocol.rs
    - crates/veld/src/services/mod.rs
    - crates/vel-cli/src/commands/connect.rs
    - docs/api/runtime.md
key-decisions:
  - "Kept `/v1/connect` and `/v1/connect/worker` reserved while activating only `/v1/connect/instances*` inside the operator-authenticated route class."
  - "Used the existing backing run and connect-run lease persistence as the source of truth for lifecycle state instead of introducing a parallel runtime store."
  - "Scoped the CLI lane to the already-exposed `instances` and `inspect` commands; launch and terminate remain HTTP-level in this slice because no additional CLI subcommands existed in the owned surface."
patterns-established:
  - "Connect transport routes stay thin and delegate lifecycle decisions to `services::connect_runtime`."
  - "Local runtime launches fail closed on unsupported runtime kinds and writable roots that escape the declared working directory."
requirements-completed: [EXEC-02, LOCAL-01, POLICY-01]
duration: 10m
completed: 2026-03-19
---

# Phase 08 Plan 03: Connect Runtime Summary

**Operator-authenticated connect runtime routes now supervise local-command launches through persisted run and lease state, and the CLI `instances`/`inspect` surfaces now hit the live backend instead of stubs**

## Performance

- **Duration:** 10 min
- **Started:** 2026-03-19T07:08:00Z
- **Completed:** 2026-03-19T07:18:00Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments

- Added focused integration coverage first for launch, list, heartbeat, inspect, terminate, expiry, and deny-path behavior on the connect-runtime transport.
- Activated `/v1/connect/instances*` as a real operator-authenticated transport backed by a dedicated runtime supervision service and persisted lease state.
- Replaced the `vel connect` stubs with live list/inspect behavior and documented the active runtime API plus the still-reserved connect paths.

## Task Commits

No commit was created for this slice. The worktree was already shared and dirty across concurrent Phase 07/08 work, and I kept the changes uncommitted for the orchestrator.

## Files Created/Modified

- `crates/veld/src/routes/connect.rs` - thin connect lifecycle handlers for launch, list, inspect, heartbeat, and terminate.
- `crates/veld/src/services/connect_runtime.rs` - supervised local-command runtime launch/orchestration over backing runs plus connect-run leases.
- `crates/veld/tests/connect_runtime.rs` - integration coverage for lifecycle success paths, expiry, and fail-closed denials.
- `crates/veld/src/app.rs` - minimal mount for the new connect routes and narrowed future-external reservation to preserve `/v1/connect` and `/v1/connect/worker`.
- `crates/veld/src/routes/mod.rs` - exported the connect route module alongside existing route modules.
- `crates/veld/src/services/agent_protocol.rs` - exposed the shared default connect lease duration for reuse.
- `crates/veld/src/services/mod.rs` - exported the connect runtime service module.
- `crates/vel-cli/src/commands/connect.rs` - replaced the stub messaging with live list/inspect output and unit coverage for formatting helpers.
- `docs/api/runtime.md` - documented the live `/v1/connect/instances*` API and the remaining reserved connect paths.

## Decisions Made

- Reused the existing run and connect-run persistence seams so inspectability, trace linkage, and terminal state stay explainable from persisted records.
- Kept the shared `app.rs` change minimal because that file already had in-flight edits from another lane.
- Left `crates/vel-cli/src/client.rs` unchanged because the needed list/inspect client methods already existed and were compatible with the route contract.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Removed an async launch failure path that used `futures::executor::block_on`**
- **Found during:** Task 2
- **Issue:** the new runtime service still contained a blocking fallback on launch failure, which was unsafe in the async service path.
- **Fix:** switched the spawn failure branch to a normal awaited run-status update and returned an internal error after persisting failure state.
- **Files modified:** `crates/veld/src/services/connect_runtime.rs`
- **Verification:** `cargo test -p veld connect -- --nocapture`
- **Committed in:** not committed

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary for correctness. No scope creep beyond stabilizing the planned connect-runtime slice.

## Issues Encountered

- `crates/veld/src/app.rs`, `crates/veld/src/routes/mod.rs`, and `crates/veld/src/services/mod.rs` already had unrelated in-flight edits from other workers. I kept my touches limited to the connect-runtime mount/export lines and did not revert or reshape the concurrent work.
- The `vel-cli` surface already only exposed `connect instances` and `connect inspect`. I wired those commands to the live backend and left launch/terminate as API-level operations for this lane.

## User Setup Required

None - this slice adds backend lifecycle routes, CLI read surfaces, and tests only.

## Next Phase Readiness

- The routing/execution lanes can now build on a real supervised local-runtime transport instead of a deny-all reservation.
- Future CLI work can add explicit launch/terminate subcommands on top of the live HTTP lifecycle without changing the backend contract introduced here.

---
*Phase: 08-coding-centric-supervised-execution-with-gsd-and-local-agents*
*Completed: 2026-03-19*

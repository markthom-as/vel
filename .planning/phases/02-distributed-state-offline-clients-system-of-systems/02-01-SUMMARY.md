---
phase: 02-distributed-state-offline-clients-system-of-systems
plan: 01
subsystem: docs, api, cli, web
tags: [diagnostics, operator-visibility, connect, sp1, contract-alignment, rust, typescript]

# Dependency graph
requires:
  - phase: 1.1-preflight-pre-phase-2-hardening
    provides: auth middleware, app.rs decomposition, zero clippy warnings

provides:
  - Aligned Phase 2 ticket scope with accurate implementation status (tickets 004, 005, 006, 012, 016)
  - GET /api/diagnostics operator endpoint returning DiagnosticsData with sync status, worker count, capabilities, freshness
  - FreshnessEntryData and DiagnosticsData DTOs in vel-api-types
  - SettingsPage.tsx System Diagnostics section surfacing live sync/freshness data
  - CLI connect commands no longer call dead /v1/connect/* routes
  - docs/api/connect.md documenting reserved status and SP2 planned endpoints
  - SP1 merge gate satisfied: aligned docs, no mismatch CLI/runtime, operator diagnostics closed

affects:
  - 02-02-PLAN.md (SP2 Lane A — reducer extraction, builds on aligned ticket 004)
  - 02-03-PLAN.md (SP2 Lane B — connect lifecycle MVP, builds on ticket 006 baseline)
  - 02-04-PLAN.md (SP2 Lane C — capability broker, uses agents-only scope decision from ticket 016)
  - 02-05-PLAN.md (SP3 — onboarding uses execution slice labels from ticket 012)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Operator diagnostics pattern: thin GET route handler calling existing cluster_workers_data service, mapping to DTO"
    - "Freshness classification: fresh (<5min), stale (>5min), missing — consistent across Rust handler and TypeScript display"
    - "CLI not-yet-active stubs: informative eprintln messages with alternative command suggestions"

key-files:
  created:
    - crates/veld/src/routes/diagnostics.rs
    - docs/api/connect.md
  modified:
    - crates/vel-api-types/src/lib.rs
    - crates/veld/src/routes/mod.rs
    - crates/veld/src/app.rs
    - crates/vel-cli/src/commands/connect.rs
    - clients/web/src/types.ts
    - clients/web/src/components/SettingsPage.tsx
    - docs/MASTER_PLAN.md
    - docs/tickets/phase-2/004-signal-reducer-pipeline.md
    - docs/tickets/phase-2/005-hlc-sync-implementation.md
    - docs/tickets/phase-2/006-connect-launch-protocol.md
    - docs/tickets/phase-2/012-tester-readiness-onboarding.md
    - docs/tickets/phase-2/016-capability-broker-secret-mediation.md

key-decisions:
  - "Diagnostics route placed in own file (routes/diagnostics.rs) not appended to signals.rs — cleaner separation of concerns"
  - "Freshness threshold of 5 minutes for fresh/stale classification — matches typical heartbeat intervals"
  - "CLI connect stubs use eprintln (not anyhow error) — informative message, exit 0, non-alarming UX"
  - "SettingsPage diagnostics fetch uses raw fetch in useEffect (not useQuery) — fire-and-forget, non-critical, no cache needed"
  - "FreshnessEntryData status field uses String not enum in Rust DTO — matches rest of API types pattern"

requirements-completed:
  - OPS-01
  - OPS-02

# Metrics
duration: 9min
completed: 2026-03-18
---

# Phase 2 Plan 01: SP1 Contract Alignment and Visibility Closure Summary

**Aligned all Phase 2 ticket scope/status, added GET /api/diagnostics operator endpoint with sync/worker/freshness data, and removed CLI/runtime mismatch on connect endpoints**

## Performance

- **Duration:** 9 min
- **Started:** 2026-03-18T15:52:16Z
- **Completed:** 2026-03-18T16:01:40Z
- **Tasks:** 3
- **Files modified:** 14

## Accomplishments

- Updated 5 Phase 2 tickets (004, 005, 006, 012, 016) and MASTER_PLAN.md with accurate SP1/SP2/SP3 sub-phase structure and implementation state
- Implemented GET /api/diagnostics returning DiagnosticsData (sync_status, active_workers, capability_summary, freshness_entries) — operator-authenticated, builds on existing cluster_workers_data service
- Removed CLI/runtime mismatch: connect.rs stubs now return informative "not yet active" messages instead of calling /v1/connect/* routes that return 404
- All SP1 merge gate criteria satisfied

## Task Commits

Each task was committed atomically:

1. **Task 1: Normalize ticket scope and master plan alignment** - `2c355a9` (docs)
2. **Task 2: Operator diagnostics data-shape closure** - `aeec435` (feat)
3. **Task 3: Connect surface consistency** - `251f371` (fix)
4. **Fmt fix: rustfmt formatting in connect.rs** - `e39afff` (chore)

## Files Created/Modified

- `crates/veld/src/routes/diagnostics.rs` - GET /api/diagnostics handler; derives sync_status from worker statuses, aggregates capability_summary, builds freshness_entries
- `crates/vel-api-types/src/lib.rs` - Added FreshnessEntryData and DiagnosticsData DTOs
- `crates/veld/src/routes/mod.rs` - Added diagnostics module declaration
- `crates/veld/src/app.rs` - Registered /api/diagnostics route; added SP2 Lane B comment to connect route reservations
- `crates/vel-cli/src/commands/connect.rs` - Replaced /v1/connect/* calls with "not yet active" informative messages
- `clients/web/src/types.ts` - Added FreshnessEntryData and DiagnosticsData TypeScript interfaces
- `clients/web/src/components/SettingsPage.tsx` - Added diagnostics state, fetch useEffect, System Diagnostics section in runtime tab
- `docs/MASTER_PLAN.md` - Phase 2 status updated with SP1/SP2/SP3 breakdown and per-ticket accurate status
- `docs/tickets/phase-2/004-signal-reducer-pipeline.md` - Added CurrentContextV1 reducer output contract
- `docs/tickets/phase-2/005-hlc-sync-implementation.md` - Added NodeIdentity prereq and WAL mode confirmation step
- `docs/tickets/phase-2/006-connect-launch-protocol.md` - Added Current Baseline section documenting shell-only state
- `docs/tickets/phase-2/012-tester-readiness-onboarding.md` - Labeled acceptance criteria by execution slice (A/B)
- `docs/tickets/phase-2/016-capability-broker-secret-mediation.md` - Added agents-only scope decision record
- `docs/api/connect.md` - New file documenting /v1/connect/* reserved status and SP2 planned endpoints

## Decisions Made

- Diagnostics route placed in its own file (routes/diagnostics.rs) rather than appended to signals.rs for clean separation of concerns
- Freshness threshold: 5 minutes for fresh/stale classification matches typical worker heartbeat intervals
- CLI connect stubs use eprintln with exit 0 rather than anyhow errors — informative, non-alarming UX for shell scripts
- SettingsPage diagnostics fetch uses raw fetch in useEffect (not useQuery) — fire-and-forget, non-critical data, no cache invalidation needed
- FreshnessEntryData status field uses String (not Rust enum) to match the rest of the vel-api-types pattern

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed rustfmt formatting violation in connect.rs**
- **Found during:** Post-task verification (make verify)
- **Issue:** Long function signature on single line violated rustfmt line length rules
- **Fix:** Wrapped `run_inspect_instance` parameters to multiple lines
- **Files modified:** crates/vel-cli/src/commands/connect.rs
- **Verification:** `rustfmt --edition 2021 --check` exits 0
- **Committed in:** e39afff (separate chore commit)

---

**Total deviations:** 1 auto-fixed (1 formatting)
**Impact on plan:** Trivial formatting fix. No scope creep. Pre-existing app.rs use-ordering formatting issue deferred (out-of-scope per deviation rules).

## Issues Encountered

- `make verify` revealed pre-existing rustfmt violations in `crates/veld/src/services/integrations.rs` and `crates/veld/src/app.rs` (use declaration ordering). These are out-of-scope pre-existing issues and have been deferred per deviation scope boundary rules.

## Next Phase Readiness

- SP1 merge gate satisfied: all three criteria met (ticket alignment, connect surface clean, operator diagnostics closed)
- SP2 lanes can now build against clean contracts: ticket 004 has reducer output contract, ticket 005 has NodeIdentity prereq, ticket 006 has baseline documented
- GET /api/diagnostics provides the observability surface that SP3 (onboarding) will build on
- Connect stubs are ready to be wired to real endpoints in SP2 Lane B

---
*Phase: 02-distributed-state-offline-clients-system-of-systems*
*Completed: 2026-03-18*

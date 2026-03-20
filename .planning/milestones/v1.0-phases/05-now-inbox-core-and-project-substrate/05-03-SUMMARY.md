---
phase: 05-now-inbox-core-and-project-substrate
plan: 03
subsystem: api
tags: [phase-5, linking, pairing, trust-state, routes, sqlite]
requires:
  - phase: 05-now-inbox-core-and-project-substrate
    provides: persisted project substrate and typed linking contracts from 05-01 and 05-02
provides:
  - short-lived scoped pairing tokens with durable redemption state
  - linked-node trust persistence with revoke and status inspection
  - authenticated `/v1/linking/*` backend routes with fail-closed redemption rules
affects: [phase-05, continuity, web, apple, cli, cluster]
tech-stack:
  added: []
  patterns: [migration-plus-repository linking substrate, backend-owned fail-closed scope checks, operator-authenticated linking routes]
key-files:
  created:
    - migrations/0039_phase5_linking.sql
    - crates/vel-storage/src/repositories/linking_repo.rs
    - crates/veld/src/services/linking.rs
    - crates/veld/src/routes/linking.rs
  modified:
    - crates/vel-storage/src/db.rs
    - crates/vel-storage/src/lib.rs
    - crates/vel-storage/src/repositories/mod.rs
    - crates/vel-storage/src/repositories/connect_runs_repo.rs
    - crates/veld/src/services/mod.rs
    - crates/veld/src/routes/mod.rs
    - crates/veld/src/app.rs
key-decisions:
  - "Pairing tokens remain short-lived and scope-bounded, with a 900-second default TTL and a 3600-second maximum."
  - "Redemption fails closed on malformed, expired, already-redeemed, or out-of-scope requests before trust state is widened."
  - "Linked-node trust state lives in backend-owned storage and is surfaced through authenticated status and revoke routes."
patterns-established:
  - "Guided linking backend slices land as migration plus repository plus storage facade plus service plus thin authenticated routes."
  - "Link-scope narrowing is allowed at redemption time, but widening beyond the issued token is rejected in service orchestration."
requirements-completed: [CONTINUITY-02]
duration: 11m
completed: 2026-03-19
---

# Phase 05-03 Summary

**Scoped pairing tokens, durable linked-node trust state, and authenticated fail-closed linking routes**

## Performance

- **Duration:** 11 min
- **Started:** 2026-03-19T01:52:08Z
- **Completed:** 2026-03-19T02:02:43Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Added the `pairing_tokens` and `linked_nodes` SQLite substrate plus repository and storage-facade methods for issue, redeem, list, and revoke flows.
- Added backend linking orchestration that enforces short-lived scoped tokens and rejects malformed, expired, already-redeemed, or out-of-scope redemption attempts.
- Mounted authenticated `/v1/linking/tokens`, `/v1/linking/redeem`, `/v1/linking/status`, and `/v1/linking/revoke/:node_id` routes with focused route and service tests.

## Task Commits

No task commits were created. This slice was executed inline in an already-dirty Phase 05 worktree and left uncommitted for review.

## Files Created/Modified

- `migrations/0039_phase5_linking.sql` - Adds `pairing_tokens` and `linked_nodes`.
- `crates/vel-storage/src/repositories/linking_repo.rs` - Persists pairing tokens and linked-node trust state with repository tests.
- `crates/vel-storage/src/db.rs` - Exposes linking storage facade methods.
- `crates/vel-storage/src/lib.rs` - Re-exports linking storage/core types.
- `crates/vel-storage/src/repositories/mod.rs` - Registers the linking repository module.
- `crates/vel-storage/src/repositories/connect_runs_repo.rs` - Moves a test-only import under `#[cfg(test)]` to keep targeted builds clean.
- `crates/veld/src/services/linking.rs` - Implements issue, redeem, list, revoke, TTL, and scope-check orchestration with service tests.
- `crates/veld/src/services/mod.rs` - Exports the linking service module.
- `crates/veld/src/routes/linking.rs` - Adds thin linking handlers and explicit scope vocabulary at the request boundary.
- `crates/veld/src/routes/mod.rs` - Registers linking routes.
- `crates/veld/src/app.rs` - Mounts authenticated linking routes and adds app-level linking route tests.

## Decisions Made

- Token issuance stays backend-owned and operator-authenticated; clients consume the linking flow without owning trust policy.
- Requested scopes may be narrower than the granted token, but never broader.
- Revocation is represented as durable trust-state mutation rather than ephemeral in-memory status.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Replaced an orphaned conversion impl with a local scope helper**
- **Found during:** Task 2 (linking service and route verification)
- **Issue:** `impl From<LinkScopeData> for LinkScope` in `crates/veld/src/services/linking.rs` violated Rust orphan rules and blocked compilation.
- **Fix:** Replaced the impl with a local `scope_from_data` helper and updated the service flow to use it directly.
- **Files modified:** `crates/veld/src/services/linking.rs`
- **Verification:** `cargo test -p veld linking -- --nocapture`

---

**Total deviations:** 1 auto-fixed (blocking compile fix)
**Impact on plan:** Necessary correctness repair only. No scope creep.

## Issues Encountered

- The first `veld` verification pass exposed the orphan-rule compile error in the new linking service. It was fixed inline before the final green test run.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The backend half of guided linking is now in place for CLI, web, and Apple continuity work.
- The next dependent slice is `05-04`, which can add the CLI fallback and runtime docs for guided node linking on top of these authenticated routes.

---
*Phase: 05-now-inbox-core-and-project-substrate*
*Completed: 2026-03-19*

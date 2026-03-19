---
phase: 07-apple-action-loops-and-behavioral-signal-ingestion
plan: 02
subsystem: api
tags: [phase-7, apple, ios, watch, voice, now, sync-actions, veld, integration-tests]
requires:
  - phase: 07-apple-action-loops-and-behavioral-signal-ingestion
    provides: typed Apple voice/schedule contracts and Swift wire parity from 07-01
provides:
  - backend-owned Apple voice route and orchestration service in `veld`
  - transcript-first persistence for Apple voice turns before query/action replies
  - backend-derived schedule answers from `/v1/now` plus safe low-risk action reuse through existing sync action seams
affects: [phase-07, apple, ios, watch, now, sync, captures]
tech-stack:
  added: []
  patterns: [thin authenticated Apple route, transcript-first voice persistence, backend-owned Apple schedule/query authority]
key-files:
  created:
    - crates/veld/src/routes/apple.rs
    - crates/veld/src/services/apple_voice.rs
    - crates/veld/tests/apple_voice_loop.rs
    - .planning/phases/07-apple-action-loops-and-behavioral-signal-ingestion/07-02-SUMMARY.md
  modified:
    - crates/veld/src/app.rs
    - crates/veld/src/routes/mod.rs
    - crates/veld/src/services/mod.rs
    - crates/veld/src/services/client_sync.rs
    - crates/veld/src/services/local_network.rs
key-decisions:
  - "Apple voice turns now persist transcript provenance before any query or mutation response is returned."
  - "Apple schedule answers are derived from backend `Now` output and exposed through the new Apple voice route rather than Swift-local synthesis."
  - "Low-risk Apple voice mutations reuse the existing `client_sync` action path and fail closed when the backend cannot resolve a safe target."
patterns-established:
  - "New Apple interaction routes should stay in the operator-authenticated group and delegate all logic to a dedicated service."
  - "When Apple quick-loop actions mutate state, they should map to existing safe replayable action seams instead of introducing client-owned policy."
requirements-completed: [IOS-01, IOS-02, IOS-03, APPLE-01]
duration: 6m
completed: 2026-03-19
---

# Phase 07 Plan 02: Apple Voice Backend Summary

**Backend-owned Apple voice turns now persist transcript provenance, answer schedule/context queries from `/v1/now`, and route safe low-risk actions through existing sync mutation seams**

## Performance

- **Duration:** 6 min
- **Started:** 2026-03-19T07:03:22Z
- **Completed:** 2026-03-19T07:09:02Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Added failing integration coverage first for transcript-first Apple voice persistence, backend-derived schedule answers, and safe low-risk Apple actions.
- Added a thin authenticated `/v1/apple/voice/turn` route and a dedicated `apple_voice` service that keeps query/mutation policy in Rust.
- Grounded Apple schedule answers in backend `Now` state and reused the existing `client_sync` mutation path for bounded safe actions.

## Task Commits

Each task was committed atomically where the repository state allowed it:

1. **Task 1: Add failing integration coverage for the backend-owned Apple voice loop** - `b6a6605` (`test`)
2. **Task 2: Implement the Apple voice route and service on top of existing capture, now, and safe-action seams** - `66f523d` (integrated repo commit containing the route/service/test slice)

**Plan metadata:** pending docs commit for summary/state updates

## Files Created/Modified

- `crates/veld/tests/apple_voice_loop.rs` - focused integration coverage for Apple transcript persistence, backend schedule answers, and safe action reuse.
- `crates/veld/src/services/apple_voice.rs` - backend Apple voice orchestration over capture persistence, `Now`, and `client_sync`.
- `crates/veld/src/routes/apple.rs` - thin authenticated Apple voice endpoint.
- `crates/veld/src/app.rs`, `crates/veld/src/routes/mod.rs`, `crates/veld/src/services/mod.rs` - route and service wiring.
- `crates/veld/src/services/client_sync.rs` - compile-blocking test fixture fix while keeping the safe action seam reusable.
- `crates/veld/src/services/local_network.rs` - compile-blocking `if-addrs` fixture update for the newer `Ifv4Addr` shape.

## Decisions Made

- Kept Apple voice interpretation and action safety in Rust even though Swift still contains older local parsing code that later plans will remove from authority.
- Used the existing safe sync action seam instead of inventing a parallel Apple-only mutation flow.
- Treated `/v1/now` as the canonical schedule source and derived Apple schedule payloads server-side from that output.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated compile-drifted fixtures for current dependency shapes**
- **Found during:** Task 2
- **Issue:** The build was blocked by pre-existing compile failures unrelated to Apple voice behavior: `Ifv4Addr` now requires `prefixlen`, and a `TailscalePeer` fixture was missing `candidate_base_urls`.
- **Fix:** Updated the affected test/helper fixtures so `veld` could compile and the Apple verification target could run.
- **Files modified:** `crates/veld/src/services/client_sync.rs`, `crates/veld/src/services/local_network.rs`
- **Verification:** `cargo test -p veld apple_voice -- --nocapture` and `cargo test -p veld now_endpoint_returns_consolidated_snapshot -- --nocapture`
- **Committed in:** `66f523d`

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary to restore compilation for the focused Phase 07 verification targets. No behavioral scope creep beyond enabling the planned Apple slice to build and test.

## Issues Encountered

- Another in-flight repo commit advanced `HEAD` while this plan was executing and absorbed the Apple implementation slice. I verified the landed tree, used the actual commit hashes in this summary, and avoided replaying duplicate code changes.

## User Setup Required

None - this slice adds backend route/service behavior and tests only.

## Next Phase Readiness

- Phase `07-03` can build bounded Apple behavior-signal ingestion on top of the new Apple backend route/service seam.
- Phase `07-04` can switch Apple clients away from local query authority and onto the backend-owned voice path that is now in place.

## Self-Check

PASSED

- FOUND: `.planning/phases/07-apple-action-loops-and-behavioral-signal-ingestion/07-02-SUMMARY.md`
- FOUND: `b6a6605`
- FOUND: `66f523d`

---
*Phase: 07-apple-action-loops-and-behavioral-signal-ingestion*
*Completed: 2026-03-19*

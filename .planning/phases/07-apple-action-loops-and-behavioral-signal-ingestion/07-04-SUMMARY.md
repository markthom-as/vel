---
phase: 07-apple-action-loops-and-behavioral-signal-ingestion
plan: 04
subsystem: apple-ui
tags: [phase-7, apple, ios, watchos, swift, velapi, now, voice, health]
requires:
  - phase: 07-apple-action-loops-and-behavioral-signal-ingestion
    provides: backend-owned Apple voice route and schedule semantics from 07-02
  - phase: 07-apple-action-loops-and-behavioral-signal-ingestion
    provides: bounded Apple behavior summary route and Now projection from 07-03
provides:
  - typed Apple client methods for /v1/now, /v1/apple/voice/turn, and /v1/apple/behavior-summary
  - cached backend-owned Now and Apple behavior summary payloads for Apple quick loops
  - iPhone voice UI aligned to backend-owned Apple replies instead of Swift-local query synthesis
  - watch quick-loop rendering over backend /v1/now plus Apple behavior summary
  - Apple/runtime/user docs aligned to auth, offline, schedule, and behavior-summary semantics
affects: [phase-07, apple, ios, watch, now, docs]
tech-stack:
  added: []
  patterns: [typed Apple route client methods, backend-owned quick-loop rendering, shared auth header seam]
key-files:
  created:
    - .planning/phases/07-apple-action-loops-and-behavioral-signal-ingestion/07-04-SUMMARY.md
  modified:
    - clients/apple/VelAPI/Sources/VelAPI/Models.swift
    - clients/apple/VelAPI/Sources/VelAPI/VelClient.swift
    - clients/apple/VelAPI/Sources/VelAPI/OfflineStore.swift
    - clients/apple/Apps/VeliOS/ContentView.swift
    - clients/apple/Apps/VelWatch/VelWatchApp.swift
    - clients/apple/Apps/VelWatch/ContentView.swift
    - clients/apple/README.md
    - docs/api/runtime.md
    - docs/user/daily-use.md
key-decisions:
  - "Apple schedule retrieval now uses typed /v1/now transport and cache data instead of Swift-local schedule synthesis."
  - "Supported iPhone voice replies route through the backend Apple voice endpoint; offline fallback is limited to provenance capture, cached backend rendering, and queued safe actions."
  - "Apple quick-loop auth stays in shared VelAPI transport via explicit operator/bearer header configuration rather than per-view request logic."
patterns-established:
  - "Apple surfaces should cache backend-owned quick-loop payloads but keep policy and query interpretation in Rust."
  - "Watch and iPhone shells can reuse the same queued sync lane for safe offline mutations while treating backend Now and Apple summary payloads as read authority."
requirements-completed: [IOS-01, IOS-02, IOS-03, HEALTH-02, APPLE-01]
duration: 7m
completed: 2026-03-19
---

# Phase 07 Plan 04: Apple Client Wiring Summary

**Apple clients now read backend-owned Now, voice, and bounded behavior-summary contracts through shared VelAPI transport/cache seams without restoring Swift-local query authority**

## Performance

- **Duration:** 7 min
- **Started:** 2026-03-19T07:27:54Z
- **Completed:** 2026-03-19T07:34:58Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Added typed Apple client support for `/v1/now`, `/v1/apple/voice/turn`, and `/v1/apple/behavior-summary`, plus explicit shared auth headers for operator-authenticated `/v1/*` calls.
- Cached backend-owned Now and Apple behavior summary payloads so watch and iPhone quick loops can render cached server truth without synthesizing local query answers.
- Reworked the iPhone Voice tab and watch quick-loop surfaces to rely on backend schedule/behavior replies, then updated Apple/runtime/user docs to match the real Phase 07 behavior.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add shared Apple transport and cache wiring for backend-owned voice and behavior payloads** - `708347f` (`feat`)
2. **Task 2: Replace iPhone/watch local query behavior with backend-owned loops and align docs** - `6b02a5e` (`feat`)

**Plan metadata:** pending docs commit for summary/state updates

## Files Created/Modified

- `clients/apple/VelAPI/Sources/VelAPI/Models.swift` - added typed `Now` transport models for Apple quick-loop reads.
- `clients/apple/VelAPI/Sources/VelAPI/VelClient.swift` - added `/v1/now`, Apple voice, Apple behavior-summary methods, and explicit auth-header configuration.
- `clients/apple/VelAPI/Sources/VelAPI/OfflineStore.swift` - cached backend-owned Now and Apple behavior summary payloads alongside the existing queue lane.
- `clients/apple/Apps/VeliOS/ContentView.swift` - routed supported voice replies through the backend Apple voice endpoint, restricted offline fallback, and exposed operator-token settings.
- `clients/apple/Apps/VelWatch/VelWatchApp.swift` and `clients/apple/Apps/VelWatch/ContentView.swift` - refreshed watch quick loops from `/v1/now` and Apple behavior summary with cached rendering and existing queued safe actions.
- `clients/apple/README.md`, `docs/api/runtime.md`, and `docs/user/daily-use.md` - aligned docs to current Apple auth, offline, voice, schedule, and behavior-summary semantics.

## Decisions Made

- Kept Swift intent suggestion only as shell behavior; supported query replies now come from backend Apple routes or cached backend payloads, not local synthesized answers.
- Treated `/v1/now` as the explicit Apple schedule source and the Apple behavior-summary route as the quick-loop health/activity surface.
- Kept offline Apple mutations on the existing sync action queue instead of widening client-owned write policy.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- `make check-apple-swift` validates the shared `VelAPI` package only; it does not compile or execute the iPhone/watch app targets on this Linux host.
- Manual iPhone/watch quick-loop exercise was not possible in this environment because the required Xcode simulator/device runtime is unavailable here.

## User Setup Required

None - no external service configuration required beyond the documented Apple endpoint and optional operator-token settings already described in repo docs.

## Next Phase Readiness

- Apple shells now consume the backend Phase 07 quick-loop contracts, so future Apple work can extend presentation without reintroducing client-owned policy.
- Backend/auth docs and the shared Apple transport/cache seam are aligned for stricter authenticated device usage.

## Self-Check

PASSED

- FOUND: `.planning/phases/07-apple-action-loops-and-behavioral-signal-ingestion/07-04-SUMMARY.md`
- FOUND: `708347f`
- FOUND: `6b02a5e`

---
*Phase: 07-apple-action-loops-and-behavioral-signal-ingestion*
*Completed: 2026-03-19*

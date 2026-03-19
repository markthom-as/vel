---
phase: 10-daily-loop-morning-overview-and-standup-commitment-engine
plan: 05
subsystem: apple
tags: [phase-10, apple, voice, ios, daily-loop, docs]
requires:
  - phase: 10-daily-loop-morning-overview-and-standup-commitment-engine
    provides: typed backend daily-loop routes and session engine from 10-02 and 10-03
  - phase: 07-apple-action-loops-and-behavioral-signal-ingestion
    provides: transcript-first Apple voice route and Apple client shell baseline
provides:
  - Apple voice MorningBriefing delegation into the shared backend daily-loop authority
  - typed Swift transport models, client methods, caching, and tests for daily-loop sessions
  - iPhone Voice-tab start/resume shell over backend-owned morning and standup sessions
  - owner docs that point CLI, Apple, and operators to `/v1/daily-loop/*` and `vel morning` / `vel standup`
affects: [phase-10, apple, daily-loop, transport, docs, offline-cache]
tech-stack:
  added: []
  patterns: [typed shared Swift transport, transcript-first backend delegation, cached-render-only offline Apple behavior]
key-files:
  modified:
    - crates/veld/src/services/apple_voice.rs
    - crates/veld/src/routes/apple.rs
    - crates/veld/tests/apple_voice_loop.rs
    - clients/apple/VelAPI/Package.swift
    - clients/apple/VelAPI/Sources/VelAPI/Models.swift
    - clients/apple/VelAPI/Sources/VelAPI/VelClient.swift
    - clients/apple/VelAPI/Sources/VelAPI/OfflineStore.swift
    - clients/apple/VelAPI/Tests/VelAPITests/DailyLoopTests.swift
    - clients/apple/Apps/VeliOS/VelApp.swift
    - clients/apple/Apps/VeliOS/ContentView.swift
    - clients/apple/README.md
    - docs/api/runtime.md
    - docs/user/daily-use.md
key-decisions:
  - "Apple remains a transcript capture and rendering shell; morning and standup policy stays in the shared Rust daily-loop service."
  - "Offline Apple daily-loop behavior is cache-only for active session state and cannot invent new prompts, commitments, or reduction logic."
  - "Operator docs now distinguish the legacy `/v1/context/morning` brief from the bounded Phase 10 daily-loop authority."
patterns-established:
  - "When a backend workflow becomes shared across CLI, web, and Apple, ship the typed Swift transport, cache seam, backend delegation, and owner docs in the same slice."
  - "Apple voice intent handlers should preserve transcript provenance first, then delegate to the narrow backend authority instead of branching into local heuristics."
requirements-completed: [VOICE-01, MORNING-01, STANDUP-01]
duration: 29m
completed: 2026-03-19
---

# Phase 10-05 Summary

**Apple voice now starts and resumes the same backend daily loop used by CLI and web, while the iPhone shell stays a thin cached transport surface**

## Accomplishments

- Routed Apple `MorningBriefing` intents through the shared backend daily-loop service so transcript-first voice entry can start or resume morning overview and standup sessions without adding Apple-only policy.
- Added typed `VelAPI` daily-loop models, shared client methods for start/active/turn routes, offline cache keys for morning and standup sessions, and package-level test coverage for session decoding and route usage.
- Wired the iPhone `Voice` tab and `VelClientStore` to load cached active sessions, start morning/standup, submit or skip prompts when online, and render cached session state without synthesizing new local commitments while offline.
- Updated Apple and runtime docs so operators see `vel morning`, `vel standup`, and `/v1/daily-loop/*` as the Phase 10 authority, while `/v1/context/morning` is clearly documented as the legacy context brief.

## Verification

- `cargo test -p veld apple_voice -- --nocapture`
- `make check-apple-swift`
- `swift test --package-path clients/apple/VelAPI --filter DailyLoop` attempted but blocked by local wrapper error: `/etc/profiles/per-user/jove/bin/swift: line 35: exec: swift-test: not found`
- `rg -n "/v1/daily-loop|vel standup|legacy context brief|resume" clients/apple/README.md docs/api/runtime.md docs/user/daily-use.md clients/apple/Apps/VeliOS/ContentView.swift clients/apple/VelAPI/Sources/VelAPI/OfflineStore.swift crates/veld/src/services/apple_voice.rs`

Rust tests passed, the shared Swift package built successfully via `make check-apple-swift`, and the doc/authority grep passed.

## Remaining Verification

- Human simulator/device verification is still pending on macOS/Xcode for the full `VeliOS` app flow:
  - start morning briefing from the Voice tab
  - resume into standup
  - confirm the one-to-three commitment cap
  - confirm offline mode only shows cached session state

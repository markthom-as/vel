---
phase: 07
slug: apple-action-loops-and-behavioral-signal-ingestion
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-03-19
---

# Phase 07 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` + Apple shared-package compile check |
| **Config file** | Rust uses Cargo defaults; Apple package uses `make check-apple-swift` |
| **Quick run command** | `cargo test -p veld apple_voice -- --nocapture && cargo test -p veld apple_behavior -- --nocapture && make check-apple-swift` |
| **Full suite command** | `make verify && make check-apple-swift && cargo test -p vel-cli -- --nocapture` |
| **Estimated runtime** | ~300 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p <touched-crate> <targeted-test> -- --nocapture` and `make check-apple-swift` when `VelAPI` or app targets move
- **After every plan wave:** Run `make verify && make check-apple-swift`
- **Before `$gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 150 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 07-01-01 | 01 | 1 | IOS-01, IOS-02, HEALTH-02, APPLE-01 | unit/schema/transport | `cargo test -p vel-api-types apple -- --nocapture && make check-apple-swift && node scripts/verify-repo-truth.mjs` | ❌ W0 | ⬜ pending |
| 07-02-01 | 02 | 2 | IOS-01, IOS-02, IOS-03, APPLE-01 | integration | `cargo test -p veld apple_voice -- --nocapture && cargo test -p veld now_endpoint_returns_consolidated_snapshot -- --nocapture` | ❌ W0 | ⬜ pending |
| 07-03-01 | 03 | 3 | HEALTH-01, HEALTH-02, APPLE-01 | integration | `cargo test -p veld apple_behavior -- --nocapture && cargo test -p veld sync_health_ingests_snapshot_and_triggers_evaluation -- --nocapture` | ❌ W0 | ⬜ pending |
| 07-04-01 | 04 | 4 | IOS-01, IOS-02, IOS-03, HEALTH-02, APPLE-01 | Apple/package/doc | `make check-apple-swift && rg -n "voice|schedule|behavior|nudge|offline" clients/apple/README.md docs/user/daily-use.md docs/api/runtime.md` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/vel-api-types/src/lib.rs` tests for Apple voice-turn, schedule snapshot, and behavior-summary DTO serialization invariants
- [ ] `clients/apple/VelAPI/Sources/VelAPI/Models.swift` decode/build verification for the new Apple DTO surfaces
- [ ] `crates/veld/tests/apple_voice_loop.rs` for transcript-first persistence, backend-owned schedule answers, and safe low-risk action handling
- [ ] `crates/veld/tests/apple_behavior_summary.rs` for bounded steps/stand/exercise rollups, ignored out-of-scope metrics, and freshness/evidence projection
- [ ] `crates/veld/src/services/now.rs` or route-level coverage proving Apple schedule answers derive from backend `Now` state
- [ ] `clients/apple/Apps/VeliOS/ContentView.swift` review and package build confirming the old Swift-local query synthesis path is removed or explicitly deprecated in favor of backend authority
- [ ] `clients/apple/VelAPI/Sources/VelAPI/OfflineStore.swift` verification that any new Apple cache state remains read-only render/cache support and does not introduce an Apple-only mutation path

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| iPhone push-to-talk captures transcript, submits to backend, and renders a typed answer with reasons | IOS-01, IOS-02, APPLE-01 | Audio permissions, transcript editing, and spoken response quality require a real device/simulator pass | In iOS app, run a voice query for current context/next commitment, confirm a persisted backend response returns and any spoken playback matches the typed answer |
| Apple Watch quick actions still route through the shared safe queue path when offline | IOS-03, APPLE-01 | Queue/reconnect behavior is easiest to confirm with live reachability changes | Go offline on watch or simulator, trigger nudge done/snooze or quick capture, reconnect, and confirm the queued action replays without a watch-only policy fork |
| Behavior summaries stay bounded to steps/stand/exercise and remain explainable | HEALTH-01, HEALTH-02 | Trust/readability of summaries is semantic rather than purely structural | Seed same-day behavior snapshots, inspect the resulting summary in the Apple surface, and confirm the evidence cites freshness/timestamps and only the bounded metric set |
| Apple docs and runtime docs describe the real permissions, offline semantics, and backend-owned voice authority | APPLE-01 | Documentation truth needs a human review against the live surfaces | Compare `clients/apple/README.md`, `docs/user/daily-use.md`, and `docs/api/runtime.md` with the implemented app flow and confirm no doc still implies Swift-local query authority |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 150s for targeted checks
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending

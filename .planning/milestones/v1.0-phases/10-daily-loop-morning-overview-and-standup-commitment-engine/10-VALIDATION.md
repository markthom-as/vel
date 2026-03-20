---
phase: 10
slug: daily-loop-morning-overview-and-standup-commitment-engine
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-03-19
---

# Phase 10 — Validation Strategy

> Per-phase validation contract for the strict morning-overview and standup loop.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` (Rust unit/integration), `vitest` 2.1.8 (web), `swift test` / `make check-apple-swift` (Apple package) |
| **Config file** | Workspace `Cargo.toml`, `clients/web/vitest.config.ts`, `clients/apple/VelAPI/Package.swift` |
| **Quick run command** | `cargo test -p vel-api-types daily_loop -- --nocapture && cargo test -p vel-storage daily_sessions -- --nocapture` |
| **Full suite command** | `make verify && make check-apple-swift` |
| **Estimated runtime** | ~30-90s for per-task targeted checks; longer for `make verify` |

---

## Sampling Rate

- **After every task commit:** run the task-level targeted command from the table below.
- **After every plan wave:** rerun the relevant `veld` integration target plus any shell/client target that depends on it.
- **Before `$gsd-verify-work`:** run `make verify`, `make check-apple-swift`, the CLI morning/standup smoke path, and one manual Apple simulator/device pass.
- **Max feedback latency:** 60 seconds for automated task checks.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 10-01-01 | 01 | 0 | SESSION-01 | Rust/unit | `cargo test -p vel-api-types daily_loop -- --nocapture` | ✅ partial | ⬜ pending |
| 10-01-02 | 01 | 0 | SESSION-01 | Rust/repo | `cargo test -p vel-storage daily_sessions -- --nocapture` | ✅ partial | ⬜ pending |
| 10-02-01 | 02 | 1 | MORNING-01 | Rust/integration | `cargo test -p veld daily_loop_morning -- --nocapture` | ❌ W0 | ⬜ pending |
| 10-02-02 | 02 | 1 | MORNING-02 | Rust/integration | `cargo test -p veld daily_loop_morning -- --nocapture` | ❌ W0 | ⬜ pending |
| 10-02-03 | 02 | 1 | MORNING-03 | Rust/route | `cargo test -p veld daily_loop_morning -- --nocapture` | ❌ W0 | ⬜ pending |
| 10-02-04 | 02 | 1 | MORNING-02, SESSION-01 | Rust/integration | `cargo test -p veld daily_loop_morning -- --nocapture` | ❌ W0 | ⬜ pending |
| 10-03-01 | 03 | 2 | STANDUP-01 | Rust/integration | `cargo test -p veld daily_loop_standup -- --nocapture` | ❌ W1 | ⬜ pending |
| 10-03-02 | 03 | 2 | STANDUP-02 | Rust/integration | `cargo test -p veld daily_loop_standup -- --nocapture` | ❌ W1 | ⬜ pending |
| 10-03-03 | 03 | 2 | STANDUP-03 | CLI/unit | `cargo test -p vel-cli daily_loop -- --nocapture` | ❌ W1 | ⬜ pending |
| 10-03-04 | 03 | 2 | STANDUP-03 | Rust/integration | `cargo test -p veld daily_loop_standup -- --nocapture` | ❌ W1 | ⬜ pending |
| 10-04-01 | 04 | 3 | SESSION-01 | web/types | `npm --prefix clients/web test -- --run src/types.test.ts` | ❌ W2 | ⬜ pending |
| 10-04-02 | 04 | 3 | MORNING-01 | web/component | `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/types.test.ts` | ❌ W2 | ⬜ pending |
| 10-05-01 | 05 | 3 | VOICE-01 | Swift/package | `swift test --package-path clients/apple/VelAPI --filter DailyLoop` | ❌ W2 | ⬜ pending |
| 10-05-02 | 05 | 3 | VOICE-01 | Rust + Swift + docs | `cargo test -p veld apple_voice -- --nocapture && make check-apple-swift && rg -n "/v1/daily-loop|vel standup|legacy context brief|resume" clients/apple/README.md docs/api/runtime.md docs/user/daily-use.md clients/apple/Apps/VeliOS/ContentView.swift clients/apple/VelAPI/Sources/VelAPI/OfflineStore.swift crates/veld/src/services/apple_voice.rs` | ❌ W2 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/vel-core/src/daily_loop.rs` — typed Phase 10 domain/session contract
- [ ] `crates/vel-api-types/src/lib.rs` — start/active/turn/outcome DTOs
- [ ] `migrations/0045_phase10_daily_sessions.sql` — durable daily-session schema
- [ ] `crates/vel-storage/src/repositories/daily_sessions_repo.rs` — resumable persistence seam
- [ ] `clients/apple/VelAPI/Package.swift` — SwiftPM test-target wiring for `DailyLoopTests`

*If none: "Existing infrastructure covers all phase requirements."*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Full morning -> standup loop fits the intended under-3-minute product contract | MORNING-02, STANDUP-03 | The time budget and tone need a real operator pass, not just schema checks | Run `vel morning`, answer/skip through the loop, continue with `vel standup`, and confirm the total flow stays brief with no more than three questions per sub-flow. |
| Morning Overview remains passive and writes no commitments before standup | MORNING-03 | This is easiest to confirm against live state before and after the morning flow | Start a morning session from CLI/web, complete it, then inspect commitments and confirm no new commitment rows were written until standup finalization. |
| Silence advances safely instead of stalling or exceeding the prompt budget | MORNING-02, STANDUP-03, SESSION-01 | Passive voice/text use needs a real timeout/silence pass | Start morning or standup, do not answer one prompt, confirm the session advances as an explicit skip/silence transition, and confirm the final prompt count still honors the three-question ceiling. |
| Apple voice start/resume uses backend authority and safe offline fallback | VOICE-01 | Linux-host automation cannot compile and run the full SwiftUI app targets | Follow the checkpoint in `10-05-PLAN.md`: simulator/device morning voice request, standup resume, then offline fallback confirmation. |
| Web/CLI shells remain thin and backend-owned | SESSION-01 | Thin-shell discipline is partly architectural and partly UX | Review changed web/CLI code to confirm prompt counts, commitment compression, and friction ranking are not recomputed outside the backend daily-loop service. |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verification or an explicit human checkpoint
- [ ] Sampling continuity: no 3 consecutive tasks without automated verification
- [ ] Wave 0 covers all typed session-state prerequisites
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s for task-level checks
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending

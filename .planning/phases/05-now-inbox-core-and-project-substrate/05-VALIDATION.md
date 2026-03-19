---
phase: 05
slug: now-inbox-core-and-project-substrate
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-03-18
---

# Phase 05 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` + Vitest 2.1.x + Apple compile check |
| **Config file** | `clients/web/vitest.config.ts`; Rust uses Cargo defaults |
| **Quick run command** | `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture && npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/components/InboxView.test.tsx` |
| **Full suite command** | `make verify && npm --prefix clients/web test && make check-apple-swift` |
| **Estimated runtime** | ~240 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p <touched-crate> <targeted-test> -- --nocapture` and/or `npm --prefix clients/web test -- --run <targeted file>`
- **After every plan wave:** Run `make verify && npm --prefix clients/web test`
- **Before `$gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 120 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 05-01-01 | 01 | 1 | PROJ-01, PROJ-02, FAMILY-01, ACTION-01 | unit/schema | `cargo test -p vel-core -- --nocapture && cargo test -p vel-api-types -- --nocapture` | ✅ partial | ⬜ pending |
| 05-02-01 | 02 | 2 | PROJ-01, PROJ-02, FAMILY-01 | repository | `cargo test -p vel-storage projects_repo -- --nocapture` | ❌ W0 | ⬜ pending |
| 05-02-02 | 02 | 2 | PROJ-03 | route/service | `cargo test -p veld project -- --nocapture` | ❌ W0 | ⬜ pending |
| 05-03-01 | 03 | 3 | CONTINUITY-02 | integration | `cargo test -p veld pairing_ -- --nocapture` | ❌ W0 | ⬜ pending |
| 05-04-01 | 04 | 4 | NOW-01, NOW-02, INBOX-01, INBOX-02, ACTION-01, REVIEW-01 | service/integration | `cargo test -p veld now inbox synthesis -- --nocapture && cargo test -p vel-cli review -- --nocapture` | ✅ partial | ⬜ pending |
| 05-05-01 | 05 | 5 | NOW-01, NOW-02, INBOX-01, INBOX-02, CONTINUITY-01, PROJ-03 | web component | `npm --prefix clients/web test -- --run src/components/NowView.test.tsx src/components/InboxView.test.tsx src/components/MainPanel.test.tsx src/components/Sidebar.test.tsx src/components/ProjectsView.test.tsx` | ✅ partial | ⬜ pending |
| 05-06-01 | 06 | 5 | CONTINUITY-01, CONTINUITY-02 | Apple compile | `make check-apple-swift` | ✅ partial | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/vel-core/src/project.rs` tests for `ProjectFamily` vocabulary, prefixed IDs, and serialization invariants
- [ ] `crates/vel-storage/src/repositories/projects_repo.rs` tests for create/list/get and legacy alias compatibility
- [ ] `crates/veld/src/services/projects.rs` tests for local-first project creation and pending-upstream confirmation behavior
- [ ] `crates/veld/src/services/linking.rs` or equivalent tests for pairing token TTL, scope enforcement, redeem, and revoke
- [ ] `crates/veld/src/routes/projects.rs` tests for DTO parity across list/detail/create flows
- [ ] `clients/web/src/components/ProjectsView.test.tsx` for the first real Projects surface
- [ ] `clients/web/src/components/NowView.test.tsx` additions covering ranked action items and project labels
- [ ] `clients/web/src/components/InboxView.test.tsx` additions covering triage actions and evidence rendering
- [ ] Apple contract verification remains compile-backed; use `make check-apple-swift` plus manual simulator smoke until a checked-in Apple test target exists

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Guided node-linking copy clearly states granted read/write/execute scope before confirmation | CONTINUITY-02 | Scope comprehension and trust wording are operator-facing semantics | Run the web or CLI linking flow, issue a token, and confirm the scope disclosure text appears before redeem/confirm |
| `Now` keeps the ranked action stack visually primary above schedule and source panels | NOW-01, NOW-02 | Visual hierarchy is easiest to confirm in the real UI | Load the web `Now` view and confirm the action stack appears first below the header/freshness banner |
| Apple clients display cached project/action/linking continuity without re-ranking locally | CONTINUITY-01 | Thin-client behavior is easiest to validate in simulator/device | Hydrate via `/v1/sync/bootstrap`, go offline, and confirm cached projects/action items/linked nodes still render without new local ranking behavior |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 120s for targeted checks
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending

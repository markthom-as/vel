---
phase: 15
slug: incremental-core-migration-and-canonical-rust-service-seams
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-03-19
---

# Phase 15 — Validation Strategy

> Per-phase validation contract for operator-action seam migration, `check_in`, `reflow`, trust/readiness projections, and project-scoped action ownership.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` + targeted `rg` contract checks |
| **Config file** | Workspace `Cargo.toml` |
| **Quick run command** | `cargo test -p vel-core operator_queue -- --nocapture && cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture` |
| **Full suite command** | `cargo test -p vel-core -- --nocapture && cargo test -p vel-api-types -- --nocapture && cargo test -p veld daily_loop -- --nocapture && cargo test -p veld now -- --nocapture && cargo test -p veld backup -- --nocapture && rg -n "check_in|reflow|readiness|project-scoped|operator action" docs .planning/phases/15-incremental-core-migration-and-canonical-rust-service-seams crates` |
| **Estimated runtime** | ~2-4 minutes for targeted Rust tests plus doc/contract checks |

---

## Sampling Rate

- **After every task commit:** Run the narrowest crate test or contract check for the touched seam.
- **After each plan wave:** Re-run at least one service-level or DTO-level assertion proving the seam still maps through existing routes cleanly.
- **Before phase close:** Manual review must confirm Phase 15 stayed migration-focused and did not drift into shell embodiment.
- **Max feedback latency:** 120 seconds for targeted checks

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 15-01-01 | 01 | 0 | MIGRATE-01, DTO-01 | docs/core truth | `rg -n "check_in|reflow|readiness|project-scoped|operator action" .planning/phases/15-incremental-core-migration-and-canonical-rust-service-seams docs/product crates/vel-core crates/vel-api-types` | ❌ W0 | ⬜ pending |
| 15-01-02 | 01 | 0 | MIGRATE-01 | domain baseline | `cargo test -p vel-core operator_queue -- --nocapture` | ✅ baseline | ⬜ pending |
| 15-02-01 | 02 | 1 | SERVICE-01, READMODEL-01 | service seam | `cargo test -p veld check_in -- --nocapture` | ❌ W0 | ⬜ pending |
| 15-02-02 | 02 | 1 | DTO-01 | DTO mapping | `cargo test -p vel-api-types check_in -- --nocapture` | ❌ W0 | ⬜ pending |
| 15-03-01 | 03 | 2 | SERVICE-01, READMODEL-01 | service seam | `cargo test -p veld reflow -- --nocapture` | ❌ W0 | ⬜ pending |
| 15-03-02 | 03 | 2 | MIGRATE-02 | daily-loop compatibility | `cargo test -p veld daily_loop -- --nocapture` | ✅ baseline | ⬜ pending |
| 15-04-01 | 04 | 3 | SERVICE-01, READMODEL-01 | readiness projection | `cargo test -p veld backup -- --nocapture && cargo test -p veld now -- --nocapture` | ✅ partial baseline | ⬜ pending |
| 15-04-02 | 04 | 3 | DTO-01 | DTO mapping | `cargo test -p vel-api-types backup -- --nocapture` | ✅ baseline | ⬜ pending |
| 15-05-01 | 05 | 4 | MIGRATE-02, SERVICE-01 | project action seam | `cargo test -p veld projects -- --nocapture && cargo test -p vel-storage projects_repo -- --nocapture` | ✅ baseline | ⬜ pending |
| 15-05-02 | 05 | 4 | READMODEL-01 | contract truth | `rg -n "project-scoped|scope_affinity|project tag|project identity" docs/product .planning/phases/15-incremental-core-migration-and-canonical-rust-service-seams crates` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `15-CONTEXT.md` — locked scope, priorities, and carry-forward decisions captured
- [ ] `15-RESEARCH.md` — current seam analysis and execution recommendation captured
- [ ] `15-VALIDATION.md` — verification plan ratified before implementation work
- [ ] roadmap updated with concrete Phase 15 plans and counts

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Phase 15 stays migration-focused rather than becoming a UI or final-logic phase | MIGRATE-01 | This is a scope-discipline judgment | Review the final plans and confirm they tighten seams without trying to close full product behavior. |
| The new seams preserve Phase 14 surface boundaries instead of reopening them | MIGRATE-02, READMODEL-01 | Architectural drift is semantic, not just testable | Read the final plans and confirm `Now`, `Inbox`, `Threads`, and `Projects` remain consumers/owners in the agreed way. |
| Project-scoped action ownership survives cross-surface projection | SERVICE-01 | This is partly a semantic modeling check | Confirm the final plan language requires visible project identity and does not flatten project actions into generic global queue items. |

---

## Validation Sign-Off

- [x] All tasks have automated or explicit manual verification
- [x] Wave 0 captures planning artifacts and roadmap updates
- [x] No watch-mode flags
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending

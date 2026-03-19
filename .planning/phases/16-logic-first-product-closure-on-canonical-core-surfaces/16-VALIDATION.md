---
phase: 16
slug: logic-first-product-closure-on-canonical-core-surfaces
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-03-19
---

# Phase 16 — Validation Strategy

> Per-phase validation contract for backend-owned `check_in`, `reflow`, trust/readiness follow-through, and project-scoped action logic.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` + targeted `npm --prefix clients/web test` + `rg` truth checks |
| **Config file** | Workspace `Cargo.toml`, `clients/web/vitest.config.ts` |
| **Quick run command** | `cargo test -p veld check_in -- --nocapture && cargo test -p veld reflow -- --nocapture && cargo test -p veld now -- --nocapture` |
| **Full suite command** | `cargo test -p vel-core -- --nocapture && cargo test -p vel-api-types -- --nocapture && cargo test -p veld -- --nocapture && npm --prefix clients/web test -- --run src/types.test.ts && rg -n "check_in|reflow|trust/readiness|project-scoped|thread escalation" .planning/phases/16-logic-first-product-closure-on-canonical-core-surfaces docs crates` |
| **Estimated runtime** | ~3-6 minutes for targeted Rust/web checks plus doc truth review |

---

## Sampling Rate

- **After every task commit:** Run the narrowest relevant crate or decoder test for the touched seam.
- **After each plan wave:** Re-run at least one `Now` route/service mapping assertion proving the widened behavior still reaches the typed read model cleanly.
- **Before phase close:** Manually confirm Phase 16 stayed logic-first and did not drift into shell embodiment work.
- **Max feedback latency:** 120 seconds for narrow checks

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 16-01-01 | 01 | 0 | LOGIC-01, READMODEL-02 | contract truth | `rg -n "check_in|reflow|project-scoped|thread escalation|transition" .planning/phases/16-logic-first-product-closure-on-canonical-core-surfaces docs/product crates/vel-core crates/vel-api-types crates/veld/src/services` | ❌ W0 | ⬜ pending |
| 16-01-02 | 01 | 0 | SHELL-ARCH-01 | route/read-model sanity | `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture` | ✅ baseline | ⬜ pending |
| 16-02-01 | 02 | 1 | FLOW-01, READMODEL-02 | service lifecycle | `cargo test -p veld check_in -- --nocapture` | ✅ baseline | ⬜ pending |
| 16-02-02 | 02 | 1 | LOGIC-01 | daily-loop consequence | `cargo test -p veld daily_loop -- --nocapture` | ✅ baseline | ⬜ pending |
| 16-03-01 | 03 | 2 | FLOW-01, LOGIC-01 | service lifecycle | `cargo test -p veld reflow -- --nocapture` | ✅ baseline | ⬜ pending |
| 16-03-02 | 03 | 2 | READMODEL-02 | route/DTO mapping | `cargo test -p veld now_service_output_maps_to_existing_now_dto_shape -- --nocapture && npm --prefix clients/web test -- --run src/types.test.ts` | ✅ baseline | ⬜ pending |
| 16-04-01 | 04 | 3 | MODE-02, READMODEL-02 | trust/readiness behavior | `cargo test -p veld trust_readiness -- --nocapture && cargo test -p veld now -- --nocapture` | ✅ partial baseline | ⬜ pending |
| 16-04-02 | 04 | 3 | SHELL-ARCH-01 | contract truth | `rg -n "summary-first trust|readiness|recovery action|review pressure" docs/product .planning/phases/16-logic-first-product-closure-on-canonical-core-surfaces crates` | ❌ W0 | ⬜ pending |
| 16-05-01 | 05 | 4 | FLOW-01, MODE-02 | project action logic | `cargo test -p veld operator_queue -- --nocapture && cargo test -p veld projects -- --nocapture` | ✅ baseline | ⬜ pending |
| 16-05-02 | 05 | 4 | SHELL-ARCH-01, READMODEL-02 | typed escalation | `cargo test -p veld threads -- --nocapture && npm --prefix clients/web test -- --run src/types.test.ts` | ✅ partial baseline | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `16-CONTEXT.md` — scope, priorities, and carry-forward logic rules captured
- [ ] `16-RESEARCH.md` — code-backed logic-closure recommendations captured
- [ ] `16-VALIDATION.md` — verification map ratified before implementation
- [ ] roadmap updated with concrete Phase 16 plans and counts

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Phase 16 remains logic-first rather than drifting into shell design | LOGIC-01, SHELL-ARCH-01 | This is a scope-discipline judgment | Review the final plans and confirm they define backend behavior and consequences, not nav/cards/layout. |
| `Now`, `Inbox`, `Threads`, and `Projects` still honor the Phase 14 taxonomy | MODE-02 | Surface-boundary correctness is partly semantic | Confirm the plans preserve `Now` as urgent-first, `Inbox` as triage, `Threads` as escalation/archive, and `Projects` as semantically project-owned. |
| Project-scoped actions remain project-owned even when routed through shared logic | FLOW-01, READMODEL-02 | Ownership drift is semantic, not just testable | Review the plan language and confirm project actions are not flattened back into global queue behavior. |

---

## Validation Sign-Off

- [x] All tasks have automated or explicit manual verification
- [x] Wave 0 captures planning artifacts and roadmap updates
- [x] No watch-mode flags
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending

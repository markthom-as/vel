---
phase: 08
slug: coding-centric-supervised-execution-with-gsd-and-local-agents
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-03-19
---

# Phase 08 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` + targeted CLI/API smoke checks + optional focused web tests for operator review surfaces |
| **Config file** | Rust/Cargo defaults; runtime/API checks use repo-local dev config |
| **Quick run command** | `cargo test -p vel-protocol -- --nocapture && cargo test -p veld agent_sdk -- --nocapture && cargo test -p vel-cli connect -- --nocapture` |
| **Full suite command** | `make verify && cargo test -p veld connect -- --nocapture && cargo test -p veld sandbox -- --nocapture` |
| **Estimated runtime** | ~360 seconds |

---

## Sampling Rate

- **After every task commit:** run the narrowest affected crate/package tests plus any touched CLI smoke path
- **After every plan wave:** run `make verify` plus the targeted Phase 08 runtime tests
- **Before `$gsd-verify-work`:** full relevant runtime, CLI, and any added operator-surface checks must be green
- **Max feedback latency:** 180 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 08-01-01 | 01 | 1 | EXEC-01, GSD-01, HANDOFF-01, HANDOFF-02, POLICY-01, LOCAL-01 | unit/schema/contract | `cargo test -p vel-core execution -- --nocapture && cargo test -p vel-api-types execution -- --nocapture && cargo test -p vel-protocol -- --nocapture` | ❌ W0 | ⬜ pending |
| 08-02-01 | 02 | 2 | EXEC-01, GSD-01, GSD-02 | integration/CLI | `cargo test -p veld execution_context -- --nocapture && cargo test -p vel-cli exec -- --nocapture` | ❌ W0 | ⬜ pending |
| 08-03-01 | 03 | 2 | EXEC-02, LOCAL-01, POLICY-01 | integration | `cargo test -p veld connect -- --nocapture && cargo test -p vel-cli connect -- --nocapture` | ❌ W0 | ⬜ pending |
| 08-04-01 | 04 | 3 | EXEC-02, GSD-02, HANDOFF-01, HANDOFF-02, POLICY-01 | integration/operator | `cargo test -p veld execution_routing -- --nocapture && cargo test -p vel-cli exec -- --nocapture` | ❌ W0 | ⬜ pending |
| 08-05-01 | 05 | 3 | EXEC-02, LOCAL-01, POLICY-01 | integration/sandbox | `cargo test -p veld wasm_guest -- --nocapture && cargo test -p veld sandbox -- --nocapture` | ❌ W0 | ⬜ pending |
| 08-06-01 | 06 | 4 | GSD-01, GSD-02, LOCAL-01, HANDOFF-02, POLICY-01 | SDK/doc/smoke | `cargo test -p veld agent_sdk -- --nocapture && cargo test -p vel-agent-sdk -- --nocapture && rg -n "GSD|connect|handoff|repo-local|writable" docs/api/runtime.md docs/user/README.md docs/user/coding-workflows.md` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `vel-core` tests for new execution-context, routing, and handoff invariants
- [ ] `vel-api-types` tests for Phase 08 DTO serialization and mapping parity
- [ ] `vel-protocol` tests for any new connect/runtime payloads or manifest-linked fields
- [ ] `veld` integration tests for `/v1/connect` launch/list/heartbeat/terminate behavior
- [ ] `veld` tests for execution routing, handoff persistence, and review-gate denials
- [ ] `veld` tests for local-runtime manifest parsing and writable-scope enforcement
- [ ] `veld` guest-runtime tests proving broker and trace mediation remain mandatory
- [ ] CLI verification for `vel connect` and any new `vel exec` surfaces

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Repo-local execution artifact export stays inside the project's primary repo root | EXEC-01, GSD-01, POLICY-01 | bounded filesystem writes are easiest to inspect directly | create/export the execution pack for a project with a real repo root and confirm no files land outside the declared root |
| Local coding runtime can be launched, inspected, and terminated from the operator shell | EXEC-02, LOCAL-01 | process lifecycle and operator ergonomics need a real shell pass | launch a local runtime, confirm heartbeats/list output, terminate it, and inspect trace/run state |
| Handoff review shows objective, scopes, expected output, and review gate before execution | HANDOFF-01, HANDOFF-02 | trust here is about readability, not just schema | create a human-to-agent handoff and confirm the same details appear in operator review output |
| Guest runtime denials remain explainable | POLICY-01 | denial clarity is partly semantic | run a guest module that requests an out-of-scope action and confirm the denial is visible and understandable |
| GSD docs describe the real repo-local handoff and artifact flow | GSD-01, GSD-02 | documentation truth needs a human pass against the live commands | compare the shipped docs to the implemented CLI/API flow and confirm there is no hidden or contradictory setup |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all missing references
- [x] No watch-mode flags
- [x] Feedback latency < 180s for targeted checks
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending

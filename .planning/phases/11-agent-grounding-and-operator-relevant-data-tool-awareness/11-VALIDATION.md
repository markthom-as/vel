---
phase: 11
slug: agent-grounding-and-operator-relevant-data-tool-awareness
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-03-19
---

# Phase 11 — Validation Strategy

> Per-phase validation contract for agent grounding, capability inspection, and operator trust visibility.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` (Rust unit/integration) + `vitest` 2.1.8 (web) |
| **Config file** | Workspace `Cargo.toml`; `clients/web/vitest.config.ts` |
| **Quick run command** | `cargo test -p vel-api-types agent_grounding -- --nocapture && cargo test -p vel-api-types agent_grounding_contract_assets -- --nocapture` |
| **Full suite command** | `cargo test -p veld agent_grounding_inspect -- --nocapture && cargo test -p veld execution_context -- --nocapture && cargo test -p vel-cli agent_inspect -- --nocapture && npm --prefix clients/web test -- --run src/types.test.ts src/data/agent-grounding.test.ts src/components/SettingsPage.test.tsx` |
| **Estimated runtime** | ~60 seconds for targeted checks; longer if broader workspace verification is added |

---

## Sampling Rate

- **After every task commit:** Run the narrowest affected command plus any matching CLI/web boundary tests.
- **After every plan wave:** Rerun the relevant `veld` integration target and any thin-shell CLI/web tests that consume the same payload.
- **Before `$gsd-verify-work`:** Full targeted Phase 11 suite plus a manual CLI/web parity pass must be green.
- **Max feedback latency:** 60 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 11-01-01 | 01 | 0 | AGENT-CTX-01, AGENT-CTX-02, AGENT-TOOLS-01 | Rust/unit | `cargo test -p vel-api-types agent_grounding -- --nocapture` | ❌ W0 | ⬜ pending |
| 11-01-02 | 01 | 0 | AGENT-CTX-01, AGENT-CTX-02, AGENT-TOOLS-01 | schema/doc | `cargo test -p vel-api-types agent_grounding_contract_assets -- --nocapture` | ❌ W0 | ⬜ pending |
| 11-02-01 | 02 | 1 | AGENT-CTX-01, AGENT-CTX-02, AGENT-TOOLS-01, AGENT-TOOLS-02 | Rust/integration | `cargo test -p veld agent_grounding_inspect -- --nocapture` | ❌ W0 | ⬜ pending |
| 11-02-02 | 02 | 1 | AGENT-REVIEW-01, AGENT-TRUST-01 | Rust/integration | `cargo test -p veld execution_context -- --nocapture` | ✅ partial | ⬜ pending |
| 11-03-01 | 03 | 2 | AGENT-TOOLS-01, AGENT-TOOLS-02, AGENT-TRUST-01 | CLI/unit | `cargo test -p vel-cli agent_inspect -- --nocapture` | ❌ W0 | ⬜ pending |
| 11-03-02 | 03 | 2 | AGENT-REVIEW-01, AGENT-TRUST-01 | web/types+component | `npm --prefix clients/web test -- --run src/types.test.ts src/data/agent-grounding.test.ts src/components/SettingsPage.test.tsx` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/vel-api-types/src/lib.rs` — typed grounding, capability, blocker, and handoff-review DTOs
- [ ] `config/schemas/agent-grounding-pack.schema.json` — machine-readable grounding-pack contract
- [ ] `config/examples/agent-grounding-pack.example.json` — checked-in example payload
- [ ] `config/schemas/agent-inspect.schema.json` — machine-readable inspect response contract
- [ ] `config/examples/agent-inspect.example.json` — checked-in inspect example
- [ ] `docs/cognitive-agent-architecture/agents/agent-grounding-contracts.md` — owner doc for the shared Phase 11 boundary

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| CLI and web show the same grounding counts, blockers, and escalation hints against the same daemon state | AGENT-TRUST-01 | Cross-surface trust parity is partly semantic, not just structural | Run `vel agent inspect` and load Settings against the same daemon, then confirm the same pending review counts, SAFE MODE blocker text, and mutation availability are shown. |
| Inspect/export stays backend-owned rather than chat-first | AGENT-CTX-02, AGENT-TRUST-01 | Architectural drift is easiest to catch in a human pass | Review the changed route/service/CLI/web code and confirm the first consumer is `/v1/agent/inspect` plus execution export, not prompt-only `/api/chat` widening. |
| Exported grounding artifacts remain bounded to the approved repo-local output directory | AGENT-REVIEW-01, AGENT-TRUST-01 | Filesystem trust needs direct inspection | Run the execution artifact preview/export flow and confirm `agent-grounding.md` and `agent-inspect.json` land only under the bounded output directory with the same data shape as the inspect route. |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verification or an explicit human checkpoint
- [x] Sampling continuity: no 3 consecutive tasks without automated verification
- [x] Wave 0 covers all typed contract prerequisites
- [x] No watch-mode flags
- [x] Feedback latency < 60s for targeted checks
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending

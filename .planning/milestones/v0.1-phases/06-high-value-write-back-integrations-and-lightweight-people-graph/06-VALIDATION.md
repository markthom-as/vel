---
phase: 06
slug: high-value-write-back-integrations-and-lightweight-people-graph
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-03-18
---

# Phase 06 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` + Vitest 2.1.x + focused CLI checks |
| **Config file** | `clients/web/vitest.config.ts`; Rust uses Cargo defaults |
| **Quick run command** | `cargo test -p vel-core integration -- --nocapture && cargo test -p vel-storage semantic_memory_repo integration_connections_repo -- --nocapture && cargo test -p veld integrations client_sync -- --nocapture` |
| **Full suite command** | `make verify && npm --prefix clients/web test && cargo test -p vel-cli -- --nocapture` |
| **Estimated runtime** | ~420 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p <touched-crate> <targeted-test> -- --nocapture` and `npm --prefix clients/web test -- --run <targeted file>` when web surfaces move
- **After every plan wave:** Run `make verify && npm --prefix clients/web test`
- **Before `$gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 150 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 06-01-01 | 01 | 1 | WB-01, CONFLICT-01, PROV-01, PEOPLE-01 | unit/schema | `cargo test -p vel-core integration -- --nocapture && cargo test -p vel-api-types -- --nocapture && node scripts/verify-repo-truth.mjs` | ❌ W0 | ⬜ pending |
| 06-02-01 | 02 | 2 | RECON-01, CONFLICT-01, PROV-01 | repository/service | `cargo test -p vel-core ordering -- --nocapture && cargo test -p vel-storage integration_connections_repo -- --nocapture && cargo test -p veld client_sync -- --nocapture` | ❌ W0 | ⬜ pending |
| 06-03-01 | 03 | 3 | WB-01, WB-02, TODO-01, CONFLICT-01, PROV-01 | integration | `cargo test -p veld todoist -- --nocapture && cargo test -p veld integrations -- --nocapture` | ❌ W0 | ⬜ pending |
| 06-04-01 | 04 | 3 | NOTES-01, REMIND-01, WB-02, WB-03, PROV-01 | integration/filesystem | `cargo test -p veld notes -- --nocapture && cargo test -p veld reminders -- --nocapture && cargo test -p veld sync -- --nocapture` | ❌ W0 | ⬜ pending |
| 06-05-01 | 05 | 4 | PEOPLE-01, PEOPLE-02, NOTES-01, PROV-01 | repository/retrieval | `cargo test -p vel-storage semantic_memory_repo -- --nocapture && cargo test -p veld retrieval -- --nocapture && cargo test -p veld people -- --nocapture` | ❌ W0 | ⬜ pending |
| 06-06-01 | 06 | 5 | GH-01, EMAIL-01, WB-01, WB-02, PROV-01, PEOPLE-02 | connector/integration | `cargo test -p veld github -- --nocapture && cargo test -p veld email -- --nocapture && cargo test -p veld integrations -- --nocapture` | ❌ W0 | ⬜ pending |
| 06-07-01 | 07 | 6 | WB-03, CONFLICT-01, PROV-01, RECON-01, PEOPLE-02 | web/cli/doc | `cargo test -p vel-cli review -- --nocapture && npm --prefix clients/web test -- --run src/components/SettingsPage.test.tsx src/components/NowView.test.tsx src/data/operator.test.ts && rg -n "write-back|conflict|people|provenance" docs/api/runtime.md docs/user/daily-use.md docs/user/integrations/README.md` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/vel-core/src/` tests for `NodeIdentity`, ordering primitive, write-back operation kinds, conflict status, and person-record serialization invariants
- [ ] `crates/vel-storage/src/repositories/` tests for write-back history, conflict queue, upstream refs, person aliases, and semantic graph record persistence
- [ ] `crates/veld/src/services/integrations_todoist.rs` tests for safe-op allowlist, label-boundary translation, and conflict persistence
- [ ] `crates/veld/src/services/` tests for note append/create scope enforcement and reminder-intent queued execution behavior
- [ ] `crates/veld/src/services/` tests for GitHub and email denied-operation cases plus draft-first email behavior
- [ ] `crates/veld/src/services/client_sync.rs` and operator queue tests for pending write/conflict projection
- [ ] `clients/web/src/components/SettingsPage.test.tsx` and related data tests for conflict/write-back status rendering
- [ ] `crates/vel-cli/src/commands/` tests for write-back/conflict status output if Phase 6 adds CLI review or status commands

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Conflict prompts clearly distinguish safe auto-applied writes from review-required writes | CONFLICT-01, WB-03 | Wording and operator trust are easier to validate in the real UI/CLI | Trigger one stale-write conflict and one safe write, then confirm the copy distinguishes `applied`, `queued`, and `review required` states |
| Notes writes never escape configured roots or project notes roots | NOTES-01, PROV-01 | Filesystem safety is easiest to confirm with a real path probe | Configure a notes root, attempt a write inside and outside the root, and confirm the out-of-scope path is denied and logged |
| Email remains draft-first and confirm-required for send | EMAIL-01, WB-01 | Human-facing risk threshold is semantic, not just structural | Exercise the email flow and confirm the first resulting object is a draft with a distinct confirm step for any send action |
| Reminder intent queue behaves sanely when no local executor or linked Apple executor is available | REMIND-01, WB-03 | Requires runtime/executor state transitions | Queue a reminder write with no eligible executor and confirm the action remains visible as pending instead of disappearing |
| People merge/link suggestions remain inspectable and reversible | PEOPLE-02 | Identity trust is easiest to assess with live examples | Create two alias-linked person candidates and confirm the UI/CLI shows the source refs before merge/confirm |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 150s for targeted checks
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending

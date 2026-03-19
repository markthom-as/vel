---
phase: 09
slug: backup-first-trust-surfaces-and-simple-operator-control
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-19
---

# Phase 09 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` (Rust unit/integration) + `vitest` 2.1.9 (web) |
| **Config file** | Workspace `Cargo.toml`; `clients/web/vitest.config.ts` |
| **Quick run command** | `cargo test -p veld doctor -- --nocapture` |
| **Full suite command** | `make verify` |
| **Estimated runtime** | ~180 seconds |

---

## Sampling Rate

- **After every task commit:** Run the narrowest affected command plus `cd clients/web && npm test -- --run SettingsPage` when UI trust state changes.
- **After every plan wave:** Run the relevant `veld` and `vel-cli` targeted tests for the touched seam.
- **Before `$gsd-verify-work`:** `make verify` plus a manual CLI backup/export/inspect flow must be green.
- **Max feedback latency:** 180 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 09-01-01 | 01 | 1 | BACKUP-01 | schema/doc | `node scripts/verify-repo-truth.mjs` | ✅ | ⬜ pending |
| 09-01-02 | 01 | 1 | CTRL-01 | Rust/unit | `cargo test -p veld doctor -- --nocapture` | ✅ partial | ⬜ pending |
| 09-02-01 | 02 | 2 | BACKUP-01 | Rust/integration | `cargo test -p veld backup_flow -- --nocapture` | ❌ W0 | ⬜ pending |
| 09-02-02 | 02 | 2 | BACKUP-01 | CLI/integration | `cargo test -p vel-cli backup -- --nocapture` | ❌ W0 |
| 09-03-01 | 03 | 3 | BACKUP-02 | web | `cd clients/web && npm test -- --run SettingsPage` | ✅ partial | ⬜ pending |
| 09-03-02 | 03 | 3 | CTRL-02 | Rust/route | `cargo test -p veld doctor -- --nocapture` | ✅ partial | ⬜ pending |
| 09-04-01 | 04 | 4 | BACKUP-02 | manual + Rust | `cargo test -p veld chat_settings_get_and_patch -- --nocapture` | ✅ but failing baseline | ⬜ pending |
| 09-04-02 | 04 | 4 | CTRL-02 | manual | `vel backup` / inspect / verify flow | ✅ existing command | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `.planning/REQUIREMENTS.md` — define `BACKUP-01`, `BACKUP-02`, `CTRL-01`, `CTRL-02`
- [ ] `config/schemas/backup-manifest.schema.json` — typed backup/export contract
- [ ] `config/examples/backup-manifest.example.json` — checked-in example
- [ ] `docs/user/backup-and-restore.md` — shipped operator workflow authority
- [ ] `crates/veld/tests/backup_flow.rs` — consistent snapshot + omission rules
- [ ] `crates/vel-cli/src/commands/backup.rs` tests — current command has no direct coverage
- [ ] Baseline repair or explicit acknowledgement for `cargo test -p veld chat_settings_get_and_patch -- --nocapture`

*If none: "Existing infrastructure covers all phase requirements."*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Backup pack is inspectable and confidence-building | BACKUP-02 | Operator trust depends on human legibility, not just machine correctness | Run backup/export flow, inspect manifest and omission list, confirm the pack explains what was and was not included. |
| Restore guidance stays narrow and non-destructive | BACKUP-02 | Full restore automation is intentionally out of scope | Follow backup-and-restore docs, validate dry-run/inspect commands, and confirm no destructive restore path is required for phase closure. |
| Control surface remains simple and bounded | CTRL-01 | Simplicity is a UX judgment across CLI/web/settings | Review CLI and web surfaces to confirm they expose effective backup/safety state without becoming a generic config editor. |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 180s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending

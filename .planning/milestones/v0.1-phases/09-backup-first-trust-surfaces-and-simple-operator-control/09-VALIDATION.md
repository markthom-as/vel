---
phase: 09
slug: backup-first-trust-surfaces-and-simple-operator-control
status: ready
nyquist_compliant: true
wave_0_complete: true
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
| **Quick run command** | `node scripts/verify-repo-truth.mjs && cargo test -p vel-api-types backup -- --nocapture` |
| **Full suite command** | `make verify` |
| **Estimated runtime** | ~30 seconds for per-task smoke; longer for focused integration runs |

---

## Sampling Rate

- **After every task commit:** Run the narrowest affected command plus `cd clients/web && npm test -- --run SettingsPage` when UI trust state changes.
- **After every plan wave:** Run the relevant `veld` and `vel-cli` targeted tests for the touched seam.
- **Before `$gsd-verify-work`:** `make verify` plus a manual CLI backup/export/inspect flow must be green.
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 09-01-01 | 01 | 0 | BACKUP-01 | schema/doc | `node scripts/verify-repo-truth.mjs && rg -n "BACKUP-01|BACKUP-02|CTRL-01|CTRL-02|v1 requirements: 32 total|Mapped to phases: 32|Last updated" .planning/REQUIREMENTS.md && node -e "const fs=require('fs'); JSON.parse(fs.readFileSync('config/schemas/backup-manifest.schema.json','utf8')); JSON.parse(fs.readFileSync('config/examples/backup-manifest.example.json','utf8'));" && rg -n "backup-manifest\\.schema\\.json|backup-manifest\\.example\\.json" config/contracts-manifest.json config/README.md && test -f docs/cognitive-agent-architecture/architecture/backup-and-operator-trust-contracts.md && rg -n "manual restore|backup inspect|backup verify" docs/user/backup-and-restore.md` | ✅ | ✅ green |
| 09-01-02 | 01 | 0 | BACKUP-01 | Rust/unit | `cargo test -p vel-api-types backup -- --nocapture` | ✅ partial | ✅ green |
| 09-02-01 | 02 | 2 | BACKUP-01 | Rust/integration | `cargo test -p veld backup_flow -- --nocapture` | ✅ | ✅ green |
| 09-02-02 | 02 | 2 | BACKUP-01 | Rust/integration | `cargo test -p veld backup_flow -- --nocapture` | ✅ | ✅ green |
| 09-03-01 | 03 | 3 | CTRL-01 | Rust/route | `cargo test -p veld chat_settings_get_and_patch -- --nocapture && cargo test -p veld doctor -- --nocapture && cargo test -p vel-cli review -- --nocapture` | ✅ | ✅ green |
| 09-03-02 | 03 | 3 | CTRL-02 | CLI/integration | `cargo test -p veld doctor -- --nocapture && cargo test -p vel-cli doctor -- --nocapture && cargo test -p vel-cli review -- --nocapture` | ✅ | ✅ green |
| 09-04-01 | 04 | 4 | BACKUP-01 | CLI + web | `cargo test -p vel-cli backup -- --nocapture && npm --prefix clients/web test -- --run src/data/operator.test.ts src/components/SettingsPage.test.tsx` | ✅ | ✅ green |
| 09-04-02 | 04 | 4 | CTRL-02 | Rust/integration | `cargo test -p veld backup_flow -- --nocapture && rg -n "nyquist_compliant: true|wave_0_complete: true" .planning/phases/09-backup-first-trust-surfaces-and-simple-operator-control/09-VALIDATION.md` | ✅ | ✅ green |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [x] `.planning/REQUIREMENTS.md` — define `BACKUP-01`, `BACKUP-02`, `CTRL-01`, `CTRL-02`
- [x] `config/schemas/backup-manifest.schema.json` — typed backup/export contract
- [x] `config/examples/backup-manifest.example.json` — checked-in example
- [x] `crates/vel-api-types/src/lib.rs` — transport DTO seam for backup trust and manifest status
- [x] `docs/user/backup-and-restore.md` — shipped operator workflow authority
- [x] Settings baseline issue explicitly isolated from Wave 0 backup evidence; later control-surface slices carry their own `chat_settings_get_and_patch` verification

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

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 30s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** ready for `$gsd-verify-work 09`

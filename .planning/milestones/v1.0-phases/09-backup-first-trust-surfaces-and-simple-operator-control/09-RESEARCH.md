# Phase 09: backup-first-trust-surfaces-and-simple-operator-control - Research

**Researched:** 2026-03-19
**Domain:** Local-first backup/export trust workflows, operator-visible safety state, and intentionally narrow control surfaces
**Confidence:** MEDIUM

## User Constraints

No `CONTEXT.md` exists for this phase. Planning constraints come from repo authority only.

- From `.planning/ROADMAP.md`:
  - Goal: add lightweight backup/export and simple control surfaces that reduce fear of loss, while keeping restore/recovery and advanced policy surfaces intentionally smaller than the core daily loop.
  - Depends on Phase 8, but planning is being requested now.
- From `.planning/STATE.md`:
  - Backup matters mainly as trust against loss; recovery is lower priority than core usability and backup/export confidence.
- From `AGENTS.md` / `CLAUDE.md` / `docs/MASTER_PLAN.md`:
  - Keep route handlers thin and put application logic in services.
  - Do not widen untyped JSON/config blobs without a typed contract.
  - Prefer local-first, inspectable, explainable storage points.
  - New HTTP surfaces should stay auth-by-default and deny-by-default.
  - Canonical contracts, schemas, templates, and docs should land before broad cross-layer implementation.

<phase_requirements>
## Phase Requirements

`BACKUP-01`, `BACKUP-02`, `CTRL-01`, and `CTRL-02` are referenced in `.planning/ROADMAP.md`, but they do not yet exist in `.planning/REQUIREMENTS.md`. The descriptions below are inferred from roadmap/state authority and should be ratified in a Wave 0 contract slice before implementation widens.

| ID | Description | Research Support |
|----|-------------|-----------------|
| BACKUP-01 | Provide an inspectable local backup/export flow that gives the operator confidence their core Vel state can be copied out safely. | Use a typed backup manifest plus SQLite-native snapshotting instead of raw live-file copy; keep output human-inspectable. |
| BACKUP-02 | Surface backup/export confidence clearly without making restore automation the center of the product. | Show last backup/export status, covered paths, omissions, and verification checks in CLI/web; keep restore documented/manual first. |
| CTRL-01 | Keep operator control surfaces simple, typed, and bounded to high-value runtime state. | Extend existing settings/doctor/diagnostics seams rather than adding a generic config editor. |
| CTRL-02 | Make safety state visible so the operator can inspect what is safe to trust before acting. | Reuse doctor/diagnostics/settings/Now trust patterns for backup freshness, export coverage, and safety warnings. |
</phase_requirements>

## Summary

Phase 09 should not invent a new backup subsystem. The codebase already has the right shape for a trust-first slice: `vel config show` exposes effective local paths, `vel doctor` checks database/artifact readiness, `vel backup` exists as manual guidance, `vel export` already emits bounded JSON for some entities, and the web Settings surface already operates as an inspectable control shell rather than a policy editor. The missing piece is not a complex recovery engine. It is a durable, typed, operator-visible backup/export workflow that turns those existing seams into something the operator can trust.

The best planning posture is: ship backup confidence first, restore second. Concretely, that means a typed backup manifest, a safe SQLite snapshot strategy, a small backup/export history surface, explicit omission rules for secrets, and direct verification commands. Do not start with in-app restore automation, retention schedulers, or a broad settings registry. Those would widen scope past the roadmap intent and compete with the core daily loop.

**Primary recommendation:** Plan Phase 09 as four slices: contracts/docs first, snapshot-backed backup service + CLI second, trust/status surfaces third, and restore guidance/manual verification last.

## Recommended Plan Slices

1. **Wave 0: contract and requirement closure**
   - Define `BACKUP-*` and `CTRL-*` in `.planning/REQUIREMENTS.md`.
   - Publish a typed backup/export manifest contract, checked-in example, schema, and owner doc under `config/` and `docs/`.
   - Decide what is included by default: database snapshot, artifact inventory, selected non-secret config values, and optionally local source snapshot paths as references or copies.

2. **Wave 1: backend snapshot and CLI trust path**
   - Add a `veld` backup service that creates a consistent database snapshot plus a manifest and bounded file pack.
   - Replace `vel backup` guidance-only output with a real backup/export command that can create and inspect packs.
   - Keep restore manual in this wave: emit instructions and verification metadata, not full automated restore.

3. **Wave 2: operator-visible safety state**
   - Add backup/export status to `doctor` and/or diagnostics.
   - Add a small Settings or Runtime card showing last successful backup/export, coverage, destination path, and warnings.
   - Reuse the current “inspect before trust” language from Now/Settings/docs.

4. **Wave 3: narrow recovery closure**
   - Add explicit restore docs and a manual verification flow.
   - Optionally add a dry-run restore check or import validator, but keep full restore automation out unless a later roadmap change asks for it.

## Standard Stack

### Core
| Library / Tool | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `axum` | 0.7.9 | Authenticated backup/control HTTP routes in `veld` | Already the repo’s route boundary and auth surface. |
| `sqlx` | 0.8.6 | SQLite access and transaction-safe repository work | Existing storage boundary; Phase 09 should stay inside it. |
| `tokio` | 1.50.0 | File I/O and background-safe snapshot/export work | Already used for async service and file operations. |
| `serde` / `serde_json` | 1.0.228 / 1.0.149 | Typed manifest and status payload serialization | Fits repo contract-first discipline. |
| `time` | 0.3.47 | Stable timestamps for backup manifests and verification output | Already used across persisted contracts. |

### Supporting
| Library / Tool | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `clap` | 4.6.0 | CLI entrypoints for `vel backup` / inspect / verify | For operator-facing backup commands. |
| `react` | 19.2.4 | Settings/runtime trust UI | For small backup status cards in existing web surfaces. |
| `vitest` | 2.1.9 | Web Settings/runtime regression coverage | For UI trust-state rendering and interaction tests. |
| SQLite native backup primitives | Official docs current as of 2025-07-12 | Consistent snapshot of a live SQLite database | Use instead of raw live-file copy. |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Extending the existing CLI/web/runtime stack | A separate backup daemon or external backup product integration | Adds operational complexity and weakens the local inspectability story for the first slice. |
| Typed manifest + plain files | Custom binary archive format | Harder to inspect, verify, and debug; unnecessary at current scope. |
| SQLite backup API / `VACUUM INTO` | Raw `cp` of a live database file | Raw copy can block writers or produce a corrupt backup if interrupted during copy. |

**Installation:**
```bash
# Wave 1 recommendation: no new crates required.
# Stay on the existing workspace stack unless a later slice proves a missing primitive.
```

**Version verification:** Versions above were verified from the current workspace lockfiles and installed web package tree on 2026-03-19. If new crates or npm packages are added during planning, verify their current registry versions at implementation time before updating this stack.

## Architecture Patterns

### Recommended Project Structure
```text
crates/
├── veld/src/services/backup.rs          # typed backup/export orchestration
├── veld/src/routes/backup.rs            # thin operator-authenticated handlers
├── vel-cli/src/commands/backup.rs       # create/list/inspect/verify commands
├── vel-api-types/src/lib.rs             # transport DTOs for backup status/history
└── vel-storage/src/repositories/        # backup history/status persistence if needed

config/
├── schemas/backup-manifest.schema.json
├── examples/backup-manifest.example.json
└── templates/backup-manifest.template.json

docs/
├── user/backup-and-restore.md
└── cognitive-agent-architecture/...     # owner doc for backup/control contract
```

### Pattern 1: Typed Backup Manifest Over a Plain File Pack
**What:** A backup/export operation should write a plain directory pack plus a typed manifest describing what was captured, from where, when, and with what omissions.
**When to use:** For all operator-visible backup/export flows.
**Example:**
```rust
#[derive(Debug, Serialize, Deserialize)]
struct BackupManifest {
    backup_id: String,
    created_at: OffsetDateTime,
    db_snapshot_path: String,
    artifact_root: String,
    included_paths: Vec<String>,
    omitted_paths: Vec<String>,
    secret_settings_omitted: bool,
    verification: BackupVerification,
}
```
// Source pattern: `crates/veld/src/services/execution_context.rs`, `config/schemas/*.json`

### Pattern 2: Bounded Export Writes
**What:** Follow the existing execution-export pattern: compute a typed pack first, then write only within an allowed root.
**When to use:** When backup/export writes to operator-selected directories or repo-local locations.
**Example:**
```rust
let pack = render_backup_pack(&context, output_dir)?;
for file in &pack.files {
    let destination = root.join(&file.relative_path);
    fs::write(&destination, &file.contents).await?;
}
```
// Source pattern: `crates/veld/src/services/execution_context.rs`

### Pattern 3: Effective Settings Overlay, Not Raw Config Editing
**What:** Keep operator control as a small typed overlay on top of runtime config, then surface the effective result.
**When to use:** For backup destination preference, backup warnings toggle, or other narrow control knobs.
**Example:**
```rust
let settings = storage.get_all_settings().await?;
let mut runtime = config.clone();
if let Some(value) = bool_setting(&settings, WRITEBACK_ENABLED_SETTINGS_KEY)? {
    runtime.writeback_enabled = value;
}
```
// Source pattern: `crates/veld/src/services/operator_settings.rs`

### Anti-Patterns to Avoid
- **Live database file copy as the primary backup implementation:** use SQLite-native snapshotting instead.
- **Generic key/value settings editor:** it widens scope and undermines typed inspectability.
- **Backup packs that silently include secrets:** omission must be explicit and visible.
- **Restore-first scope:** full restore automation is larger than the roadmap intent.
- **Web-only backup logic:** policy and durable rules belong in Rust backend layers.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Live SQLite backup | Raw file copy of an in-use DB | SQLite backup API or `VACUUM INTO` | Official SQLite guidance says these create consistent snapshots of a live DB, while raw copy can block writers or corrupt on interruption. |
| Backup archive format | Custom opaque binary blob | Plain files + typed manifest + checksums | Easier to inspect, test, and recover manually. |
| Backup destination policy | Free-form write-anywhere behavior | Same bounded-write pattern used by execution export | Keeps writes reviewable and fail-closed. |
| Control surface expansion | Full policy/config editor | Extend existing Settings/doctor/diagnostics seams | Matches repo intent for simple operator control. |
| Secret export behavior | Ad hoc field filtering in handlers | Centralized manifest omission rules and explicit docs | Prevents prompt/log/export leakage and keeps behavior consistent. |

**Key insight:** The hard part here is not compression or file copying. It is preserving trust: consistent snapshot semantics, explicit omissions, bounded writes, and operator-visible verification.

## Common Pitfalls

### Pitfall 1: Inconsistent backup contents
**What goes wrong:** The database, artifacts, and source files are copied at different logical times, so the backup pack does not describe a coherent state.
**Why it happens:** Treating backup as a series of uncoordinated filesystem copies.
**How to avoid:** Snapshot the database first using SQLite-native backup semantics, timestamp the whole operation, and record copied paths plus omissions in the manifest.
**Warning signs:** Backup verifies structurally but restore docs cannot explain why artifacts/runs do not match the DB snapshot.

### Pitfall 2: Secret leakage through “helpful” export
**What goes wrong:** Public settings and secret settings get mixed into the same backup/export payload.
**Why it happens:** Reusing raw `get_all_settings()` output without a typed export boundary.
**How to avoid:** Default to omitting secrets from backup packs, record omissions explicitly, and require a separate narrow path for secret migration if ever needed.
**Warning signs:** Tokens or provider secrets show up in JSON manifests, logs, fixtures, or UI status cards.

### Pitfall 3: Recovery scope swallowing the phase
**What goes wrong:** Planning drifts into import pipelines, full restore automation, retention jobs, and conflict-replay semantics.
**Why it happens:** Backup work often expands naturally into disaster-recovery tooling.
**How to avoid:** Keep the primary success criteria on backup confidence, inspectability, and manual verification. Treat restore as docs plus narrow validation first.
**Warning signs:** New routes/commands are mostly about restore, not creating or inspecting backups.

### Pitfall 4: Settings becoming a miscellaneous control plane
**What goes wrong:** Backup/control work gets implemented as a generic “advanced settings” surface.
**Why it happens:** Existing settings persistence is easy to append to.
**How to avoid:** Add only typed, high-value controls tied to clear operator decisions and reflect the effective state back immediately.
**Warning signs:** New settings keys lack schema/docs/tests or expose raw internals instead of effective state.

### Pitfall 5: Assuming the current settings baseline is green
**What goes wrong:** Plans build on top of settings behavior without acknowledging current test instability.
**Why it happens:** The web Settings suite is green, but a targeted backend settings test currently fails in the repo baseline.
**How to avoid:** Start Phase 09 with a narrow baseline check and either repair or explicitly account for the failing backend expectation before widening settings work.
**Warning signs:** Route payload assertions disagree between backend and web tests.

## Code Examples

Verified patterns from official sources and the current repo:

### SQLite Live Backup Guidance
```text
Use SQLite's online backup API or VACUUM INTO for the DB snapshot step,
then copy artifacts/config references around that snapshot.
Do not treat raw live-file copy as the primary backup implementation.
```
// Source: https://www.sqlite.org/backup.html
// Source: https://www.sqlite.org/lang_vacuum.html

### Bounded Repo-Local Export
```rust
pub fn render_gsd_artifacts(
    context: &ExecutionContextData,
    output_dir: Option<&str>,
) -> Result<ExecutionArtifactPackData, AppError> {
    let output_dir = normalize_output_dir(output_dir)?;
    Ok(ExecutionArtifactPackData { /* ... */ })
}
```
// Source: `crates/veld/src/services/execution_context.rs`

### Effective Runtime Settings Payload
```rust
map.insert(
    "writeback_enabled".to_string(),
    serde_json::json!(runtime_config.writeback_enabled),
);
```
// Source: `crates/veld/src/services/chat/settings.rs`

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Copy the SQLite file under a lock with shell tools | Use SQLite online backup API or `VACUUM INTO` for live snapshots | Official SQLite docs updated through 2025-07-12 | Better snapshot consistency and clearer tradeoffs around I/O vs CPU. |
| Opaque backup blobs | Plain file packs with manifests/checksums | Current ecosystem norm for inspectable local tools | Better operator trust and easier manual recovery. |
| Generic settings/control panels | Typed effective-state surfaces | Matches Vel Phase 2/5-8 operator-surface discipline | Keeps control smaller than the daily loop and reduces accidental complexity. |

**Deprecated/outdated:**
- Raw live-file DB copy as the main backup method: outdated for this phase because official SQLite docs provide safer live-backup primitives.
- Guidance-only `vel backup`: insufficient as the final Phase 09 answer; keep the command name, but turn it into a typed trust workflow.

## Key Risks

- Phase 09 requirement IDs are not yet defined in `.planning/REQUIREMENTS.md`; planning without fixing that will create avoidable ambiguity.
- Secret-handling policy for backup/export is currently implicit; Phase 09 needs an explicit omission contract.
- The current settings route has baseline test drift: `cargo test -p veld chat_settings_get_and_patch -- --nocapture` fails today.
- The environment here does not have the `sqlite3` CLI installed, so any plan that assumes shelling out to `sqlite3` needs a fallback or a repo-local implementation path.
- Scope creep into restore automation, retention scheduling, or advanced policy UI would violate the roadmap emphasis on backup confidence first.

## Likely Touch Points

Canonical docs and modules most likely touched by this phase:

- `docs/MASTER_PLAN.md`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `config/README.md`
- `config/schemas/app-config.schema.json`
- `config/templates/vel.toml.template`
- `crates/vel-cli/src/commands/backup.rs`
- `crates/vel-cli/src/commands/config.rs`
- `crates/vel-cli/src/commands/doctor.rs`
- `crates/veld/src/services/operator_settings.rs`
- `crates/veld/src/services/chat/settings.rs`
- `crates/veld/src/services/doctor.rs`
- `crates/veld/src/routes/chat.rs`
- `crates/veld/src/routes/diagnostics.rs`
- `crates/veld/src/routes/doctor.rs`
- `crates/veld/src/services/execution_context.rs`
- `clients/web/src/components/SettingsPage.tsx`
- `clients/web/src/components/SettingsPage.test.tsx`
- `docs/user/privacy.md`
- `docs/user/setup.md`
- `docs/user/surfaces.md`
- `docs/user/troubleshooting.md`

## Open Questions

1. **What exactly should a Phase 09 backup pack include by default?**
   - What we know: DB path and artifact root are already first-class, inspectable paths.
   - What's unclear: whether local source snapshots under `var/integrations/` are copied, referenced, or left entirely outside the pack.
   - Recommendation: decide this in Wave 0 and encode it in the manifest as explicit include/omit rules.

2. **Should secret settings ever be exportable?**
   - What we know: Google and Todoist secrets are intentionally stored separately from public settings.
   - What's unclear: whether disaster-recovery convenience ever justifies exporting them.
   - Recommendation: default to no; if needed later, make it a separate, explicit migration flow with its own review and docs.

3. **Does Phase 09 need a restore route or only restore docs plus local CLI verification?**
   - What we know: roadmap/state says recovery stays smaller than the core loop.
   - What's unclear: whether even a dry-run restore validator is needed in this phase.
   - Recommendation: plan manual restore docs and a verify command first; defer destructive restore automation unless a ticket requires it.

4. **How should backup confidence surface in the UI?**
   - What we know: Settings/Now already use small, inspectable trust/recovery cues.
   - What's unclear: whether backup status belongs in Settings only or also in Now/runtime cards.
   - Recommendation: keep backup status in Settings/runtime first; only add a lightweight stale-warning cue elsewhere if confidence truly affects the daily loop.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `cargo test` (Rust unit/integration) + `vitest` 2.1.9 (web) |
| Config file | Workspace `Cargo.toml`; `clients/web/vitest.config.ts` |
| Quick run command | `cargo test -p veld doctor -- --nocapture` |
| Full suite command | `make verify` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| BACKUP-01 | Create a typed backup/export pack with a consistent DB snapshot and inspectable manifest | Rust integration | `cargo test -p veld backup_flow -- --nocapture` | ❌ Wave 0 |
| BACKUP-02 | Surface backup coverage/last-success/warnings to operator surfaces without making restore primary | Web + route integration | `cd clients/web && npm test -- --run SettingsPage` | ✅ partial |
| CTRL-01 | Keep control surfaces typed and narrow across `/api/settings` and CLI | Rust route test | `cargo test -p veld chat_settings_get_and_patch -- --nocapture` | ✅ but failing baseline |
| CTRL-02 | Show operator-visible safety state for backup/config trust | Rust + web integration | `cargo test -p veld doctor -- --nocapture` | ✅ partial |

### Sampling Rate
- **Per task commit:** Run the narrowest affected command plus `cd clients/web && npm test -- --run SettingsPage` when UI trust state changes.
- **Per wave merge:** `cargo test -p veld -- --nocapture` for touched route/service seams and the relevant `vel-cli` crate tests.
- **Phase gate:** `make verify` plus a manual CLI backup/export/inspect flow before `/gsd:verify-work`.

### Wave 0 Gaps
- [ ] `.planning/REQUIREMENTS.md` — define `BACKUP-01`, `BACKUP-02`, `CTRL-01`, `CTRL-02`
- [ ] `config/schemas/backup-manifest.schema.json` — typed backup/export contract
- [ ] `config/examples/backup-manifest.example.json` — checked-in example
- [ ] `docs/user/backup-and-restore.md` — shipped operator workflow authority
- [ ] `crates/veld/tests/backup_flow.rs` — consistent snapshot + omission rules
- [ ] `crates/vel-cli/src/commands/backup.rs` tests — current command has no direct coverage
- [ ] Baseline repair or explicit acknowledgement for `cargo test -p veld chat_settings_get_and_patch -- --nocapture`

## Sources

### Primary (HIGH confidence)
- Repo authority: `AGENTS.md`, `CLAUDE.md`, `README.md`, `docs/MASTER_PLAN.md`
- Planning authority: `.planning/ROADMAP.md`, `.planning/STATE.md`
- Config authority: `config/README.md`, `config/schemas/app-config.schema.json`, `config/templates/vel.toml.template`
- Current implementation seams: `crates/vel-cli/src/commands/backup.rs`, `crates/veld/src/services/operator_settings.rs`, `crates/veld/src/services/chat/settings.rs`, `crates/veld/src/services/doctor.rs`, `crates/veld/src/services/execution_context.rs`
- SQLite Backup API: https://www.sqlite.org/backup.html
- SQLite `VACUUM INTO`: https://www.sqlite.org/lang_vacuum.html

### Secondary (MEDIUM confidence)
- `docs/user/privacy.md`, `docs/user/setup.md`, `docs/user/surfaces.md`, `docs/user/troubleshooting.md` for current trust/recovery language and operator expectations
- `clients/web/src/components/SettingsPage.tsx` and `clients/web/src/components/SettingsPage.test.tsx` for current web control-surface behavior

### Tertiary (LOW confidence)
- None. No unverified community-only sources were used.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - verified from workspace lockfiles, installed web packages, and current repo architecture.
- Architecture: MEDIUM - codebase seams are clear, but the phase lacks a dedicated ticket and requirement definitions.
- Pitfalls: MEDIUM - strongly supported by current repo patterns and official SQLite docs, but final backup scope decisions are still open.

**Research date:** 2026-03-19
**Valid until:** 2026-04-18

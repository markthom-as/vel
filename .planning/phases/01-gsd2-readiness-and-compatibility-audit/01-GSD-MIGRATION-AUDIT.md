# Phase 01 GSD Migration Audit

**Status:** complete
**Milestone:** `0.5.8` GSD migration and phase reset
**Completed:** 2026-04-15

## Verdict

Do not perform a blind cutover from the current `get-shit-done` v1 workflow to `GSD 2`.

The repo has two live planning surfaces:

- legacy `.planning/` state still drives the Codex-facing GSD skills and `gsd-tools.cjs`
- `.gsd/` state already exists and records `M001/S01` as validated, with `M001/S02` active

That means Phase 02 should implement a compatibility bridge or controlled cutover path. The bridge should preserve current Codex workflow behavior while making the `.gsd` state authoritative only where direct checks prove it matches the operator workflow.

## Current v1 Dependency Surface

The active Codex GSD workflow is still materially coupled to the local v1 install:

- install path: `/Users/jove/.codex/get-shit-done`
- version: `1.26.0`
- primary command shim: `node /Users/jove/.codex/get-shit-done/bin/gsd-tools.cjs`
- workflow documents: `/Users/jove/.codex/get-shit-done/workflows/*.md`
- templates and references: `/Users/jove/.codex/get-shit-done/templates/*.md` and `/Users/jove/.codex/get-shit-done/references/*.md`
- Codex skills: `/Users/jove/.codex/skills/gsd-*`

Command-backed checks:

```text
cat /Users/jove/.codex/get-shit-done/VERSION
1.26.0

rg -l "get-shit-done|gsd-tools\\.cjs" /Users/jove/.codex/skills/gsd-* | wc -l
40

rg -l "get-shit-done|gsd-tools\\.cjs" .planning docs README.md AGENTS.md .gsd | wc -l
87
```

The workflow depends on v1-specific structured helpers, including:

- `init progress`
- `init execute-phase`
- `init new-milestone`
- `phase-plan-index`
- `state-snapshot`
- `roadmap analyze`
- `validate health`
- `config-set`
- `state begin-phase`
- `progress bar`

The phase execution flow also assumes v1 file conventions:

- active phase directories live under `.planning/phases/`
- plans are named like `01-01-PLAN.md`
- completion is represented by matching `01-01-SUMMARY.md`
- milestone progress is inferred from plan/summary counts
- the active roadmap remains `.planning/ROADMAP.md`

## Available GSD 2 Surface

The repo already contains a `.gsd/` tree:

```text
.gsd/PROJECT.md
.gsd/REQUIREMENTS.md
.gsd/STATE.md
.gsd/DECISIONS.md
.gsd/milestones/M001/M001-ROADMAP.md
.gsd/milestones/M001/slices/S01/S01-PLAN.md
.gsd/milestones/M001/slices/S01/S01-SUMMARY.md
.gsd/milestones/M001/slices/S02/S02-PLAN.md
.gsd/milestones/M001/slices/S03/S03-PLAN.md
```

That surface is more slice/task oriented than the legacy phase/plan layout. It records:

- active milestone: `M001: GSD Migration and Phase Reset`
- active slice: `S02: GSD 2 Migration Cutover and Codex Integration`
- validated requirement: `R001`, the v1 dependency and migration-constraints inventory

The local shell now exposes `gsd` and `gsd-cli` through the globally installed npm package `gsd-pi@2.75.0`:

```text
command -v gsd
/opt/homebrew/bin/gsd

gsd --version
2.75.0
```

The default shell still resolves `node` to `/opt/homebrew/opt/node@20/bin/node` (`v20.20.1`), while `gsd-pi@2.75.0` declares `node >=22.0.0`. With `/opt/homebrew/opt/node@22/bin` first in `PATH`, Node resolves to `v25.8.1` and basic GSD commands run:

```text
PATH="/opt/homebrew/opt/node@22/bin:$PATH" gsd headless --help
PATH="/opt/homebrew/opt/node@22/bin:$PATH" gsd list
```

This proves an installed GSD 2 command surface exists, but it does not yet prove workflow equivalence. `gsd headless status --timeout 60000 --output-format json` repeatedly reported the milestone complete, then timed out after three 60-second restart attempts. `gsd graph status` failed because the installed package could not resolve `@gsd-build/mcp-server`.

## Compatibility Gaps

### 1. Legacy and GSD 2 state disagree

`.gsd/STATE.md` says S01 is complete and S02 is active. Before this audit was backfilled, `.planning/phases/01-gsd2-readiness-and-compatibility-audit/` had no `01-01-SUMMARY.md` and no `01-GSD-MIGRATION-AUDIT.md`, so legacy progress still treated Phase 01 as unexecuted.

Phase 02 should not assume one state tree can silently replace the other. It needs an explicit compatibility rule for which surface is authoritative for progress, routing, and closeout.

### 2. Codex skills hard-code v1 paths

The Codex skill wrappers reference `/Users/jove/.codex/get-shit-done/...` workflow, template, and reference files. A cutover that removes or bypasses those files would break normal commands such as progress, execute-phase, plan-phase, and health unless the skills are regenerated or bridged.

### 3. v1 structured tools still drive current operator checks

Current progress and phase execution use `gsd-tools.cjs` structured JSON outputs. GSD 2 has an installed command surface, but it currently requires an explicit Node `>=22` path and has unresolved behavior/dependency gaps for `headless status` and `graph status`. Until equivalent progress, health, roadmap, execution, closeout, and graph workflows are checked end to end, migration must be treated as unproven.

### 4. v1 milestone parsing still has drift

`init progress` currently reports `milestone_version: "v0.1"` even though the active planning packet is `0.5.8`. `validate health` is currently healthy after the Phase 01 reconciliation, but the milestone-version drift remains a bridge/cutover concern because it can make automation route against stale historical records.

### 5. Absolute path drift remains visible

Several historical planning references use `/home/jove/...` paths, while this workspace is under `/Users/jove/...`. This does not block the audit, but Phase 02 should avoid introducing more absolute path assumptions and should prefer repo-relative links inside checked-in docs.

## Migration Constraints

Phase 02 must satisfy these constraints before claiming migration:

1. Codex-facing commands continue to work for progress, phase execution, roadmap analysis, health, and closeout.
2. Active milestone discovery remains limited to `.planning/phases/01-*` through `.planning/phases/03-*` or an explicitly documented `.gsd` equivalent.
3. Archived milestone packets under `.planning/milestones/` are not treated as live work.
4. `.gsd` and `.planning` do not silently diverge on the active phase/slice.
5. The chosen path is documented as one of:
   - successful GSD 2 migration
   - compatibility bridge
   - explicit defer with rationale

## Phase 02 Recommendation

Proceed with a compatibility bridge first.

Recommended sequence:

1. Treat `.gsd` as evidence that the GSD 2 migration shape exists, not as proof that Codex workflows can abandon v1.
2. Keep v1 `gsd-tools.cjs` available for Codex skill execution until equivalent GSD 2 commands are runtime-wired and checked.
3. Add or update docs that explain the dual-state period and the source of truth for progress routing.
4. Repair the legacy `.planning` progress artifacts so Phase 01 is complete in both systems.
5. Verify progress, health, roadmap analysis, phase discovery, and next-step routing before closing Phase 02.

## Evidence Commands

```text
cat /Users/jove/.codex/get-shit-done/VERSION
rg -l "get-shit-done|gsd-tools\\.cjs" /Users/jove/.codex/skills/gsd-* | wc -l
rg -l "get-shit-done|gsd-tools\\.cjs" .planning docs README.md AGENTS.md .gsd | wc -l
find .planning/phases -maxdepth 1 -type d | sort
find .gsd/milestones/M001 -maxdepth 3 -type f | sort
node /Users/jove/.codex/get-shit-done/bin/gsd-tools.cjs init progress
node /Users/jove/.codex/get-shit-done/bin/gsd-tools.cjs init new-milestone
node /Users/jove/.codex/get-shit-done/bin/gsd-tools.cjs roadmap analyze
node /Users/jove/.codex/get-shit-done/bin/gsd-tools.cjs validate health
command -v gsd
gsd --version
gsd --help
PATH="/opt/homebrew/opt/node@22/bin:$PATH" gsd headless --help
PATH="/opt/homebrew/opt/node@22/bin:$PATH" gsd list
PATH="/opt/homebrew/opt/node@22/bin:$PATH" gsd headless status --timeout 60000 --output-format json
PATH="/opt/homebrew/opt/node@22/bin:$PATH" gsd graph status
```

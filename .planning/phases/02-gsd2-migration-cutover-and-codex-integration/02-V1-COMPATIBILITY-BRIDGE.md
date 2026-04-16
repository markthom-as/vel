# Phase 02 V1 Compatibility Bridge

**Status:** completed bridge design
**Phase:** `02-gsd2-migration-cutover-and-codex-integration`
**Source:** Phase 01 GSD migration audit

## Purpose

Phase 01 found enough `GSD 2` state to continue the migration, and later verification found an installed `gsd-pi@2.75.0` command surface. That still is not enough workflow-equivalence evidence to cut over blindly.

This bridge keeps the existing Codex-facing `get-shit-done` v1 workflow available while future GSD work proves which `GSD 2` pieces can safely become authoritative.

## Current Dual-State Problem

Vel currently has two planning surfaces:

- `.planning/` remains the workflow surface used by current Codex GSD skills and the v1 `gsd-tools.cjs` helper.
- `.gsd/` exists and records `M001/S02` as the active migration slice.

The surfaces describe the same migration effort in different shapes. `.planning/` is phase/plan oriented. `.gsd/` is milestone/slice/task oriented. They must not be allowed to silently diverge during Phase 02.

## Bridge Source-of-Truth Rules

During the bridge period:

1. `.planning/` remains authoritative for Codex v1 command execution, phase discovery, and legacy progress checks.
2. `.gsd/` is authoritative only for the GSD 2 migration packet shape and for decisions that have been explicitly mirrored or reconciled with `.planning/`.
3. If `.planning/` and `.gsd/` disagree, the discrepancy is a bridge issue, not an implicit migration decision.
4. Archived packets under `.planning/milestones/` are historical evidence unless an active Phase 02 change explicitly references them.
5. New migration claims require command-backed evidence. A document or state entry alone is not enough.

## Codex V1 Command Preservation

Phase 02 preserved the current Codex command surface until a replacement is runtime-wired, dependency-complete, and checked.

Preserved behavior includes:

- progress and next-step routing through the v1 helper
- active phase discovery under `.planning/phases/`
- phase plan and summary conventions such as `02-01-PLAN.md` and matching summaries
- roadmap analysis and health checks, even if known historical warnings remain
- closeout behavior that current `gsd-*` Codex skills expect

Do not remove, rename, or bypass the local v1 workflow path as part of this bridge unless equivalent GSD 2 commands are present, runtime-wired, and verified in the same change.

## GSD 2 Usage Boundaries

During the bridge, `.gsd/` may be used for:

- recording the migration milestone/slice/task shape
- comparing GSD 2 state against legacy `.planning` state
- documenting a future cutover target once command equivalence is known
- tracking decisions that are also reflected in the legacy workflow when they affect active routing

During the bridge, `.gsd/` must not be used as the sole authority for:

- routing Codex skills
- declaring a phase complete
- replacing active `.planning/phases/` discovery
- changing roadmap status without a matching legacy reconciliation
- proving that GSD 2 is feature-complete or workflow-equivalent to the current v1 surface

## Migration, Bridge, or Defer Criteria

Choose **migration** only when all of these are true:

- a concrete GSD 2 command surface is installed, resolves the required Node runtime without ad hoc `PATH` overrides, and has all required runtime dependencies
- progress, health, roadmap analysis, phase/slice discovery, execution, and closeout have checked equivalents
- Codex-facing skills or wrappers route to the new surface without breaking current commands
- `.planning` and `.gsd` agree on active work and completion state
- rollback can restore the v1 command path without reconstructing lost state

Choose **compatibility bridge** when:

- v1 commands remain the only verified complete workflow
- `.gsd` state is useful but not yet sufficient to drive Codex
- the safest next step is explicit reconciliation and preservation
- Phase 02 can reduce ambiguity without broad toolchain replacement

Choose **defer** when:

- the GSD 2 executable surface cannot satisfy its runtime or dependency requirements
- replacing v1 would require editing Codex skills outside the intended scope
- state reconciliation would require speculative interpretation
- verification cannot distinguish active work from archived historical packets

## Rollback Plan

Rollback should be simple because the bridge is conservative.

1. Keep the v1 `get-shit-done` install and absolute command path available.
2. Leave `.planning/phases/` as the active Codex workflow surface.
3. Treat `.gsd/` changes made during Phase 02 as advisory until they are reconciled.
4. If a GSD 2 route fails verification, revert routing to v1 commands and record the failed check in Phase 02 verification notes.
5. Do not delete legacy planning artifacts as part of rollback. Mark superseded bridge notes explicitly instead.

## Phase 02 Verification Checklist

- [x] v1 version check still reports the installed local workflow.
- [x] Codex skill references to `get-shit-done` / `gsd-tools.cjs` are preserved.
- [x] active `.planning/phases/` discovery returns only the current `0.5.8` phase packet.
- [x] `.gsd/STATE.md` and `.planning` active phase state agree after closeout reconciliation.
- [x] progress / next-step routing works through the selected command path.
- [x] roadmap analysis does not treat archived milestone packets as active scope.
- [x] health checks are run and residuals are separated from blockers.
- [x] no claim of GSD 2 cutover is made without workflow-equivalence evidence.
- [x] rollback to v1 command execution remains possible without editing repo history.

## Phase 02 Cutover Position

The selected position is **compatibility bridge**.

Future work may upgrade that position to migration only after the Node runtime, package dependencies, command behavior, and Codex routing have been verified. Until then, v1 remains the operational workflow and `.gsd` remains a migration-state surface that must be reconciled before it drives work.

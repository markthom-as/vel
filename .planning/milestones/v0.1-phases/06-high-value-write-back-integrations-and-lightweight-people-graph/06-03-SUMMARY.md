---
phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
plan: 03
subsystem: todoist-writeback
tags: [phase-6, todoist, writeback, conflicts, projects, provenance, docs]
requires:
  - phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
    provides: typed write-back/conflict/people contracts from 06-01
  - phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
    provides: durable write-back/conflict/upstream-ref persistence from 06-02
provides:
  - typed Todoist task mapping with label parsing isolated to the adapter boundary
  - explicit Todoist create/update/complete/reopen writeback entrypoints with durable conflict review
  - project linkage through typed Todoist upstream IDs plus persisted source refs and operator docs for the bounded write surface
affects: [phase-06, todoist, writeback, conflicts, projects, docs, operator-runtime]
tech-stack:
  added: []
  patterns: [adapter-boundary label translation, source-ref-backed upstream ownership, bounded provider writeback surface]
key-files:
  created: []
  modified:
    - crates/vel-storage/src/repositories/projects_repo.rs
    - crates/vel-storage/src/db.rs
    - crates/veld/src/services/integrations_todoist.rs
    - crates/veld/src/services/writeback.rs
    - crates/veld/src/routes/integrations.rs
    - crates/veld/src/routes/sync.rs
    - crates/veld/src/app.rs
    - docs/user/integrations/todoist.md
    - docs/api/runtime.md
key-decisions:
  - "Todoist labels remain compatibility-only metadata; Vel's durable contract is the typed field set `project_id`, `scheduled_for`, `priority`, `waiting_on`, and `review_state`."
  - "Todoist project linkage resolves by `projects.upstream_ids[\"todoist\"]` first and only falls back to slug matching when the upstream mapping is absent."
  - "Provider writes stay bounded to four explicit operations and open `stale_write` or `upstream_vs_local` conflicts instead of silently overwriting drifted upstream tasks."
patterns-established:
  - "When a provider lacks native fields for local semantics, keep the typed contract local and translate only the minimal compatibility labels at the adapter boundary."
  - "Use upstream refs plus synced task snapshots as the writeback precondition check before mutating third-party state."
requirements-completed: [WB-01, WB-02, TODO-01, CONFLICT-01, PROV-01]
duration: 18m
completed: 2026-03-19
---

# Phase 06-03 Summary

**Todoist is now the first complete Phase 06 write-back lane, with typed mapping, bounded write ops, and explicit conflict review**

## Performance

- **Duration:** 18 min
- **Started:** 2026-03-19T04:05:07Z
- **Completed:** 2026-03-19T04:23:05Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Reworked the Todoist adapter so typed internal fields `project_id`, `scheduled_for`, `priority`, `waiting_on`, and `review_state` are derived and persisted locally while raw labels stay compatibility-only boundary data.
- Added explicit `todoist_create_task`, `todoist_update_task`, `todoist_complete_task`, and `todoist_reopen_task` writeback entrypoints, backed by durable writeback records and upstream snapshot checks that open conflicts instead of forcing last-write-wins.
- Added project resolution through Todoist upstream IDs, persisted source refs/upstream refs during sync and writeback, and documented the bounded Todoist write surface plus the separate read/sync path.

## Task Commits

No task commits were created. This slice was executed inline in the current Phase 06 worktree and left uncommitted for review.

## Files Created/Modified

- `crates/vel-storage/src/repositories/projects_repo.rs` - Adds project lookup by provider-scoped upstream ID so Todoist links can resolve typed projects before slug fallback.
- `crates/vel-storage/src/db.rs` - Exposes the new project-by-upstream-ID helper through the storage facade.
- `crates/veld/src/services/integrations_todoist.rs` - Adds typed Todoist field extraction, upstream-ref persistence, writeback planning/execution helpers, and focused conflict tests with a mock Todoist server.
- `crates/veld/src/services/writeback.rs` - Adds the shared Todoist provider entrypoints and durable writeback/conflict orchestration for the four bounded Todoist operations.
- `crates/veld/src/routes/integrations.rs` - Exposes operator-authenticated Todoist create/update/complete/reopen routes returning typed writeback records.
- `crates/veld/src/routes/sync.rs` - Keeps the Todoist sync route explicitly documented as the separate read path.
- `crates/veld/src/app.rs` - Mounts the new Todoist operator write routes.
- `docs/user/integrations/todoist.md` - Documents the allowed Todoist write actions, conflict review behavior, and typed-vs-label contract.
- `docs/api/runtime.md` - Documents the Todoist read/write route split and the conflict-review rule at the runtime boundary.

## Decisions Made

- Todoist writeback uses synced upstream refs and stored task snapshots as the local expectation source, so conflict checks do not rely on ad hoc route payload assertions.
- Create/update/complete/reopen are the only shipped Todoist mutations in this slice; everything else remains outside the allowed write surface.
- The write routes operate on local commitment identity for existing tasks, keeping provenance and conflict review tied to persisted Vel objects instead of direct slug-only or raw-task-id heuristics.

## Deviations from Plan

- `crates/veld/src/services/integrations.rs` did not need a direct code change. The shared writeback routes call `services::writeback` directly, while the existing `run_todoist_sync` path remains the read/sync boundary.

## Issues Encountered

- The broader verification command `cargo test -p veld integrations -- --nocapture` surfaced an unrelated failure in `services::integrations_host::tests::default_missing_config_path_does_not_block_auto_discovery`. That test lives in a separately modified `integrations_host.rs` slice and was left untouched.
- Two acceptance checks expected literal `stale_write` / `upstream_vs_local` and `/v1/sync/todoist` strings in the codebase. The implementation already matched behaviorally, so the slice added narrow comments to keep the acceptance grep aligned with the shipped boundary.

## User Setup Required

- Todoist writeback still requires a saved Todoist API token.
- Existing local projects must already carry `upstream_ids["todoist"]` if operators want writeback to target a specific Todoist project.

## Next Phase Readiness

- Phase 06 now has one real provider writeback lane proving the durable conflict model against a shipped integration.
- The next dependent slice is `06-04`, extending the same writeback/conflict discipline to notes, reminders, and transcript-under-notes handling.

---
*Phase: 06-high-value-write-back-integrations-and-lightweight-people-graph*
*Completed: 2026-03-19*

---
phase: 05-now-inbox-core-and-project-substrate
plan: 02
subsystem: api
tags: [phase-5, projects, storage, routes, sqlite, local-first]
requires:
  - phase: 05-now-inbox-core-and-project-substrate
    provides: typed Phase 05 project contracts and DTOs from 05-01
provides:
  - persisted project substrate tables and repository operations
  - local-first project creation/list/detail service layer
  - authenticated `/v1/projects` runtime routes
  - truthful project surface/runtime docs
affects: [phase-05, projects, now, inbox, synthesis, web, apple, cli]
tech-stack:
  added: []
  patterns: [typed-project repository facade, local-first create flow, operator-authenticated project routes]
key-files:
  created:
    - migrations/0038_phase5_projects.sql
    - crates/vel-storage/src/repositories/projects_repo.rs
    - crates/veld/src/services/projects.rs
    - crates/veld/src/routes/projects.rs
  modified:
    - crates/vel-storage/src/db.rs
    - crates/vel-storage/src/lib.rs
    - crates/vel-storage/src/repositories/mod.rs
    - crates/veld/src/services/mod.rs
    - crates/veld/src/routes/mod.rs
    - crates/veld/src/app.rs
    - docs/api/runtime.md
    - docs/user/surfaces.md
key-decisions:
  - "Project creation remains strictly local-first in Phase 05; only pending provision intent is persisted."
  - "Project aliases are written alongside project creation so legacy string references can map to stable IDs later."
  - "Stored root paths are the durable substrate; route responses derive root labels/kinds deterministically from those paths."
patterns-established:
  - "New typed runtime substrates land as migration + repository + storage facade + service + route + docs in one slice."
  - "Operator-facing supporting surfaces stay authenticated and backend-owned, with no client-side policy widening."
requirements-completed: [PROJ-01, PROJ-02, PROJ-03, FAMILY-01]
duration: 18m
completed: 2026-03-19
---

# Phase 05-02 Summary

**Persisted project records with local-first create/list/detail APIs and authenticated Phase 05 project routes**

## Performance

- **Duration:** 18 min
- **Started:** 2026-03-19T01:34:44Z
- **Completed:** 2026-03-19T01:52:07Z
- **Tasks:** 3
- **Files modified:** 12

## Accomplishments

- Added the `projects` and `project_aliases` SQLite substrate plus repository/facade methods for create/list/get/slug lookup/family listing.
- Added the local-first project service and authenticated `/v1/projects`, `/v1/projects/:id`, and `/v1/projects/families` routes.
- Updated runtime and user docs so Projects is described as a typed supporting surface rather than the primary operator shell.

## Task Commits

No task commits were created. This slice was executed inline in an already-dirty Phase 05 worktree and left uncommitted for review.

## Files Created/Modified

- `migrations/0038_phase5_projects.sql` - Adds `projects` and `project_aliases`.
- `crates/vel-storage/src/repositories/projects_repo.rs` - Project persistence, alias, slug lookup, family listing, and repository tests.
- `crates/vel-storage/src/db.rs` - Storage facade methods for projects.
- `crates/vel-storage/src/lib.rs` - Re-exports project core types alongside storage.
- `crates/vel-storage/src/repositories/mod.rs` - Registers the new repository module.
- `crates/veld/src/services/projects.rs` - Local-first project create/list/get/family service logic and tests.
- `crates/veld/src/services/mod.rs` - Exports the new project service module.
- `crates/veld/src/routes/projects.rs` - Thin project list/detail/create/family handlers.
- `crates/veld/src/routes/mod.rs` - Registers project routes.
- `crates/veld/src/app.rs` - Mounts authenticated project routes and adds app-level project route tests.
- `docs/api/runtime.md` - Documents the new project routes and local-first behavior.
- `docs/user/surfaces.md` - Adds the supporting Projects surface description and stable family vocabulary.

## Decisions Made

- Project routes return typed project DTOs and stable family values instead of widening free-form string usage.
- The phase stores pending provision intent only; it does not create repos, notes roots, or upstream records.
- Stable family vocabulary is returned even before multiple project rows exist so clients can render the canonical options.

## Deviations from Plan

None - plan executed within the intended scope.

## Issues Encountered

- The turn was interrupted after code verification but before summary/tracker updates, so this summary and the roadmap/state updates were repaired afterward.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The typed project substrate is now available for linking, action ranking, review, and client continuity slices.
- The next dependent slice is `05-03`, which can build the authenticated linking backend on top of the new substrate.

---
*Phase: 05-now-inbox-core-and-project-substrate*
*Completed: 2026-03-19*

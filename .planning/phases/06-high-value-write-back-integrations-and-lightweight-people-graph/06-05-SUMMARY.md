---
phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
plan: 05
subsystem: people-registry-and-semantic-graph
tags: [phase-6, people, semantic-memory, retrieval, provenance, sync, docs]
requires:
  - phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
    provides: typed write-back/conflict/people contracts from 06-01
  - phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
    provides: durable write-back/conflict/upstream-ref persistence from 06-02
  - phase: 06-high-value-write-back-integrations-and-lightweight-people-graph
    provides: project substrate and transcript-under-notes lane from 05-02 and 06-04
provides:
  - minimal practical people persistence with explicit alias records and operator-authenticated people routes
  - provenance-bearing semantic retrieval over projects, notes, transcript_note rows, threads, and people
  - thin-client sync/bootstrap hydration for typed people continuity
affects: [phase-06, people, semantic-memory, retrieval, sync, docs]
tech-stack:
  added: []
  patterns: [typed people registry, alias-driven identity linkage, provenance-bearing semantic indexing, bounded retrieval source sets]
key-files:
  created:
    - migrations/0041_phase6_people_and_graph.sql
    - crates/vel-storage/src/repositories/people_repo.rs
    - crates/veld/src/services/people.rs
    - crates/veld/src/services/retrieval.rs
    - crates/veld/src/routes/people.rs
    - .planning/phases/06-high-value-write-back-integrations-and-lightweight-people-graph/06-05-SUMMARY.md
  modified:
    - crates/vel-core/src/semantic.rs
    - crates/vel-api-types/src/lib.rs
    - crates/vel-storage/src/db.rs
    - crates/vel-storage/src/lib.rs
    - crates/vel-storage/src/repositories/mod.rs
    - crates/vel-storage/src/repositories/projects_repo.rs
    - crates/vel-storage/src/repositories/assistant_transcripts_repo.rs
    - crates/vel-storage/src/repositories/threads_repo.rs
    - crates/vel-storage/src/repositories/semantic_memory_repo.rs
    - crates/veld/src/adapters/notes.rs
    - crates/veld/src/services/mod.rs
    - crates/veld/src/services/client_sync.rs
    - crates/veld/src/services/context_generation.rs
    - crates/veld/src/services/context_runs.rs
    - crates/veld/src/routes/mod.rs
    - crates/veld/src/app.rs
    - docs/cognitive-agent-architecture/cognition/memory-model.md
    - .planning/ROADMAP.md
    - .planning/STATE.md
key-decisions:
  - "People stay practical first: one `people` table plus explicit `person_aliases`, with no opaque auto-merge heuristics."
  - "Semantic graph widening is limited to durable Phase 06 entities the runtime already owns: projects, notes, transcript_note rows, threads, and people."
  - "Every new semantic hit carries stable provenance back to a typed local ID, an upstream external ID, or both."
  - "Context-generation retrieval now uses a bounded Phase 06 source-kind set rather than assuming capture-only recall."
patterns-established:
  - "New durable entities should project through storage, sync/bootstrap, and operator routes in the same slice instead of leaving typed continuity half-wired."
  - "Semantic widening should happen through a dedicated retrieval service boundary so source-kind policy stays explicit and reviewable."
requirements-completed: [PEOPLE-01, PEOPLE-02, NOTES-01, PROV-01]
duration: 21m
completed: 2026-03-19
---

# Phase 06-05 Summary

**Vel now has a practical people registry and a provenance-bearing semantic graph over the durable Phase 06 entities**

## Performance

- **Duration:** 21 min
- **Started:** 2026-03-19T04:43:00Z
- **Completed:** 2026-03-19T05:04:20Z
- **Tasks:** 2
- **Files modified:** 20

## Accomplishments

- Added `people` and `person_aliases` persistence plus storage exports for `create_person`, `list_people`, `get_person`, `upsert_person_alias`, and `list_person_aliases`.
- Added operator-authenticated `GET /v1/people`, `GET /v1/people/:id`, and `POST /v1/people/:id/aliases` routes over a typed `services::people` boundary.
- Expanded semantic source kinds and provenance so projects, notes, transcript-under-notes rows, threads, and people all index and retrieve as first-class typed hits instead of anonymous capture-like records.
- Added a dedicated `services::retrieval` seam and widened context-generation retrieval to the bounded Phase 06 source-kind set.
- Extended sync/bootstrap hydration so thin clients receive typed people lists alongside projects, linked nodes, action items, write-backs, and conflicts.
- Updated the memory-model doc to explicitly name projects, notes, transcripts, threads, and people as the durable graph-linked recall entities.

## Task Commits

No task commits were created. This slice was executed inline in the current Phase 06 worktree and left uncommitted for review.

## Files Created/Modified

- `migrations/0041_phase6_people_and_graph.sql` - Adds the `people` and `person_aliases` tables plus supporting indexes.
- `crates/vel-storage/src/repositories/people_repo.rs` - Implements typed people persistence and alias upsert/list queries.
- `crates/vel-storage/src/repositories/semantic_memory_repo.rs` - Adds Phase 06 source kinds, provenance fields, direct indexing helpers, and rebuild/query coverage for projects, notes, transcripts, threads, and people.
- `crates/vel-storage/src/repositories/projects_repo.rs` - Indexes projects into semantic memory when the project substrate changes.
- `crates/vel-storage/src/repositories/assistant_transcripts_repo.rs` - Indexes transcript rows as `transcript_note` records on insert.
- `crates/vel-storage/src/repositories/threads_repo.rs` - Indexes thread titles/status as durable semantic thread records.
- `crates/veld/src/adapters/notes.rs` - Indexes synced note documents as note-path semantic records.
- `crates/veld/src/services/people.rs` - Adds typed people listing/get/alias orchestration.
- `crates/veld/src/services/retrieval.rs` - Defines the bounded Phase 06 retrieval source-kind policy and query seam.
- `crates/veld/src/services/client_sync.rs` - Hydrates typed people into sync/bootstrap payloads and asserts that in tests.
- `crates/veld/src/routes/people.rs` - Exposes the operator people routes.
- `crates/veld/src/app.rs` - Mounts the new `/v1/people` route family.
- `crates/vel-core/src/semantic.rs` - Adds Phase 06 semantic source kinds and provenance fields.
- `crates/vel-api-types/src/lib.rs` - Adds the typed alias-upsert request plus reverse integration-source conversion for the new route boundary.
- `docs/cognitive-agent-architecture/cognition/memory-model.md` - Documents the narrowed durable entity set for graph-linked retrieval.
- `.planning/ROADMAP.md` - Advances Phase 06 to `5/7` with `06-05` complete.
- `.planning/STATE.md` - Advances the active tracker to `06-06` next.

## Decisions Made

- Alias rows stay explicit and inspectable, with uniqueness on `(platform, handle)` instead of provider-specific merge heuristics.
- Note recall uses note-path provenance derived from the existing notes sync lane, which keeps retrieval tied to configured notes roots rather than inventing a new notes store.
- Transcript recall uses the durable assistant transcript row as the semantic source, preserving the Phase 06 transcript-under-notes model.
- Retrieval widening was implemented in a new service module because the path named in the plan did not exist in the live tree.

## Deviations from Plan

- The migration landed as `migrations/0041_phase6_people_and_graph.sql`, not `0040`, because `0040_phase6_conflicts_and_writebacks.sql` already exists in the current tree from the earlier Phase 06 foundation slice.
- The plan referenced `crates/veld/src/services/retrieval.rs` before that file existed; this slice created the retrieval service and routed the existing context-generation call path through it.
- `crates/veld/src/adapters/notes.rs`, `crates/vel-storage/src/repositories/projects_repo.rs`, `crates/vel-storage/src/repositories/assistant_transcripts_repo.rs`, and `crates/vel-storage/src/repositories/threads_repo.rs` also changed so the widened semantic entity set is indexed at the actual mutation/ingest seams instead of only being available through a manual rebuild path.

## Issues Encountered

- The first compile pass in `semantic_memory_repo.rs` had ownership/mapping mistakes while widening the rebuild path; those were fixed inline before the verification rerun.
- The first retrieval test fixture underweighted project hits because the seeded project text did not share enough query terms; the fixture was tightened so project, note, transcript, thread, and person hits all rank under the same bounded query.

## User Setup Required

- No new operator setup is required for the people registry itself.
- Semantic note recall depends on the existing notes sync lane or bounded note writeback path producing durable note-path records.

## Next Phase Readiness

- Phase 06 now has typed project/action/write-back/conflict foundations, Todoist and local notes/reminders write lanes, a practical people registry, and a widened provenance-bearing retrieval graph.
- The next dependent slice is `06-06`, adding the bounded GitHub and email provider lanes with typed project and people linkage.

---
*Phase: 06-high-value-write-back-integrations-and-lightweight-people-graph*
*Completed: 2026-03-19*

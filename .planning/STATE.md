---
gsd_state_version: 1.0
milestone: 0.5
milestone_name: backend-core-rewrite
release_line: 0.5.0-beta
next_beta_target: 0.5.0-beta
current_phase: 63
current_phase_name: todoist-as-canonical-task-proving-adapter
current_plan: 63-04-PLAN
current_work_id: 0.5.63.4
status: in_progress
stopped_at: "Phase 63 plan 03 completed with ownership-aware Todoist sync, tombstones, and mediated outward writes; active execution advanced into black-box Todoist adapter proof"
last_updated: "2026-03-22T23:59:00Z"
last_activity: 2026-03-22
progress:
  total_phases: 9
  completed_phases: 6
  total_plans: 36
  completed_plans: 29
  percent: 81
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-21)

**Core value:** Reliable, local-first capture and recall that a solo operator can trust — with the runtime infrastructure to safely extend execution to autonomous agents without losing control.
**Current focus:** `0.5` backend-only core rewrite from the frozen canonical object/action/policy/module/workflow packet

Status: `0.5` active at Phase 63
Release Line: 0.5.0-beta
Current Work ID: 0.5.63.4
Current Phase: 63
Current Phase Name: todoist-as-canonical-task-proving-adapter
Current Plan: 63-04-PLAN
Total Plans in Phase: 4
Progress: 81%
Last Activity: 2026-03-22
Last Activity Description: Completed Phase 63 plan 03 with ownership-aware Todoist sync, tombstones, and mediated outward writes, then advanced active execution into Todoist black-box constitutional proof

## Current Position

Phase: 63 (todoist-as-canonical-task-proving-adapter) — IN PROGRESS
Plan: prove Todoist now behaves as a constitutional adapter over canonical task, sync, history, and write-intent law

## Accumulated Context

### Active Decisions

- `0.4.x` is closed and should no longer absorb new implementation work
- `0.5` starts from the frozen packet in `.planning/milestones/v0.5-core-rewrite/`
- Phase 57 is complete and is the authority-lock phase for canonical objects, actions, policy, modules, workflows, sync, and task semantics
- Phase 58 is complete and is the storage/system-of-record substrate phase
- Phase 59 is complete and is the typed action membrane, policy, audit, and hostile-path proof phase
- Phase 60 is complete and is the governed module/bootstrap/activation phase
- Todoist and Google Calendar remain the proving adapters for `0.5`
- UI/client embodiment work is explicitly out of scope for this milestone
- roadmap communication continues to use semver release-line language with four-part work IDs like `0.5.57.1`

### Current Concerns

- the `0.5` packet is large enough that implementation discipline has to stay phase-bounded
- Apple/client embodiment remains out of scope until the backend rewrite lands
- top-level authority docs need to point at `0.5` as active, not future

### Next Step

Execute [63-04-PLAN.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/63-04-PLAN.md) and close Todoist as a black-box proving adapter over canonical task and membrane law.

### Roadmap Evolution

- `0.4.x` closed at Phase 56 with build and focused regression evidence
- `0.5` is now active, and Phase 57 is complete
- active execution has moved into Phase 58 substrate work
- `58-01` has landed the typed ID, envelope, and storage-trait base layer
- `58-02` has landed canonical object, registry, and relation persistence
- `58-03` has landed integration-account, SyncLink, runtime-record, and projection persistence
- `58-04` has landed deterministic bootstrap and migration-artifact replay scaffolding
- `58-05` has landed storage-neutral query, projection rebuild seams, and substrate roundtrip proof
- `59-01` has landed the generic action vocabulary, typed action contracts, registry seam, and initial object actions
- `59-02` has landed shared policy/grant types plus runtime precedence and grant narrowing behavior
- `59-03` has landed ownership overlays plus typed stale/conflict classification
- `59-04` has landed typed audit/explain payloads plus append-only `WriteIntent` dispatch recording
- `59-05` has landed happy-path and hostile-path membrane proof plus typed error-surface verification
- `60-01` has landed typed registry IDs, canonical registry object contracts, a dedicated registry-store seam, and runtime loader service
- `60-02` has landed deterministic core bootstrap, seeded workflow reconciliation, and idempotent bootstrap proof
- `60-03` has landed typed module capability requests plus policy-mediated activation and refusal-path verification
- `60-04` has landed shared core/provider registration proof through canonical module manifests and provider-module registration service
- `61-01` has landed typed workflow context binding plus the minimal lawful workflow-step vocabulary
- `61-02` has landed workflow grant envelopes plus mediated skill invocation over module activation and membrane policy
- `61-03` has landed manual invocation, run records, approval seams, and dry-run runtime evidence over canonical objects
- `61-04` has landed black-box workflow runtime proof plus stable refusal/error-surface tests
- Phase 61 is complete and active execution has advanced into Phase 62 native calendar semantics
- `62-01` has landed native `Calendar` / `Event` object contracts, typed calendar relations, and canonical `event_*` / `calendar_*` content IDs
- `62-02` has landed canonical recurrence contracts, attendee participation types, and bounded occurrence materialization proof
- `62-03` has landed governed availability read-model contracts, projection/materialization, and explainability proof
- Phase 62 is complete and active execution has advanced into Phase 63 Todoist proving-adapter work
- `63-01` has landed deterministic Todoist multi-account linking, idempotent backlog import over canonical objects and `SyncLink`, and multi-account non-collision proof
- `63-02` has landed canonical Todoist task/project mapping, raw-tag plus `task_semantics` interpretation proof, and attached-comment mapping without reopening message/thread ontology
- `63-03` has landed ownership-aware Todoist sync, tombstones by default, conservative outward writes through `WriteIntent`, and continuous `TaskEvent` history across provider and local changes

---
*Last updated: 2026-03-22 after completing `63-03` and advancing active execution to `63-04`*

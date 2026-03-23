---
gsd_state_version: 1.0
milestone: 0.5.1
milestone_name: canonical-client-reconnection
release_line: 0.5.1-beta
next_beta_target: 0.5.1-beta
current_phase: 71
current_phase_name: cleanup-web-proof-and-apple-handoff
current_plan: 71-01-PLAN
current_work_id: 0.5.1.71.1
status: complete
stopped_at: "Phase 71 closed `0.5.1` with explicit accepted debt for live workflow-dispatch browser proof"
last_updated: "2026-03-23T03:58:00Z"
last_activity: 2026-03-22
progress:
  total_phases: 6
  completed_phases: 6
  total_plans: 6
  completed_plans: 6
  percent: 100
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-21)

**Core value:** Reliable, local-first capture and recall that a solo operator can trust — with the runtime infrastructure to safely extend execution to autonomous agents without losing control.
**Current focus:** `0.5.1` canonical client reconnection against the frozen `0.5` backend contracts

Status: `0.5.1` complete at Phase 71
Release Line: 0.5.1-beta
Current Work ID: 0.5.1.71.1
Current Phase: 71
Current Phase Name: cleanup-web-proof-and-apple-handoff
Current Plan: 71-01-PLAN
Total Plans in Phase: 1
Progress: 100%
Last Activity: 2026-03-22
Last Activity Description: Closed Phase 70 and advanced `v0.5.1` into cleanup, proof, and Apple handoff

## Current Position

Phase: 71 (cleanup-web-proof-and-apple-handoff) — COMPLETE
Plan: delete or quarantine stale seams, prove the truthful web line, and publish the Apple handoff packet

## Accumulated Context

### Active Decisions

- `0.4.x` is closed and should no longer absorb new implementation work
- `0.5` is frozen backend authority and is not reopened by `0.5.1`
- `0.5.1` is a truth-alignment milestone, not a redesign or backend renegotiation
- only `Now`, `Threads`, and `System` are first-class surfaces in `0.5.1`
- Apple is out for implementation and gets handoff/spec docs only
- roadmap communication continues to use semver release-line language with four-part work IDs
- `v0.5.1` doctrine is now frozen, including adjacent-section `Now`, single-bound-object `Threads` invocation, one-route `/system`, degraded-state posture, and the pre-frozen configuration action allow-list
- Phase 67 has now frozen the rewrite/quarantine/delete disposition for stale surface and transport seams
- the shared web transport boundary now exists and page-level fetch escapes are gone

### Current Concerns

- the current frontend is low-trust and should be reused only where structurally sound
- browser-executed proof is complete for the shipped truthful surface
- no backend schema negotiation is allowed except for provable bugs
- live browser workflow dispatch proof is accepted deferred debt because no shipped canonical invocation route exists in the frozen `v0.5.1` boundary

### Next Step

`0.5.1` execution is complete. The next step is milestone audit/archive or successor-milestone definition.

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
- `63-04` has landed black-box Todoist adapter proof plus hostile-path error-surface verification
- Phase 63 is complete and active execution has advanced into Phase 64 Google Calendar adapter work
- `64-01` has landed Google Calendar multi-account linking, bounded canonical import, and multi-account/window/idempotence proof
- `64-02` has landed canonical Google calendar/event/attendee mapping with native-first participation and lawful provider-stub fallback
- `64-03` has landed recurrence fidelity, native availability bridging, and Google tombstone transitions without reopening provider-shaped scheduling truth
- `64-04` has landed conservative Google writes through `WriteIntent` plus black-box and hostile-path adapter proof
- `65-01` has landed live canonical write routes for Todoist and Google Calendar plus quarantine of superseded legacy write paths
- `65-02` has landed caller/DTO reconciliation against `WriteIntent` authority with bounded read/configuration compatibility retained in quarantine
- `65-03` has landed milestone-spanning execution-backed proof and explicit deferred-work capture for post-`0.5` work
- `0.5` closed with canonical backend authority, proving adapters, and hard write-path cutover
- `0.5.1` is now active as the canonical client reconnection line
- Phase 66 is complete with the truth doctrine, contract freeze, and milestone lock
- Phase 67 is complete with explicit seam dispositions and kill-list law
- Phase 68 is complete with a shared canonical transport boundary and no page-level direct fetch escapes
- Phase 69 is complete with truthful `Now` rebinding, adjacent task/calendar sections, and an internal write-intent seam for direct commitment completion
- Phase 70 is complete with the three-surface shell collapse, truthful `Threads`, and a minimal authoritative `/system` surface
- Phase 71 is complete with browser proof, Apple handoff, and explicit accepted debt for live workflow-dispatch browser proof

---
*Last updated: 2026-03-22 after closing Phase 70 of milestone `0.5.1`*

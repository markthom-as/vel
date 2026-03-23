# Roadmap: Vel `0.5` Core Rewrite

## Status

Closed milestone packet. `0.5` is complete and verified.

## Milestone Framing

Milestone `0.5` is the backend-only core rewrite that turns Vel into a canonical object-centered authority runtime with:

- first-class canonical objects as the system of record
- a typed action membrane for all reads and writes
- policy, grants, ownership, and audit as mandatory infrastructure
- multi-account integrations as a built-in core concern
- first-class module, skill, and workflow primitives
- Todoist and Google Calendar as the proving adapters for the new core

Implementation-critical clarifications for task semantics, tag interpretation, Google auth posture, recurrence scope, tombstone restore, and dry-run law are locked in [0.5-CLARIFICATION-ADDENDUM.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/0.5-CLARIFICATION-ADDENDUM.md). Concrete task contracts are additionally locked in [TASK-SCHEMA.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/TASK-SCHEMA.md) and [TASK-EVENT-CONTRACT.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/TASK-EVENT-CONTRACT.md).

This milestone intentionally does **not** include new product UI work. The follow-on milestone should rebuild or reconnect UI surfaces against the stabilized backend contracts landed here.

## Scope Guardrails

`0.5` is only about the new backend substrate and adapter-backed proving flows:

- canonical object kernel and relation/storage rewrite
- action/tool membrane and policy engine
- module loading and core module bootstrap
- workflow and skill runtime primitives
- Todoist and Google Calendar adapters with conservative bidirectional sync
- hard cutover to the new system of record

Do not widen this milestone into:

- web UI or Apple/client embodiment work
- remote registries, marketplace, or broad external packaging
- WASM sandbox runtime
- broad connector expansion beyond Todoist and Google Calendar
- speculative automation triggers, visual workflow builders, or planner/product widening

Future-provider note:

Apple Reminders is expected after `0.5` as a future provider of the same canonical `Task` type, especially for routine and maintenance commitments. It should not introduce a separate task species.

## Locked Decisions

- Vel objects are first-class canonical records with stable Vel IDs and external linkage metadata.
- Integrations are multi-account by default.
- Todoist is the main shaping model for task semantics in `0.5`.
- Calendar concepts are first-class in core: calendars, events, recurrence, attendees, locations, and availability.
- Availability is canonical, but can be backed by Vel's internal calendar representation rather than treated as an external-only artifact.
- Recurrence is first-class, while synced recurrence state still defers to the source platform as the applied truth.
- Sync is bidirectional, but field conflict resolution follows explicit field ownership; source-owned fields win.
- Deletes should prefer tombstones/archive semantics with explicit confirmation before destructive outward propagation.
- Workflow and skill primitives must be defined in core and built as backend foundations, not deferred to UI milestone work.
- Modules should actually load in `0.5`, with core modules bootstrapped through the same registry/loader path as Todoist and Google Calendar modules.
- Mutating capability paths should exist, but config defaults must keep external writes read-only until explicitly enabled.
- `0.5` should use a hard cutover rather than a long parallel-run migration lane.

## Requirement Buckets

| ID | Description |
|----|-------------|
| CORE-50-01 | Canonical object kernel exists with typed envelopes, relations, schemas, and external linkage metadata. |
| CORE-50-02 | Vel becomes the system of record for canonical objects, not a thin mirror of provider data. |
| CORE-50-03 | Multi-account integration identity, cursors, provenance, and ownership are built into the core substrate. |
| ACTION-50-01 | All reads/writes flow through a typed action membrane with stable schemas and dry-run support. |
| POLICY-50-01 | Grants, field ownership, confirmation posture, and audit logs govern mutating behavior. |
| MODULE-50-01 | Core, Todoist, and Google Calendar modules load through one registry/loader path. |
| WF-50-01 | Skills and workflows exist as first-class governed backend primitives with typed context binding. |
| CAL-50-01 | Canonical calendar objects cover calendars, events, attendees, recurrence, locations, and availability. |
| TODOIST-50-01 | Todoist tasks/projects/sections/labels/comments and sync state map into canonical Vel objects. |
| GCAL-50-01 | Google Calendar calendars/events/attendees/recurrence/location/availability map into canonical Vel objects. |
| SYNC-50-01 | Bidirectional sync works with explicit ownership and source-favored conflict resolution on source-owned fields. |
| CUTOVER-50-01 | Existing backend seams are retired and the new canonical core becomes the live backend path. |
| VERIFY-50-01 | Milestone verification proves backend authority, adapter correctness, and policy/audit behavior with execution-backed evidence. |

## Phases

- [x] **Phase 57: Architecture freeze, canonical contracts, and milestone lock** - Ratify the object model, action/policy/module/workflow vocabulary, sync ownership rules, and `0.5` scope before implementation spreads.
- [x] **Phase 58: Canonical object kernel and system-of-record storage rewrite** - Build the durable object, relation, account, linkage, sync-cursor, tombstone, and migration substrate that the rest of `0.5` depends on.
- [x] **Phase 59: Action membrane, policy engine, and audit authority** - Make every core read/write flow through typed actions governed by grants, field ownership, confirmation posture, and audit logging.
- [x] **Phase 60: Module loader, registry, and core-module bootstrap** - Land the registry and loader path that can boot core modules first, then load Todoist and Google Calendar through the same governed path.
- [x] **Phase 61: Workflow and skill primitives over canonical objects** - Define and implement the minimal backend runtime for skills, workflows, typed context binding, action composition, and run records.
- [x] **Phase 62: Calendar core model and canonical availability semantics** - Implement the first-class calendar/domain layer that makes recurrence, attendees, location, and availability native to Vel rather than adapter accidents.
- [x] **Phase 63: Todoist multi-account adapter and canonical task cut-in** - Prove the new substrate with full-backlog Todoist sync, task-shaped canonical semantics, conservative bidirectional writes, and ownership-aware conflict handling.
- [x] **Phase 64: Google Calendar multi-account adapter and canonical calendar cut-in** - Add Google Calendar as the second proving adapter with calendar/event sync, recurrence fidelity, availability integration, and conservative writes.
- [x] **Phase 65: Hard cutover, backend contract reconciliation, and milestone verification** - Remove or retire the superseded backend paths, repair callers against the new authority, and close the milestone with execution-backed verification.

## Progress

**Planned execution order:** 57 -> 58 -> 59 -> 60 -> 61 -> 62 -> 63 -> 64 -> 65

| Phase | Requirements | Status |
|-------|--------------|--------|
| 57. Architecture freeze, canonical contracts, and milestone lock | CORE-50-01, CORE-50-02, CORE-50-03, ACTION-50-01, POLICY-50-01, MODULE-50-01, WF-50-01, CAL-50-01, SYNC-50-01 | Complete |
| 58. Canonical object kernel and system-of-record storage rewrite | CORE-50-01, CORE-50-02, CORE-50-03, CUTOVER-50-01 | Complete |
| 59. Action membrane, policy engine, and audit authority | ACTION-50-01, POLICY-50-01, SYNC-50-01 | Complete |
| 60. Module loader, registry, and core-module bootstrap | MODULE-50-01, POLICY-50-01 | Complete |
| 61. Workflow and skill primitives over canonical objects | WF-50-01, ACTION-50-01, POLICY-50-01 | Complete |
| 62. Calendar core model and canonical availability semantics | CAL-50-01, CORE-50-01, SYNC-50-01 | Complete |
| 63. Todoist multi-account adapter and canonical task cut-in | TODOIST-50-01, CORE-50-03, SYNC-50-01 | Complete |
| 64. Google Calendar multi-account adapter and canonical calendar cut-in | GCAL-50-01, CAL-50-01, CORE-50-03, SYNC-50-01 | Complete |
| 65. Hard cutover, backend contract reconciliation, and milestone verification | CUTOVER-50-01, VERIFY-50-01 | Complete |

## Closeout

Milestone `0.5` is closed. The executed closeout evidence is captured in:

- [65-MILESTONE-EVIDENCE.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/65-MILESTONE-EVIDENCE.md)
- [65-DEFERRED-WORK.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/65-DEFERRED-WORK.md)

The next milestone has not been activated yet. Follow-on work should start by defining a successor packet rather than silently extending `0.5`.

## Phase Details

### Phase 57: Architecture freeze, canonical contracts, and milestone lock

**Goal:** define the lawful substrate of `0.5` before code spreads across storage, adapters, workflows, or modules.
**Requirements:** CORE-50-01, CORE-50-02, CORE-50-03, ACTION-50-01, POLICY-50-01, MODULE-50-01, WF-50-01, CAL-50-01, SYNC-50-01
**Depends on:** `0.4.x` roadmap truth plus the imported `v0.5` reference packs
**Success Criteria:**
1. Durable docs define the canonical object envelope, typed object families, relation model, external linkage model, and multi-account integration identity rules.
2. Field ownership, source-favored conflict resolution, tombstone/delete posture, and read-only-by-default external write policy are explicit and reviewable.
3. The module, skill, workflow, and action vocabulary is locked well enough that downstream phases do not need to re-decide core nouns.
4. The calendar model explicitly covers calendars, events, attendees, recurrence, location, and availability as first-class canonical concepts.
5. `Todoist` and `Google Calendar` proving scope is written tightly enough that adapter phases cannot widen into broad connector-platform work.
**Plans:** 3-4 plans

### Phase 58: Canonical object kernel and system-of-record storage rewrite

**Goal:** build the new durable substrate that makes Vel the live system of record for canonical objects and integration state.
**Requirements:** CORE-50-01, CORE-50-02, CORE-50-03, CUTOVER-50-01
**Depends on:** Phase 57
**Success Criteria:**
1. Canonical object storage supports stable Vel IDs, typed object families, typed relations, and canonical metadata without collapsing back into untyped blobs.
2. Integration accounts, source links, sync cursors, ownership metadata, provenance, and tombstones live in the same substrate as first-class state.
3. Migration scaffolding exists to cut existing durable state into the new model without requiring a permanent dual-write lane.
4. The resulting storage and repository seams still honor repo layering rules rather than pushing transport DTO or adapter logic downward.
**Plans:** 4-5 plans

### Phase 59: Action membrane, policy engine, and audit authority

**Goal:** make the new core lawful so every read/write path is named, typed, auditable, and governable.
**Requirements:** ACTION-50-01, POLICY-50-01, SYNC-50-01
**Depends on:** Phase 58
**Success Criteria:**
1. Core object reads, queries, mutations, adapter writes, and workflow-triggered operations execute through one typed action registry.
2. Grants, ownership rules, and confirmation posture can deny or permit actions without hidden bypasses.
3. Mutating actions support dry-run and emit audit/run records with enough evidence to explain who tried to change what and why.
4. Read-only-by-default external write posture is configuration-backed rather than a scattered adapter convention.
**Plans:** 4 plans

### Phase 60: Module loader, registry, and core-module bootstrap

**Goal:** land the governed loading path that can bootstrap Vel's own core modules and then load provider modules through the same system.
**Requirements:** MODULE-50-01, POLICY-50-01
**Depends on:** Phase 59
**Success Criteria:**
1. Module manifests, registry records, validation rules, and loader behavior are implemented and documented.
2. Core modules load through the same registry path as integration modules, with privilege differences expressed through policy rather than a totally separate architecture.
3. Todoist and Google Calendar modules can declare requested capabilities, canonical object surfaces, and sync/action entry points before adapter logic fully lands.
4. The loader is narrow and explicit enough to avoid becoming an unbounded plugin marketplace substrate in this milestone.
**Plans:** 3 plans

### Phase 61: Workflow and skill primitives over canonical objects

**Goal:** establish backend workflow and skill foundations that compose actions, context, tools, and policy over canonical objects.
**Requirements:** WF-50-01, ACTION-50-01, POLICY-50-01
**Depends on:** Phase 60
**Success Criteria:**
1. Skills and workflows have typed manifests, typed context binding, stable run records, and policy-governed access to actions/tools.
2. Workflows can compose actions and skills over arbitrary canonical objects structurally, even if only a subset is exercised first.
3. Manual invocation works end-to-end, while triggers, advanced branching, and UI affordances remain explicitly deferred.
4. Mutation-capable workflow paths exist, but config defaults keep external writes read-only unless the operator enables them.
**Plans:** 3-4 plans

### Phase 62: Calendar core model and canonical availability semantics

**Goal:** make calendar semantics native to Vel before the Google adapter lands so the provider maps into Vel instead of dictating it.
**Requirements:** CAL-50-01, CORE-50-01, SYNC-50-01
**Depends on:** Phase 61
**Success Criteria:**
1. Canonical calendar objects exist for calendars, events, recurrence/series state, attendees, and locations.
2. Availability is represented as a first-class canonical view or state backed by the internal calendar model rather than treated as an adapter-specific afterthought.
3. Recurrence preserves source-governed applied truth while still giving Vel a first-class series/occurrence vocabulary.
4. The calendar model is rich enough that Google Calendar integration does not need to invent shadow semantics in its adapter layer.
**Plans:** 3 plans

### Phase 63: Todoist multi-account adapter and canonical task cut-in

**Goal:** prove the new architecture against the operator's primary task system and let Todoist shape the first practical task semantics in core.
**Requirements:** TODOIST-50-01, CORE-50-03, SYNC-50-01
**Depends on:** Phase 62
**Success Criteria:**
1. Multi-account Todoist connection, identity, backlog sync, and cursor/reconciliation state work against the core substrate.
2. Tasks, projects, sections, labels, comments, due semantics, priorities, and completion state map into first-class Vel objects or linked metadata without provider leakage owning the ontology.
3. Bidirectional sync exists with explicit field ownership and conservative write posture; source-owned fields win on conflicts.
4. Delete/archive/tombstone behavior is explicit, reviewable, and safe by default rather than a hidden destructive side effect.
**Plans:** 4 plans

### Phase 64: Google Calendar multi-account adapter and canonical calendar cut-in

**Goal:** prove the second half of the MVP through a rich calendar adapter that exercises the canonical calendar model honestly.
**Requirements:** GCAL-50-01, CAL-50-01, CORE-50-03, SYNC-50-01
**Depends on:** Phase 63
**Success Criteria:**
1. Multi-account Google Calendar linking, backlog sync, and ongoing reconciliation work through the new account and cursor model.
2. Calendars, events, attendees, recurrence, location, and availability all map through canonical Vel concepts instead of remaining provider-native sidecars.
3. Conservative bidirectional writes exist behind config gates, with conflict handling and source ownership explicit at the field level.
4. The adapter proves the calendar model can support real provider complexity without reopening Phase 57's core contract decisions.
**Plans:** 4 plans

### Phase 65: Hard cutover, backend contract reconciliation, and milestone verification

**Goal:** make the new core the only live backend path, remove superseded seams, and close `0.5` with real evidence.
**Requirements:** CUTOVER-50-01, VERIFY-50-01
**Depends on:** Phase 64
**Success Criteria:**
1. The new canonical object core is the live backend system of record and superseded backend paths are retired or explicitly isolated.
2. Remaining callers are reconciled to the new backend authority, even if that required breaking and repairing prior internal contracts during the milestone.
3. Verification demonstrates typed action authority, policy/audit coverage, module loading, workflow primitives, Todoist sync, and Google Calendar sync through execution-backed checks.
4. Any deferred UI, trigger, registry, or broader ecosystem work is written down explicitly rather than silently assumed to be part of `0.5`.
**Plans:** 3 plans

---
*Drafted: 2026-03-22 from operator interview decisions plus imported `v0.5` reference packs*

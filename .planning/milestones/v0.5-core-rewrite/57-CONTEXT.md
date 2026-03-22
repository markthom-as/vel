# Phase 57: Architecture Freeze, Canonical Contracts, and Milestone Lock - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 57 is the contract-lock phase for milestone `0.5`.

It defines the lawful backend substrate precisely enough that downstream implementation phases can build storage, action routing, modules, workflows, and adapters without re-deciding the core ontology.

This phase is intentionally architecture- and contract-heavy. It should not widen into broad implementation beyond the minimum checked-in contract, schema, and migration-shape artifacts needed to keep later phases honest.

</domain>

<decisions>
## Locked Decisions

### Canonical taxonomy
- **D-01:** `0.5` uses four entity classes: `content`, `registry`, `read_model`, and `runtime`.
- **D-02:** Canonical content objects are `Task`, `Project`, `Event`, `Calendar`, `Thread`, `Message`, `Person`, `Tag`, `IntegrationAccount`, `SyncLink`, `Workflow`, and `Config`.
- **D-03:** Canonical registry objects are `Module`, `Skill`, and `Tool`.
- **D-04:** `Availability` is a governed read model, not a first-class content object in `0.5`.
- **D-05:** Runtime/control records such as `RunRecord`, `AuditEntry`, `Grant`, `Approval`, `SyncJob`, `WriteIntent`, and projection/materialization records stay outside the canonical content substrate.

### Core object and linkage model
- **D-06:** Durable canonical objects share one narrow universal envelope.
- **D-07:** The envelope may expose an optional `source_summary` convenience field.
- **D-08:** Canonical linkage truth remains external through `SyncLink` objects and typed relations; `source_summary` is derived and compact, not a second source of truth.
- **D-09:** `IntegrationAccount` is a first-class canonical object; credentials stay in secure external secret state.
- **D-10:** `SyncLink` is a first-class canonical object with historical state, not embedded only as per-object JSON.
- **D-11:** IDs should be typed and prefixed for content objects.
- **D-12:** Relations should be strongly typed and directional from day one.
- **D-13:** Schemas should stay tight in `0.5`, with only narrow governed extension packs/facets.

### Task and calendar domain shape
- **D-14:** Canonical `Task.status` should stay small and provider-neutral enough to map other systems later.
- **D-15:** Todoist sections remain non-first-class in `0.5`.
- **D-16:** Todoist labels map to canonical `Tag` plus provider facet.
- **D-17:** Todoist comments become `AttachedCommentRecord` attached records, not canonical `Message` by default.
- **D-18:** `Message` remains a first-class canonical object for `Thread`.
- **D-19:** Google Calendar attendees use `Person` links plus participation metadata.
- **D-20:** Recurrence should be modeled as series plus derived/materialized occurrence views, with explicit exception records when needed.
- **D-21:** Locations stay a simple canonical payload on `Event`; first-class `Place` is deferred.

### Action membrane and policy
- **D-22:** The membrane is grounded in object-generic actions first, with typed domain aliases layered on top.
- **D-23:** Field ownership uses hybrid static defaults plus dynamic overlays.
- **D-24:** `policy.explain` and `object.explain` are contract requirements in Phase 57.
- **D-25:** Read-only controls must exist at workspace, module, and integration-account levels.
- **D-26:** External-provider writes should always move through an explicit `WriteIntent` runtime record, even when execution is effectively immediate.

### Module, skill, and workflow posture
- **D-27:** All workflows live in canonical object storage, including seeded built-ins.
- **D-28:** Built-in workflows are seeded into canonical storage on bootstrap/import so references, policy, and audit use one uniform workflow substrate.
- **D-29:** Seeded workflows require fork-before-modify unless explicitly marked editable.
- **D-30:** Most seeded built-ins should be immutable or forkable; editable is reserved for clearly local/operator-owned seeded workflows.
- **D-31:** Editable seeded workflows require explicit reconciliation/version-drift fields.
- **D-32:** Registry objects use stable human-readable IDs.
- **D-33:** Canonical registry IDs should be dot-delimited semantic identifiers such as `module.integration.todoist` and `module.integration.google-calendar`.
- **D-34:** Skills and tools are canonical registry objects with manifest-backed definitions and optional persisted overlays, not pure ghost assets and not ordinary content objects.
- **D-35:** Core modules should be organized by concern rather than as one privileged god-module.
- **D-36:** Skills do not call raw tools directly; all capability use goes through the membrane or explicit mediated runtime calls.
- **D-37:** Hooks may be named and reserved in the contract, but executable hooks are deferred.

### Adapter and cutover posture
- **D-38:** Todoist proves the task-side adapter model; Google Calendar proves the calendar-side model.
- **D-39:** Google Calendar default sync window should be bounded at past 90 days / future 365 days, with explicit expansion.
- **D-40:** Upstream deletes should create local tombstones immediately with reconciliation state.
- **D-41:** Tombstones stay hidden from default queries unless explicitly requested.
- **D-42:** Compatibility DTO scaffolding is allowed during the milestone but Phase 57 should define removal criteria now.
- **D-43:** Phase 57 should define the migration artifact format rather than leaving it fully implicit for Phase 58.

### Rust backend implementation posture
- **D-44:** Phase 57 must lock backend-facing crate or module boundary guidance rather than leaving the multiplatform Rust shape implicit.
- **D-45:** Domain logic must be storage-agnostic behind explicit store and transaction traits.
- **D-46:** Canonical objects and runtime/control records need explicit serde and wire-compatibility rules.
- **D-47:** Typed Rust ID newtypes are the recommended backend model even when wire forms are strings.
- **D-48:** Feature gating should separate core, providers, storage engines, secrets, and runtime extras explicitly.
- **D-49:** The backend contract must define an error taxonomy, optimistic concurrency posture, deterministic bootstrap rules, and migration framework expectations.
- **D-50:** Secret access, scheduling/runtime execution, and query behavior must remain behind explicit backend traits rather than leaking platform details into domain code.
- **D-51:** Projection/read-model layers stay distinct from canonical truth layers and remain rebuildable.
- **D-52:** Phase 57 should publish a target capability matrix and testability contract for multiplatform Rust execution.

</decisions>

<specifics>
## Specific Ideas

- The canonical store should hold the durable semantic world; runtime/control records should remain adjacent infrastructure rather than being shoved into the same ontology.
- `Workflow` should be treated more like durable product/program logic than like a manifest alias, because it needs provenance, mutability rules, policy attachment, and stable references.
- `Skill` and `Tool` should be canonical enough to explain, permission, deprecate, and pin, while remaining manifest-defined in implementation.
- `source_summary` should exist because explain/debug/query ergonomics will need it broadly, but it should stay compact so it does not metastasize into a fake linkage layer.

</specifics>

<canonical_refs>
## Canonical References

### Existing repo authority
- `docs/MASTER_PLAN.md`
- `README.md`
- `.planning/PROJECT.md`
- `.planning/milestones/v0.5-core-rewrite/ROADMAP.md`
- `.planning/milestones/v0.5-core-rewrite/CONTEXT.md`

### Phase 57 inputs
- `.planning/milestones/v0.5-core-rewrite/PHASE-57-INTERVIEW-NOTES.md`
- `.planning/milestones/v0.5-core-rewrite/57-DEPENDENCY-AND-INVARIANTS.md`
- `.planning/milestones/v0.5-core-rewrite/57-RUST-BACKEND-CONTRACT-MATRIX.md`
- `.planning/milestones/v0.5-core-rewrite/57-TEST-PROVING-LADDER.md`
- `.planning/milestones/v0.5-core-rewrite/57-RISK-SPIKES.md`
- `.planning/milestones/v0.5-core-rewrite/REFERENCE_PACKS.md`
- `.planning/milestones/v0.5-core-rewrite/reference-packs/vel-core-arch-pack/docs/02-core-object-model.md`
- `.planning/milestones/v0.5-core-rewrite/reference-packs/vel-core-arch-pack/docs/04-module-system.md`
- `.planning/milestones/v0.5-core-rewrite/reference-packs/vel-core-arch-pack/docs/05-core-tools-api.md`
- `.planning/milestones/v0.5-core-rewrite/reference-packs/vel-core-arch-pack/docs/06-permissions-and-policy.md`
- `.planning/milestones/v0.5-core-rewrite/reference-packs/vel-skill-workflow-pack/docs/04-runtime-architecture.md`
- `docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md`
- `docs/cognitive-agent-architecture/architecture/storage-layer.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable strengths
- Repo layering rules are already explicit and should remain intact.
- Existing typed DTO discipline, config/schema publication, and auth-default posture are worth preserving.
- Existing execution/audit/tracing expectations can be reused for actions, modules, and workflows.

### Current gaps that `0.5` must close
- The live tree still lacks a true canonical object-centered substrate.
- Multi-account identity and sync-linkage are not first-class enough for the proving integrations.
- Workflow/module/skill/tool governance is not yet normalized into one coherent backend model.
- Adapter semantics still risk being provider-led unless Phase 57 freezes the contract clearly.
- The backend-safe contract still needs explicit dependency discipline and spike-driven risk validation so the architecture packet remains executable rather than decorative.

</code_context>

<deferred>
## Deferred Ideas

- UI/client embodiment against the new core
- executable hooks
- broader connector expansion
- visual workflow authoring
- remote registries and marketplace
- WASM execution substrate

</deferred>

---

*Context gathered: 2026-03-22 for Phase 57 planning*

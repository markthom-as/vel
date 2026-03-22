# Phase 63: Todoist Multi-Account Adapter And Canonical Task Cut-In - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 63 is the task-side proving adapter phase for milestone `0.5`.

Its job is to prove the new canonical substrate, lawful membrane, governed registry/runtime, and task-shaped core semantics against the operator's actual MVP task system: Todoist.

Phase 63 is authored against the authoritative intended outputs of Phases 57 through 62 planning packets, even where those artifacts are not yet materialized in implementation code or final filesystem locations.

Phase 63 defines the Todoist adapter boundary, multi-account identity and backlog sync behavior, canonical mapping for Todoist tasks/projects/sections/labels/comments, conservative bidirectional writes, and ownership-aware reconciliation. It does not define Google Calendar behavior, generalized connector expansion, or UI/client task surfaces.

This phase should not widen into:

- Google Calendar adapter work
- generic connector-platform abstractions beyond what Todoist needs to prove the core
- UI/client embodiment of Todoist-backed tasks
- workflow-trigger automation over Todoist events
- non-Todoist task ontology redesign
- broad cross-provider merge machinery

</domain>

<decisions>
## Locked Dependencies

- **D-01:** Phase 57 locked the task-side core shape: canonical `Task`, `Project`, `Tag`, `Thread`, `Message`, `IntegrationAccount`, and `SyncLink`; Todoist sections remain non-first-class; labels map to canonical `Tag` plus provider facet; comments become `AttachedCommentRecord`, not canonical `Message` by default.
- **D-01a:** [0.5-CLARIFICATION-ADDENDUM.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/0.5-CLARIFICATION-ADDENDUM.md) locks the canonical task status enum, canonical due/priority/description/subtask posture, typed `TodoistTaskFacet`, tag-to-`task_semantics` interpretation law, dry-run law, and Google-shared auth/governance posture for future expansion.
- **D-01b:** `task_type` is first-class in `0.5`; no separate stored `is_routine` boolean exists; the normalized task-side history type is `TaskEvent`; local Vel-originated writes must also emit task history with explicit provenance.
- **D-02:** Phase 58 owns canonical storage for accounts, SyncLinks, runtime records, tombstones, and projections.
- **D-03:** Phase 59 owns ownership/conflict semantics, read-only posture, `WriteIntent`, audit, and policy mediation for adapter writes.
- **D-04:** Phase 60 owns `module.integration.todoist` registration and governed activation path.
- **D-05:** Phase 61 owns manual workflow runtime; Todoist-backed tasks may later be consumed by workflows, but this phase does not widen into workflow-trigger automation.
- **D-06:** Todoist is the main shaping model for task semantics in `0.5`, but the canonical task model still remains Vel-owned rather than becoming a raw provider mirror.
- **D-07:** Sync is bidirectional, but field conflict resolution follows explicit field ownership; source-owned fields win.
- **D-08:** Upstream deletes create local tombstones with reconciliation state by default.

### Phase 63-specific posture

- **D-09:** Multi-account support is required from the start rather than treated as future cleanup.
- **D-10:** Todoist backlog import may be broad, but it still reconciles into canonical objects and linkage truth rather than mirroring blindly.
- **D-11:** Todoist projects are first-class canonical `Project` mappings; sections remain project-scoped/provider-shaped in `0.5`.
- **D-11a:** Canonical task has one primary project/container relation only in `0.5`; sections stay provider-facet metadata.
- **D-12:** Labels map to canonical tags with provider facets preserved.
- **D-13:** Comments remain attached records with author/timestamp/body metadata; they do not automatically become canonical thread/message content.
- **D-14:** Conservative bidirectional writes exist, but external mutation remains config-gated and mediated through policy and `WriteIntent`.
- **D-15:** Provider modules must not bypass registry, membrane, or policy law just because this is the first proving adapter.
- **D-16:** Todoist task ownership and outward-write behavior must conform to the checked-in [0.5-FIELD-OWNERSHIP-MATRIX.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/0.5-FIELD-OWNERSHIP-MATRIX.md) artifact rather than adapter-local convention.
- **D-17:** Todoist must be treated as both a current-state adapter and an event-capable behavioral-history source where provider activity or delta signals are available; current task state, sync/linkage state, and append-only history are distinct layers.

</decisions>

<specifics>
## Specific Ideas

- The main risk in Phase 63 is letting Todoist's practical shape quietly become the ontology instead of a proving adapter into Vel's task model.
- The second main risk is treating full-backlog sync as permission to skip ownership, tombstone, and reconciliation rules.
- A third risk is reducing Todoist to current task state only and losing the provider history needed for reschedule, rewrite, and churn analysis.
- This phase should prove that Todoist can be mapped honestly into canonical Vel objects while still preserving source-owned truth and conservative outward-write posture.
- The adapter should remain boringly constitutional: same registry path, same membrane, same `WriteIntent` law, same multi-account infrastructure.
- Tag interpretation and derived task semantics should be proved concretely, not left as a theoretical future seam.

</specifics>

<canonical_refs>
## Canonical References

- `.planning/milestones/v0.5-core-rewrite/57-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/58-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/59-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/59-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/60-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/60-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/61-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/62-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/ROADMAP.md`
- `.planning/milestones/v0.5-core-rewrite/0.5-CLARIFICATION-ADDENDUM.md`
- `.planning/milestones/v0.5-core-rewrite/0.5-FIELD-OWNERSHIP-MATRIX.md`
- `docs/cognitive-agent-architecture/architecture/0.5-action-membrane-and-policy.md`
- `docs/cognitive-agent-architecture/architecture/0.5-ownership-conflict-and-write-intent.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable strengths

- The packet already has lawful multi-account, linkage, tombstone, membrane, and registry seams that Todoist should consume instead of re-inventing.
- The workflow/runtime packet already keeps task execution consumers separate from adapter sync concerns.
- The task-side object model is already narrow enough to preserve Vel ownership while still biasing practical semantics toward Todoist.

### Main dangers

- provider-first task semantics flattening canonical ownership
- sync logic bypassing `WriteIntent` because Todoist feels "simple"
- comments getting forced prematurely into canonical thread/message semantics
- sections accidentally becoming first-class just because the provider exposes them

</code_context>

<deferred>
## Deferred Ideas

- Google Calendar task/event interplay
- UI/client task surfaces
- generalized connector-platform expansion
- workflow-trigger behavior over Todoist changes
- richer collaboration semantics for comments/activity
- Apple Reminders as a future routine-oriented provider over the same canonical `Task` type

</deferred>

---

*Context gathered: 2026-03-22 for Phase 63 planning*

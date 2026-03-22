# Milestone `0.5` Core Rewrite - Context

**Gathered:** 2026-03-22
**Status:** Ready for deeper phase planning

<domain>
## Milestone Boundary

`0.5` is a backend-only rewrite milestone. Its job is to replace the current backend substrate with a canonical object-centered authority core that can safely power future UI and automation work.

This milestone is intentionally not a UI milestone. The operator wants the next milestone to reconnect or rebuild client surfaces against the backend that lands here.

</domain>

<decisions>
## Locked Decisions

### System of record and integration posture
- **D-01:** Vel canonical objects become the system of record in `0.5`.
- **D-02:** Todoist and Google Calendar are the proving integrations because they are the operator's actual MVP.
- **D-03:** Integrations must support multiple accounts by default rather than treating multi-account as a follow-on concern.
- **D-04:** External systems map into Vel objects through external linkage and ownership rules; they do not remain the only semantic truth.
- **D-05:** Sync is bidirectional, but field conflict resolution follows explicit field ownership.
- **D-06:** Source-owned fields win in conflicts.
- **D-07:** Deletes should default to tombstone/archive semantics with explicit confirmation before destructive outward propagation.
- **D-08:** The milestone should use a hard cutover rather than a long shadow or dual-run lane.

### Core object and domain shape
- **D-09:** Vel objects remain first-class and stable, not temporary wrappers around provider payloads.
- **D-10:** Todoist should be the main shaping model for task semantics in `0.5`.
- **D-11:** Calendar concepts must be first-class in core: calendars, events, recurrence, attendees, locations, and availability.
- **D-12:** Availability is canonical, likely backed by Vel's internal calendar representation rather than treated as an external-only byproduct.
- **D-13:** Recurring events must be first-class in core, but applied recurrence state should still defer to the source integration's actual state.
- **D-14:** Calendars themselves should be first-class canonical objects, not just containers around events.

### Workflow, module, and execution posture
- **D-15:** Workflow primitives need real backend definitions in `0.5`, including actions composed of skills, access/policy, tools, and typed context.
- **D-16:** The workflow runtime should stay minimal in this milestone: manual invocation, typed context binding, policy/audit, and action composition are in scope; trigger explosion and UI authoring are not.
- **D-17:** Module loading is in scope, not just module spec documents.
- **D-18:** Core modules should load first, then Todoist and Google Calendar modules, through the same registry/loader path.
- **D-19:** Mutation-capable execution should exist, but external writes must default behind config to read-only.

### Milestone scope and non-goals
- **D-20:** `0.5` is comfortable breaking internal contracts now if that is required to land the new core cleanly.
- **D-21:** UI/client work is intentionally deferred to the subsequent milestone.
- **D-22:** The default non-goals are accepted: no new web embodiment work, no Apple embodiment work, no remote registries, no marketplace, no WASM runtime, and no broad connector expansion beyond Todoist and Google Calendar.

</decisions>

<specifics>
## Specific Ideas

- The core should be built as a lawful substrate: object kernel first, action membrane second, policy before broad mutation.
- Multi-account integration identity, source linkage, cursors, and field ownership have to land in the substrate early or the adapter phases will rot.
- Todoist should shape task semantics honestly instead of pretending the first task model is provider-neutral.
- Calendar recurrence and availability need first-class concepts early or Google Calendar integration will force shadow semantics later.
- Workflow and skill runtime should become backend primitives now so later higher-order task work composes against stable infrastructure instead of reopening the core.

</specifics>

<canonical_refs>
## Canonical References

### Existing repo authority
- `docs/MASTER_PLAN.md`
- `README.md`
- `.planning/PROJECT.md`
- `.planning/ROADMAP.md`
- `.planning/codebase/ARCHITECTURE.md`
- `.planning/codebase/STACK.md`

### Imported `0.5` reference packs
- `.planning/milestones/v0.5-core-rewrite/REFERENCE_PACKS.md`
- `.planning/milestones/v0.5-core-rewrite/reference-packs/vel-core-arch-pack/README.md`
- `.planning/milestones/v0.5-core-rewrite/reference-packs/vel-core-arch-pack/docs/10-development-phases.md`
- `.planning/milestones/v0.5-core-rewrite/reference-packs/vel-skill-workflow-pack/README.md`
- `.planning/milestones/v0.5-core-rewrite/reference-packs/vel-skill-workflow-pack/docs/11-mvp-and-phased-roadmap.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable strengths to preserve
- The repo already has durable layering rules around `vel-core`, `vel-storage`, `vel-api-types`, and `veld`.
- Auth-by-default routes, typed DTO seams, config/schema discipline, and contract docs are already stronger than most greenfield rewrites.
- Existing run-tracing, audit-oriented design, and execution observability expectations can be reused for actions, modules, skills, and workflows.
- Existing connector/config vocabulary in `config/README.md` and the architecture docs should be reused instead of reinvented under provider-specific names.

### Existing drift to avoid normalizing
- Current backend behavior still mixes historical milestone assumptions that predate the new canonical object-centered direction.
- Some integration and product seams remain shaped around older daily-loop and UI-first architecture rather than a true canonical object substrate.
- The current codebase is not yet organized around module loading, workflow primitives, or multi-account integration identity as first-class infrastructure.

### Critical rewrite seam
- `0.5` should preserve repo discipline and durable contracts where still valid, but it should not preserve old backend shapes just to avoid churn.
- The cutover should retire superseded backend paths rather than leaving a shadow second authority lane.

</code_context>

<deferred>
## Deferred Ideas

- Web/client embodiment against the new core
- Apple/client parity against the new backend contracts
- Trigger-heavy workflow automation and scheduler expansion
- Visual workflow authoring
- Remote registries or package marketplace surfaces
- WASM runtime and broader untrusted execution substrate
- Additional providers beyond Todoist and Google Calendar

</deferred>

---

*Context gathered: 2026-03-22 from operator interview and `v0.5` reference-pack import*

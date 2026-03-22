# Phase 60: Module Loader, Registry, And Core-Module Bootstrap - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 60 is the governed module-loading phase for milestone `0.5`.

Its job is to take the Phase 57 registry/workflow contracts, the Phase 58 substrate, and the Phase 59 lawful membrane, and turn them into one deterministic registry/loader path that can bootstrap Vel's own core modules first and then load Todoist and Google Calendar modules through the same governed seam.

Phase 60 is authored against the authoritative intended outputs of Phases 57, 58, and 59 planning packets, even where those artifacts are not yet materialized in implementation code or final filesystem locations.

Phase 60 defines governed registration, reconciliation, seeding, and activation boundaries for canonical registry objects and seeded workflows. It does not define workflow runtime execution, provider adapter behavior, or membrane execution semantics beyond activation-time policy mediation.

This phase should not widen into:

- workflow runtime execution beyond seeded workflow materialization/bootstrap posture
- provider adapter sync behavior
- marketplace or remote registry design
- arbitrary third-party plugin execution substrate
- UI/API transport work
- storage redesign already owned by Phase 58

</domain>

<decisions>
## Locked Dependencies

- **D-01:** Phase 57 owns registry taxonomy, canonical IDs, seeded-workflow posture, and manifest-backed registry semantics.
- **D-02:** Phase 58 owns canonical persistence, bootstrap substrate, and migration/scaffolding seams.
- **D-03:** Phase 59 owns lawful membrane, policy, grants, and audit authority.
- **D-04:** `Module`, `Skill`, and `Tool` remain canonical registry objects rather than manifest ghosts or ordinary content objects.
- **D-05:** Workflows remain canonical content objects stored in canonical storage, including seeded built-ins.
- **D-06:** Core modules and integration modules must load through the same registry/loader path; trust or privilege differences are expressed through policy and provenance, not an entirely different architecture.
- **D-07:** Registry IDs remain stable human-readable semantic identifiers such as `module.integration.todoist` and `module.integration.google-calendar`.
- **D-08:** Deterministic, idempotent bootstrap remains mandatory for seeded modules, skills, tools, and workflows.

### Phase 60-specific posture

- **D-09:** This phase must implement loader and registry behavior, not merely restate manifest existence.
- **D-10:** Module activation and capability requests must remain mediated by the Phase 59 membrane and policy layers.
- **D-11:** Feature gating must allow core registry behavior to compile without provider modules enabled.
- **D-12:** The registry must support compiled-in module sources first, while preserving a clean abstraction for future filesystem/package module sources.
- **D-13:** Seeded updates and local overrides must reconcile deterministically rather than silently overwriting local state.
- **D-14:** Activation must remain about governed eligibility and enablement, not runtime invocation or execution semantics.
- **D-15:** Phase 60 should distinguish `registered`, `reconciled`, `seeded`, `eligible`, `activated`, and `invokable` as separate states or postures rather than blurring them together.
- **D-16:** Reconciliation should use explicit named states rather than implicit drift folklore.

</decisions>

<specifics>
## Specific Ideas

- The main risk in Phase 60 is building a registry that is technically shared but behaviorally split between core modules and provider modules.
- The second main risk is treating manifest loading as direct execution rather than as governed materialization into canonical registry state.
- If bootstrap and reconciliation are not explicit here, later phases will quietly invent side paths for core updates, local overrides, and provider enablement.
- This phase should leave behind one narrow, testable loader path that later workflow and adapter phases can consume without reinterpreting registry identity or capability requests.
- Provider modules should remain boringly equal to core modules at the loader boundary: same loader contract, same registry seam, same reconciliation model, same policy-mediated activation path.

</specifics>

<canonical_refs>
## Canonical References

- `.planning/milestones/v0.5-core-rewrite/57-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/57-03-PLAN.md`
- `.planning/milestones/v0.5-core-rewrite/58-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/58-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/59-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/59-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/60-ACTIVATION-STATE-MATRIX.md`
- `.planning/milestones/v0.5-core-rewrite/60-RISK-SPIKES.md`
- `.planning/milestones/v0.5-core-rewrite/ROADMAP.md`
- `docs/cognitive-agent-architecture/architecture/0.5-module-skill-tool-and-workflow-registry.md`
- `docs/cognitive-agent-architecture/architecture/0.5-workflow-runtime-primitives.md`
- `docs/cognitive-agent-architecture/architecture/0.5-rust-backend-implementation-constraints.md`
- `docs/cognitive-agent-architecture/architecture/0.5-required-backend-traits-and-capability-matrix.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable strengths

- The backend packet already names `ManifestSource`, `RegistryLoader`, and `RegistryReconciler` as explicit roles.
- Phase 58 already requires deterministic bootstrap scaffolding and canonical registry persistence seams.
- Phase 59 already requires capability and policy mediation that module loading should reuse rather than bypass.

### Main dangers

- split bootstrap paths for core and integration modules
- registry identity drifting from canonical module IDs into file-path or crate-name semantics
- module activation bypassing policy/capability checks because core modules are treated as ambiently trusted
- widening the loader into a marketplace or general plugin-execution substrate before the canonical core is proven

</code_context>

<deferred>
## Deferred Ideas

- filesystem/package module distribution as a first-class product surface
- remote registries and marketplace
- executable hooks
- arbitrary third-party module execution
- full workflow runtime behavior beyond bootstrap/materialization
- provider adapter sync and outward mutation behavior
- runtime invocation of workflows, skills, or tools
- registry-bypass hardcoded activation paths for integrations

</deferred>

---

*Context gathered: 2026-03-22 for Phase 60 planning*

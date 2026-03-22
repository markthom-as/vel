# Phase 61: Workflow And Skill Primitives Over Canonical Objects - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 61 is the minimal governed workflow-runtime phase for milestone `0.5`.

Its job is to take the canonical workflow and registry contracts from Phase 57, the substrate from Phase 58, the lawful membrane from Phase 59, and the governed module/bootstrap path from Phase 60, and turn them into a minimal backend runtime for skills and workflows over canonical objects.

Phase 61 is authored against the authoritative intended outputs of Phases 57 through 60 planning packets, even where those artifacts are not yet materialized in implementation code or final filesystem locations.

Phase 61 defines typed context binding, action and skill composition, run records, approval seams, manual invocation, and mediated capability use for workflow execution. It does not define trigger sprawl, workflow authoring UX, broad branching/loop semantics, or provider-specific adapter execution behavior.

Phase 61 defines the manual invocation workflow runtime and its mediated execution contract. It does not define scheduler, trigger, event-listener, background automation, or autonomous invocation behavior.

This phase should not widen into:

- visual or user-facing workflow authoring
- scheduler- or trigger-heavy automation
- adapter-specific business logic
- direct raw-tool invocation by skills
- broad hook execution
- calendar/task domain expansion already owned by later phases

</domain>

<decisions>
## Locked Dependencies

- **D-01:** Phase 57 owns workflow object model, registry entity model, step taxonomy baseline, grant-envelope posture, and the rule that skills do not call raw tools directly.
- **D-02:** Phase 58 owns canonical storage and runtime-record persistence substrate.
- **D-03:** Phase 59 owns action contracts, policy evaluation, grants, ownership/conflict semantics, audit, explainability, and `WriteIntent` dispatch posture.
- **D-04:** Phase 60 owns registration, reconciliation, seeding, and activation of modules, skills, tools, and seeded workflows.
- **D-05:** Workflows remain canonical content objects; `Skill` and `Tool` remain canonical registry objects.
- **D-06:** Workflow runtime must operate through the Phase 59 membrane rather than creating a second execution law.
- **D-07:** Manual invocation is the required proving posture for `0.5`; triggers and broad automation remain deferred.
- **D-07a:** Activation from Phase 60 does not imply invokability or execution; Phase 61 is the first phase that defines lawful manual invocation runtime behavior.

### Phase 61-specific posture

- **D-08:** Workflow steps should stay minimal: `action`, `skill`, `approval`, `sync`, and optional `condition` only where needed.
- **D-09:** Run records remain runtime/control records, not canonical content objects.
- **D-10:** Workflow grant envelopes must narrow authority rather than widening it.
- **D-11:** Typed context binding must be explicit enough to support arbitrary canonical objects structurally, even if only a subset is exercised first.
- **D-12:** Approval and dry-run paths must remain first-class runtime behavior, not bolt-on exceptions.
- **D-13:** Skills may mediate capability use, but they still execute through action/runtime mediation rather than raw tool reach-through.
- **D-14:** Run records, audit entries, approval records, and `WriteIntent` records remain distinct runtime/control concepts with distinct purposes.
- **D-15:** Dry-run must never perform irreversible external mutation and must not mutate canonical content state; only runtime/control evidence may change.
- **D-16:** Refusal and denial must propagate as explicit runtime states rather than collapsing into generic failure noise.

</decisions>

<specifics>
## Specific Ideas

- The main risk in Phase 61 is turning workflows into a second membrane with custom action semantics.
- The second risk is overbuilding orchestration before the proving integrations exist.
- This phase should prove that workflows can bind typed canonical context, compose lawful actions/skills, produce run records, and pause for approval without requiring UI surfaces or trigger infrastructure.
- This phase should prove that refusal, denial, and approval-needed states remain first-class runtime outcomes rather than side-channel errors.
- The runtime should be small enough that later calendar and adapter phases can exercise it honestly rather than inheriting a speculative automation engine.

</specifics>

<canonical_refs>
## Canonical References

- `.planning/milestones/v0.5-core-rewrite/57-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/57-03-PLAN.md`
- `.planning/milestones/v0.5-core-rewrite/58-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/59-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/59-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/60-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/60-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/61-GRANT-ENVELOPE-MATRIX.md`
- `.planning/milestones/v0.5-core-rewrite/61-RUNTIME-STATE-MACHINE.md`
- `.planning/milestones/v0.5-core-rewrite/ROADMAP.md`
- `docs/cognitive-agent-architecture/architecture/0.5-module-skill-tool-and-workflow-registry.md`
- `docs/cognitive-agent-architecture/architecture/0.5-workflow-runtime-primitives.md`
- `docs/cognitive-agent-architecture/architecture/0.5-action-membrane-and-policy.md`
- `docs/cognitive-agent-architecture/architecture/0.5-ownership-conflict-and-write-intent.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable strengths

- The canonical workflow/runtime contract already distinguishes workflow content objects from runtime records.
- The membrane packet already provides explicit action, policy, grant, audit, and refusal-path law that the runtime should reuse.
- The module packet already gives skills/tools/workflows canonical identities and governed activation posture.

### Main dangers

- embedding execution semantics directly into workflow definitions without typed runtime boundaries
- letting skills bypass the membrane because they are "internal"
- widening into trigger/scheduler architecture before manual invocation is proven
- treating run records as canonical content instead of runtime/control evidence

</code_context>

<deferred>
## Deferred Ideas

- trigger frameworks and scheduler orchestration
- visual workflow authoring
- rich looping and general branching systems
- executable hooks
- direct raw-tool execution by skills
- provider-specific workflow behaviors
- implicit retries with side-effecting re-entry
- autonomous or background invocation semantics

</deferred>

---

*Context gathered: 2026-03-22 for Phase 61 planning*

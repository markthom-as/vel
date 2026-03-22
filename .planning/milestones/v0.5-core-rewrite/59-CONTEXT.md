# Phase 59: Action Membrane, Policy Engine, And Audit Authority - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 59 is the lawful membrane phase for milestone `0.5`.

Its job is to turn the Phase 57 membrane/policy contracts and the Phase 58 substrate into an executable backend action layer where reads, mutations, ownership evaluation, grants, confirmation, audit, and `WriteIntent` dispatch all move through one typed, explainable path.

Phase 59 is authored against the authoritative intended outputs of Phase 57 and Phase 58 planning packets, even where those artifacts are not yet materialized in implementation code or final filesystem locations.

This phase should not widen into:

- provider adapter write logic in full
- UI/API transport layer
- workflow authoring UX
- storage/bootstrap redesign already owned by Phase 58
- generalized cross-provider merge behavior beyond membrane ownership/conflict contracts

</domain>

<decisions>
## Locked Dependencies

- **D-01:** Phase 57 owns ontology and contract law.
- **D-02:** Phase 58 owns substrate and persistence.
- **D-03:** Phase 59 owns lawful action/governance behavior over that substrate.
- **D-04:** Generic object actions remain the backbone; specialized actions are layered aliases, not sovereign action kingdoms.
- **D-05:** `WriteIntent` remains a provider-agnostic runtime record.
- **D-06:** Ownership evaluation is dynamic at runtime over static defaults plus overlays.
- **D-07:** Stale state and conflict state are distinct and must remain distinct in the error model.
- **D-08:** Hostile-path verification is mandatory; polite-path success alone is not sufficient proof of a lawful membrane.

### Phase 59-specific posture
- **D-09:** Grants must be modeled explicitly rather than left as vague implied permission state.
- **D-10:** Policy precedence must be explicit across workspace, module, integration account, object, action, and execution context where applicable.
- **D-11:** Audit and explainability are first-class consequences of the membrane, not optional diagnostics.
- **D-12:** `WriteIntent` execution dispatch belongs here as a contract and runtime path, but full provider adapter behavior remains downstream.

</decisions>

<specifics>
## Specific Ideas

- The membrane is where good ontology becomes governable behavior.
- The core risk in this phase is not under-modeling happy paths; it is under-modeling refusal, stale state, and policy friction.
- If grants are not explicit here, they will become ambient power by accident.
- If the error surface is not canonical here, every later phase will smuggle in bespoke failure semantics.

</specifics>

<canonical_refs>
## Canonical References

- `.planning/milestones/v0.5-core-rewrite/59-CONTINUITY-SHEET.md`
- `.planning/milestones/v0.5-core-rewrite/PHASE-59-INTERVIEW-NOTES.md`
- `.planning/milestones/v0.5-core-rewrite/59-MEMBRANE-ERROR-MATRIX.md`
- `.planning/milestones/v0.5-core-rewrite/59-RISK-SPIKES.md`
- `.planning/milestones/v0.5-core-rewrite/57-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/58-CONTEXT.md`
- `.planning/milestones/v0.5-core-rewrite/57-DEPENDENCY-AND-INVARIANTS.md`
- `.planning/milestones/v0.5-core-rewrite/57-RUST-BACKEND-CONTRACT-MATRIX.md`
- `.planning/milestones/v0.5-core-rewrite/57-TEST-PROVING-LADDER.md`
- `docs/cognitive-agent-architecture/architecture/0.5-action-membrane-and-policy.md`
- `docs/cognitive-agent-architecture/architecture/0.5-ownership-conflict-and-write-intent.md`
- `docs/cognitive-agent-architecture/architecture/0.5-rust-backend-implementation-constraints.md`
- `docs/cognitive-agent-architecture/architecture/0.5-required-backend-traits-and-capability-matrix.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable strengths
- The repo already has audit/tracing expectations that can anchor membrane observability.
- The Phase 58 substrate packet already keeps runtime, projections, and canonical storage separate.
- Existing route/service layering discipline fits a typed action registry well if preserved.

### Main dangers
- smuggling provider semantics into the membrane too early
- letting grants remain implicit
- merging stale-state and ownership-conflict behavior into one mushy error
- auditing only successful actions and not denied or dry-run paths

</code_context>

<deferred>
## Deferred Ideas

- full provider outward mutation semantics
- adapter-specific retry logic
- transport DTO/API layer redesign
- workflow authoring surfaces
- generalized cross-provider merge engine

</deferred>

---

*Context gathered: 2026-03-22 for Phase 59 planning*

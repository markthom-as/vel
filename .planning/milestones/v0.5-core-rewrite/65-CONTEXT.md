# Phase 65: Hard Cutover, Backend Contract Reconciliation, And Milestone Verification - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 65 is the cutover and closure phase for milestone `0.5`.

Its job is to make the new canonical core the only live backend authority, retire or isolate superseded backend seams, reconcile remaining callers against the new contracts, and close the milestone with execution-backed verification.

Phase 65 is authored against the authoritative intended outputs of Phases 57 through 64 planning packets, even where those artifacts are not yet materialized in implementation code or final filesystem locations.

Phase 65 defines cutover sequencing, compatibility DTO retirement, caller reconciliation, superseded-path isolation/removal, milestone-level verification, and explicit deferred-work recording. It does not define new ontology, new adapter semantics, new workflow runtime behavior, or new UI/client work.

This phase should not widen into:

- reopening canonical object, membrane, registry, workflow, task, or calendar ontology
- adding new integration scope
- UI/client rebuild work
- speculative post-`0.5` platform design
- stealth redesign under the name of cleanup

</domain>

<decisions>
## Locked Dependencies

- **D-01:** Phase 57 owns the canonical law and anti-drift invariants.
- **D-02:** Phase 58 owns the system-of-record substrate and migration/bootstrap scaffolding.
- **D-03:** Phase 59 owns the lawful membrane, policy, grants, audit, explainability, and `WriteIntent` semantics.
- **D-04:** Phase 60 owns the governed loader/registry/bootstrap path.
- **D-05:** Phase 61 owns the minimal manual workflow runtime.
- **D-06:** Phase 62 owns the native calendar core and governed availability read-model.
- **D-07:** Phase 63 proves Todoist as the task-side constitutional adapter.
- **D-08:** Phase 64 proves Google Calendar as the calendar-side constitutional adapter.

### Phase 65-specific posture

- **D-09:** This phase is about cutover and verification, not architectural renegotiation.
- **D-10:** Compatibility DTO layers are allowed only as temporary scaffolding and should now be removed or explicitly isolated with removal criteria met.
- **D-11:** Superseded backend paths should be retired or isolated; leaving dual live paths counts as incomplete cutover.
- **D-12:** Verification must be execution-backed across the whole milestone surface: substrate, membrane, registry, workflow runtime, Todoist, and Google Calendar.
- **D-13:** Deferred work must be written down explicitly rather than silently surviving inside “temporary” compatibility seams.
- **D-14:** Hard cutover means the new canonical system becomes the live backend authority, not a shadow path beside legacy behavior.
- **D-15:** Phase 65 milestone proof must enforce the locked [0.5-CLARIFICATION-ADDENDUM.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/0.5-CLARIFICATION-ADDENDUM.md) and [0.5-FIELD-OWNERSHIP-MATRIX.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/0.5-FIELD-OWNERSHIP-MATRIX.md) artifacts rather than treating them as advisory notes.
- **D-16:** Phase 65 fails if any legacy path still handles writes, even if some read compatibility remains temporarily quarantined.
- **D-17:** Human-readable milestone closeout evidence must be recorded in `.planning/milestones/v0.5-core-rewrite/65-MILESTONE-EVIDENCE.md`.

</decisions>

<specifics>
## Specific Ideas

- The main risk in Phase 65 is calling the milestone complete while legacy seams still own real behavior.
- The second main risk is treating verification as a documentation ritual instead of an execution-backed proof chain.
- This phase should make the new core unmistakably authoritative and make any remaining deferred work explicit, bounded, and post-`0.5`.
- Cleanup here should be constitutional cleanup, not a disguised excuse to reopen settled architecture.

</specifics>

<canonical_refs>
## Canonical References

- `.planning/milestones/v0.5-core-rewrite/57-DEPENDENCY-AND-INVARIANTS.md`
- `.planning/milestones/v0.5-core-rewrite/57-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/58-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/59-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/60-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/61-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/62-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/63-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/64-VERIFICATION.md`
- `.planning/milestones/v0.5-core-rewrite/0.5-CLARIFICATION-ADDENDUM.md`
- `.planning/milestones/v0.5-core-rewrite/0.5-FIELD-OWNERSHIP-MATRIX.md`
- `.planning/milestones/v0.5-core-rewrite/ROADMAP.md`
- `docs/MASTER_PLAN.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable strengths

- The planning packet already has per-phase validation and verification targets that can be composed into milestone proof.
- The compatibility-DTO and migration-artifact posture was intentionally defined early, so cutover should now be able to close those loops instead of improvising them.
- Adapter and runtime phases were shaped to be constitutional, which makes them cutover-friendly if the repo actually honors those seams.

### Main dangers

- leaving old routes/services active “for safety” and never actually cutting over
- reconciling callers partially, then declaring victory
- allowing deferred work to hide inside compatibility code
- verifying slices independently but never proving the whole integrated backend path end-to-end

</code_context>

<deferred>
## Deferred Ideas

- UI/client rebuild against `0.5` backend contracts
- scheduler/trigger automation
- broader connector expansion
- remote registries/marketplace
- post-`0.5` product/platform widening

</deferred>

---

*Context gathered: 2026-03-22 for Phase 65 planning*

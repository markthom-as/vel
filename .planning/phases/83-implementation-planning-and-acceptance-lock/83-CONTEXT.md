# Phase 83: Implementation planning and acceptance lock - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Convert the approved design packet into an execution-ready milestone with proof expectations. This phase should not reopen design law; it should define the next implementation sequence, acceptance criteria, and retirement plan.

</domain>

<decisions>
## Implementation Decisions

### Implementation sequence
- **D-01:** Shared primitive rebuild should happen first, even if it delays visible page progress.
- **D-02:** Shell/surface primitives should be removed aggressively rather than preserved for compatibility.
- **D-03:** Lower-level components may be wrapped temporarily, but only with an explicit removal path.

### Proof and validation
- **D-04:** Browser proofs and screenshot/fixture artifacts are required in the follow-on implementation line.
- **D-05:** The implementation line should not re-litigate the design packet.
- **D-06:** Interactive prototype outputs from Phase 82 should be treated as execution guidance, not optional inspiration.

### Anti-drift posture
- **D-07:** The follow-on line must explicitly defend against `Now` scope creep, `Threads` absorbing `System`, `System` fragmentation, module shell leakage, and primitive variant drift.

### the agent's Discretion
- Exact shape of the next milestone roadmap
- How to package browser-proof expectations into phase-level acceptance criteria
- Whether to add a component migration matrix as part of the implementation handoff

</decisions>

<specifics>
## Specific Ideas

- The remaining risk is implementation drift, not missing conceptual decisions.
- The implementation milestone should start from removal/rebuild of shared primitives, not cosmetic page reskinning.

</specifics>

<canonical_refs>
## Canonical References

### Active milestone packet
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`

### Milestone lock and upstream specs
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-MILESTONE-LOCK.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/81-COMPONENT-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/80-FOUNDATION-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/79-INTERACTION-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/78-PARADIGM-SPEC.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/core/` — current primitive inventory to map to retire/rebuild decisions
- `clients/web/src/shell/` — current shell surfaces that should not receive backward-compat wrappers

### Established Patterns
- `0.5.2` left the repo with a reusable structure but insufficiently explicit design-system law
- `0.5.3` exists to close that gap before implementation resumes

### Integration Points
- This phase should produce planning docs and milestone handoff artifacts
- The outputs should directly seed the next active implementation milestone

</code_context>

<deferred>
## Deferred Ideas

- actual implementation of the next line
- any backend renegotiation that is not forced by design-contract proof

</deferred>

---

*Phase: 83-implementation-planning-and-acceptance-lock*
*Context gathered: 2026-03-23*

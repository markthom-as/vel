# Phase 82: Page-level UI specs - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Apply the shared paradigm, interaction, foundation, and component law to `Now`, `Threads`, and `System`. This phase must produce concrete page-level UI specs and clickable mockups that validate shell law and disclosure logic.

</domain>

<decisions>
## Implementation Decisions

### Deliverable fidelity
- **D-01:** The primary artifact is HTML/CSS interactive prototypes checked into the repo.
- **D-02:** Static-only mockups are not sufficient as the primary artifact.
- **D-03:** Interaction fidelity matters more than visual polish.
- **D-04:** Desktop-first is acceptable, but mobile coverage must include shell plus one key flow per surface.

### Required prototype coverage
- **D-05:** `Now` normal state must be mocked.
- **D-06:** `Now` degraded state must be mocked.
- **D-07:** `Threads` normal state must be mocked.
- **D-08:** `Threads` focused-block state must be mocked.
- **D-09:** `System` integration-issue state must be mocked.
- **D-10:** `System` control-view state must be mocked.

### Surface boundaries
- **D-11:** `Now` remains bounded and non-inbox-like in every page-level spec.
- **D-12:** `Threads` must not absorb `System` concerns.
- **D-13:** `System` must not fragment into disconnected views.

### the agent's Discretion
- Exact prototype file organization
- Whether the prototypes are static HTML pages with light interactivity or a small routed prototype shell
- How much visual token polish is necessary to validate layout and interaction without over-building

</decisions>

<specifics>
## Specific Ideas

- The point of prototypes is to test shell law and disclosure logic, not to chase pixel-perfect visual polish.
- Browser-proof artifacts should make implementation drift harder later.

</specifics>

<canonical_refs>
## Canonical References

### Active milestone packet
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`

### Governing specs
- `.planning/milestones/v0.5.3-ui-system-design-draft/78-PARADIGM-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/79-INTERACTION-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/80-FOUNDATION-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/81-COMPONENT-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-MILESTONE-LOCK.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/shell/` — shell posture and current chrome structure
- `clients/web/src/views/now/`
- `clients/web/src/views/threads/`
- `clients/web/src/views/system/`

### Established Patterns
- current views already map to the three-surface model
- page specs should preserve this structure while replacing drifted layout and interaction patterns

### Integration Points
- This phase will likely produce design docs and prototype files rather than product implementation code
- The outputs should feed directly into Phase 83 implementation planning

</code_context>

<deferred>
## Deferred Ideas

- near-final visual polish
- production implementation
- Apple-specific mockups

</deferred>

---

*Phase: 82-page-level-ui-specs*
*Context gathered: 2026-03-23*

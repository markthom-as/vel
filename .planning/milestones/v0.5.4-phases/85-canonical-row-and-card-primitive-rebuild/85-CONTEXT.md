# Phase 85: Canonical row and card primitive rebuild - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Replace drifted shared primitives with the approved row-first system. This phase covers canonical row anatomy, bounded card taxonomy, reduced metric-strip emphasis, and explicit wrapper/removal posture for lower-level shared components.

</domain>

<decisions>
## Implementation Decisions

### Primitive posture
- **D-01:** The component system is row-first.
- **D-02:** Cards are reserved for nudges, run/action blocks, media/artifact blocks, and config blocks.
- **D-03:** Drawers are sparing.
- **D-04:** Metric strips should be reduced in prominence.
- **D-05:** One canonical density is used for MVP.

### Canonical row rules
- **D-06:** One base row skeleton exists with bounded surface subclasses.
- **D-07:** Rows may include actions, tags, state, and minimal metadata, but must not become overloaded catch-alls.
- **D-08:** Confidence is not shown as raw row numbers.
- **D-09:** Provenance stays as badge/disclosure, not inline row content.

### Migration posture
- **D-10:** Lower-level shared components may be wrapped temporarily only with explicit removal paths.

### the agent's Discretion
- Exact primitive names and file layout
- How many old primitives can be merged vs replaced outright
- Whether some existing `Panel*` components become internal implementation details during migration

</decisions>

<specifics>
## Specific Ideas

- The main failure mode is allowing rows/cards to fork into inconsistent variants.
- The base row should not become a metadata soup.

</specifics>

<canonical_refs>
## Canonical References

- `.planning/milestones/v0.5.3-phases/83-implementation-planning-and-acceptance-lock/83-IMPLEMENTATION-HANDOFF.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/81-COMPONENT-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-MILESTONE-LOCK.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-NOW-UI-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-THREADS-UI-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-SYSTEM-UI-SPEC.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/core/NowItemRow/`
- `clients/web/src/core/Cards/`
- `clients/web/src/core/PanelChrome/`
- `clients/web/src/core/SurfaceDrawer/`

### Integration Points
- `Now`, `Threads`, and `System` will all consume the primitives from this phase
- later surface phases should not invent new primitive families unless this phase proves a gap

</code_context>

<deferred>
## Deferred Ideas

- surface-specific polish
- browser proofs

</deferred>

---

*Phase: 85-canonical-row-and-card-primitive-rebuild*
*Context gathered: 2026-03-23*

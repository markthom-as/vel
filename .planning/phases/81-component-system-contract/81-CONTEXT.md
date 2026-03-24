# Phase 81: Component-system contract - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Define the reusable primitives, row anatomy, variants, and composition constraints for the client. This phase must turn the design system and interaction doctrine into a composable component contract that later implementation can execute against.

</domain>

<decisions>
## Implementation Decisions

### Primitive posture
- **D-01:** The system is row-first, with selective cards only where richer containers are justified.
- **D-02:** Cards are reserved for nudges, run/action blocks, media/artifact blocks, and config blocks.
- **D-03:** Metric strips should be reduced in prominence.
- **D-04:** Drawers are a sparing primitive, not the default answer to detail.
- **D-05:** One canonical density is used for MVP.

### Canonical row posture
- **D-06:** One base row skeleton should exist with surface-specific subclasses.
- **D-07:** Rows may carry actions, tags, status, and minimal metadata, but should not become overloaded catch-alls.
- **D-08:** Confidence should not appear as a raw number on standard rows.
- **D-09:** Provenance should not appear inline on standard rows.

### Shell and surface primitives
- **D-10:** Shell/surface primitives should be aggressively replaced rather than compatibility-wrapped.
- **D-11:** Lower-level primitives may be wrapped temporarily, but only with an explicit removal path.
- **D-12:** Shell chrome must not be reinvented per surface.

### Control surface posture
- **D-13:** `System > Control` should feel dense but readable.
- **D-14:** Use structured rows with expandable detail rather than playful card grids or raw tables by default.

### the agent's Discretion
- Exact primitive taxonomy naming
- Whether certain existing `core/` items merge or survive as thin wrappers before removal
- The best way to express variant rules without creating an oversized component inventory

</decisions>

<specifics>
## Specific Ideas

- The major component failure mode is letting rows/cards fork into inconsistent variants.
- Modules must not leak into shell chrome.
- The component system should be practical for implementation, not a speculative design-system cathedral.

</specifics>

<canonical_refs>
## Canonical References

### Active milestone packet
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`

### Component and milestone lock docs
- `.planning/milestones/v0.5.3-ui-system-design-draft/81-COMPONENT-SPEC.md` — component families, composition scope, and anti-patterns
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-MILESTONE-LOCK.md` — locked row-first posture, card taxonomy, row anatomy recommendations, and retirement posture
- `.planning/milestones/v0.5.3-ui-system-design-draft/79-INTERACTION-SPEC.md` — interaction constraints the component system must encode
- `.planning/milestones/v0.5.3-ui-system-design-draft/80-FOUNDATION-SPEC.md` — tokens and visual rules the component system must inherit

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/core/` — existing primitive inventory to evaluate for survival, merge, or retirement
- `clients/web/src/core/PanelChrome/`
- `clients/web/src/core/NowItemRow/`
- `clients/web/src/core/Button/`
- `clients/web/src/core/SurfaceDrawer/`

### Established Patterns
- the repo already separates `core/`, `shell/`, and `views/`
- some existing primitives are reusable, but several are too surface-specific to count as a durable shared system

### Integration Points
- This phase should produce a mapping from existing primitives to the intended next system
- The follow-on implementation milestone should rebuild shared primitives before page-by-page surface work

</code_context>

<deferred>
## Deferred Ideas

- exhaustive component gallery implementation
- advanced density preferences beyond the single canonical MVP density
- module-defined shell chrome

</deferred>

---

*Phase: 81-component-system-contract*
*Context gathered: 2026-03-23*

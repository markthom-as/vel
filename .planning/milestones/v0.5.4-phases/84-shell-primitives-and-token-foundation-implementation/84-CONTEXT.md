# Phase 84: Shell primitives and token foundation implementation - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Rebuild the shared shell primitives and token foundation in the real web client. This phase must implement the top band, nudge zone, docked action bar, typography stack, and theme/token baseline from the locked `0.5.3` design packet.

</domain>

<decisions>
## Implementation Decisions

### Shell law
- **D-01:** Top orientation band, nudge zone, and bottom action bar are durable shell regions.
- **D-02:** Shell chrome stays instrument-like and spatially stable across surfaces.
- **D-03:** Mobile action bar is docked, not a floating desktop overlay.
- **D-04:** Shell/surface primitives should not receive backward-compat wrappers.

### Typography and token foundation
- **D-05:** Font stack is `Geist / Inter / JetBrains Mono`.
- **D-06:** `Geist` is for display and major headers only.
- **D-07:** `Inter` is the default UI/body face.
- **D-08:** `JetBrains Mono` is for timestamps, IDs, logs, provenance, and system output.
- **D-09:** Dark-first warmer industrial graphite/copper temperament is the canonical direction.
- **D-10:** Copper is accent only; surfaces remain dark and state colors override brand/provider accent when needed.

### Provider and state handling
- **D-11:** Provider identity is recognizable but subdued.
- **D-12:** Provider tint never overrides state color and never uses full-card brand treatment.

### the agent's Discretion
- Exact token names and CSS variable layout
- Exact implementation path for loading or aliasing the typography stack
- Whether legacy token helpers remain temporarily as compatibility shims below the shell layer

</decisions>

<specifics>
## Specific Ideas

- The shell should feel like an instrument panel, not app chrome.
- The main implementation risk is leaving old shell seams in place and layering the new chrome on top.

</specifics>

<canonical_refs>
## Canonical References

### Handoff and milestone lock
- `.planning/milestones/v0.5.3-phases/83-implementation-planning-and-acceptance-lock/83-IMPLEMENTATION-HANDOFF.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-MILESTONE-LOCK.md`

### Relevant design specs
- `.planning/milestones/v0.5.3-ui-system-design-draft/79-INTERACTION-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/80-FOUNDATION-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/prototypes/README.md`

### Existing code surfaces
- `clients/web/src/shell/`
- `clients/web/src/core/Theme/tokens.ts`
- `clients/web/src/index.css`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- current `shell/` directory already separates `AppShell`, `Navbar`, and `MainPanel`
- current theme tokens exist in `clients/web/src/core/Theme/tokens.ts`
- current font assets already include `Inter`, `Space Grotesk`, `IBM Plex`, and `Outfit`; the new milestone may require changing this stack

### Integration Points
- shell primitives must be applied across all three surfaces
- later phases depend on this phase delivering the shared chrome and token baseline

</code_context>

<deferred>
## Deferred Ideas

- surface-specific implementation
- proof scripts
- cleanup of low-level wrappers beyond what this phase directly touches

</deferred>

---

*Phase: 84-shell-primitives-and-token-foundation-implementation*
*Context gathered: 2026-03-23*

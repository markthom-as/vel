# Phase 80: Foundation design system and theming - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Define the visual system for the web client: typography, iconography, spacing, theme tokens, and color semantics. This phase must turn the locked design direction into named, reusable token decisions.

</domain>

<decisions>
## Implementation Decisions

### Typography
- **D-01:** Font stack is locked to `Geist / Inter / JetBrains Mono`.
- **D-02:** `Geist` is for display and major headers only.
- **D-03:** `Inter` is the default body/UI typeface.
- **D-04:** `JetBrains Mono` is for timestamps, logs, IDs, provenance, and system output.
- **D-05:** Tabular numerals should be enabled globally where supported.
- **D-06:** Sentence case is the default casing rule.

### Theme direction
- **D-07:** Dark-first is canonical for this milestone.
- **D-08:** The visual temperament is warmer industrial: charcoal/graphite with restrained copper accents.
- **D-09:** Copper is accent, not flood fill.
- **D-10:** Surfaces remain dark and should not be broadly tinted.
- **D-11:** State colors override accent when trust/severity demands it.

### Provider and identity color
- **D-12:** Providers should be recognizable but subdued.
- **D-13:** Provider identity uses icon plus subtle tint, not dominant color fields.
- **D-14:** Provider color never overrides state color.
- **D-15:** No full-card provider brand coloring.

### Object and state color posture
- **D-16:** Object colors are selective rather than universal.
- **D-17:** Distinct required state identities include warning, degraded, blocked, active, done, syncing, and offline.
- **D-18:** Thread color should not inherit project color by default.
- **D-19:** Provider/client colors stay separate from project/object colors.

### the agent's Discretion
- Exact token naming scheme
- Exact neutral ladder and accent ramp values, as long as they stay within the locked temperament
- Whether a restrained secondary accent is useful without weakening the primary system

</decisions>

<specifics>
## Specific Ideas

- Typography should feel technical-instrumental with a slight editorial layer.
- The UI should be slightly denser than the current line, but clearer rather than cramped.
- Message/thread surfaces should avoid turning into “confetti” through over-coloring.

</specifics>

<canonical_refs>
## Canonical References

### Active milestone packet
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`

### Foundation design docs
- `.planning/milestones/v0.5.3-ui-system-design-draft/80-FOUNDATION-SPEC.md` — typography, color-system, spacing, and motion scope
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-MILESTONE-LOCK.md` — locked font stack, color temperament, and provider identity constraints
- `.planning/milestones/v0.5.3-ui-system-design-draft/00-SOURCE-INTEGRATION-NOTES.md` — imported source decisions that the foundation should preserve

### Product and paradigm law
- `.planning/milestones/v0.5.3-ui-system-design-draft/78-PARADIGM-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/79-INTERACTION-SPEC.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/core/Theme/tokens.ts` — current token and shell utility baseline
- `clients/web/src/index.css` — existing font and theme definitions
- `clients/web/src/core/Icons/` — current icon infrastructure

### Established Patterns
- the current client already uses a dark orange/copper language, but as class bundles rather than a fully governed token ladder
- typography and icon usage are partially established but not locked as design law

### Integration Points
- This phase should leave named token outputs that later component and page-spec phases can consume directly

</code_context>

<deferred>
## Deferred Ideas

- light mode implementation
- advanced personalization beyond the bounded preferences already in scope
- extra accent families beyond what the core system requires

</deferred>

---

*Phase: 80-foundation-design-system-and-theming*
*Context gathered: 2026-03-23*

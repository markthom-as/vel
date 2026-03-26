# Phase 78: Product paradigms and surface doctrine - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Define the operating model for the web client before visual and implementation decisions spread. This phase locks what `Now`, `Threads`, and `System` are for, what may appear inline, and what must escalate elsewhere.

</domain>

<decisions>
## Implementation Decisions

### Surface doctrine
- **D-01:** `Now` is strictly bounded and must not become inbox-like.
- **D-02:** `Threads` is object/context first and chronology second.
- **D-03:** `System` is hybrid: read-first in `Overview` and `Integrations`, more operational in `Operations` and `Control`.
- **D-04:** `Now`, `Threads`, and `System` remain the only top-level surfaces.

### `Now` bounds
- **D-05:** Allowed first-class `Now` content is limited to active task, one or two next items, current event, next event, nudges, and trust state.
- **D-06:** Forbidden on `Now`: messages, threads, artifacts, runs/logs, people, raw integrations, config, long queues, and project grouping.
- **D-07:** `Now` follows one dominant slot plus one subordinate slot.
- **D-08:** Projects are tag-only on `Now`.

### Escalation doctrine
- **D-09:** Drawer is for shallow inspection and small context.
- **D-10:** Thread is for thinking, resolving, deciding, evidence, ambiguity, and multi-object context.
- **D-11:** `System` owns trust, integration, repair, and structural configuration depth.
- **D-12:** Bounded config work may appear in `Threads`, but browsing config space and schema-level editing stay in `System`.

### Shell posture
- **D-13:** Shell chrome stays instrument-like and spatially consistent across surfaces.
- **D-14:** Top orientation band, nudge zone, and bottom action bar are durable shell regions.
- **D-15:** Projects are stronger contextual identity in `Threads` and first-class in `System`, but should never structure `Now`.

### the agent's Discretion
- Exact wording of short surface doctrine statements
- How best to compress the doctrine into planning-ready acceptance criteria
- Which current UI patterns to list as explicit retirements, as long as they do not contradict the already-locked bans

</decisions>

<specifics>
## Specific Ideas

- The product should feel like an OS, not an app.
- The major failure modes to guard against are:
  - letting `Now` expand beyond bounds
  - letting `Threads` absorb `System` concerns
  - letting `System` fragment into disconnected views
  - letting modules leak into shell chrome
  - letting rows/cards fork into inconsistent variants

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Active milestone packet
- `.planning/ROADMAP.md` — active milestone phases, goals, and success criteria
- `.planning/REQUIREMENTS.md` — active requirement buckets
- `.planning/STATE.md` — active milestone state

### Milestone lock and source integration
- `.planning/milestones/v0.5.3-ui-system-design-draft/00-SOURCE-INTEGRATION-NOTES.md` — imported UI pack decisions and operator chat locks
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-MILESTONE-LOCK.md` — resolved decisions, remaining design locks, banned patterns, and implementation-ready next steps

### Paradigm and surface doctrine
- `.planning/milestones/v0.5.3-ui-system-design-draft/78-PARADIGM-SPEC.md` — phase-specific paradigm doctrine
- `docs/product/now-surface-canonical-contract.md` — durable `Now` contract that must remain bounded and non-dashboard-like
- `.planning/PROJECT.md` — accepted product decisions and current focus

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/shell/` — existing shell split that can map to the instrument-like chrome doctrine
- `clients/web/src/views/now/` — current `Now` composition and existing drift points to constrain
- `clients/web/src/views/threads/` — current continuity surface implementation to compare against the desired object-first posture
- `clients/web/src/views/system/` — current structural surface implementation for `System`

### Established Patterns
- `clients/web/src/core/` already separates reusable primitives from views, but the design system contract is not yet explicit enough
- the `0.5.2` packet already froze the three-surface posture and should be preserved as the embodiment baseline

### Integration Points
- This phase primarily writes doctrine and planning artifacts, not product code
- Later phases will consume these decisions to define interaction, theme, and component rules

</code_context>

<deferred>
## Deferred Ideas

- No new top-level surfaces
- No Apple implementation
- No workflow-builder or planner-studio widening
- No provider expansion

</deferred>

---

*Phase: 78-product-paradigms-and-surface-doctrine*
*Context gathered: 2026-03-23*

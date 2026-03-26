# Phase 79: Interaction system and UX-state law - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Define the UX-state model and interaction rules shared across all surfaces and primitives. This phase locks state handling, action categories, confirmation/optimism rules, disclosure behavior, and shell interaction doctrine.

</domain>

<decisions>
## Implementation Decisions

### Action and feedback law
- **D-01:** Optimistic by default: complete task, dismiss nudge, defer nudge, toggle preference.
- **D-02:** Confirmation required: delete, disconnect, revoke auth, destructive resets, and high-risk external actions.
- **D-03:** Inline feedback plus persistent review path is the trust model.
- **D-04:** Retry/review affordances should be standardized wherever action results matter.
- **D-05:** Critical actions must never be hover-only.

### Disclosure and escalation
- **D-06:** Drawer is for shallow inspection.
- **D-07:** Thread is for discussion, reasoning, evidence, and multi-object ambiguity.
- **D-08:** `System` is the destination for trust, integration, and structural configuration depth.
- **D-09:** Bounded config work may be editable inline in `Threads`; broad config browsing must escalate.

### Shell interaction law
- **D-10:** The shell remains instrument-like across surfaces.
- **D-11:** The nudge zone is always present, but compresses outside `Now`.
- **D-12:** The action bar is always visible except in extreme focus modes where it must be instantly recallable.
- **D-13:** Mobile uses a docked action bar.
- **D-14:** Breadcrumbs appear only when needed in focused subviews.

### Thread-specific interaction
- **D-15:** Threads open into the continuity stream by default.
- **D-16:** Thread ordering is hybrid: recency first, with relevance and pinned context influence.
- **D-17:** Provenance is collapsed by default.
- **D-18:** Filters are sticky per thread.

### the agent's Discretion
- Exact UX-state vocabulary labels beyond the already-set categories
- Which interactions belong in a shared review panel versus inline detail, as long as the locked escalation rules hold
- Exact mapping of state-to-feedback components

</decisions>

<specifics>
## Specific Ideas

- Interaction fidelity matters more than visual polish in the mockup phase.
- The user prefers muscle memory and reachability over clever collapsing behavior.
- The system should feel fast, legible, and dependable rather than animated or “smart.”

</specifics>

<canonical_refs>
## Canonical References

### Active milestone packet
- `.planning/ROADMAP.md` — phase goal and success criteria
- `.planning/REQUIREMENTS.md` — active requirement buckets
- `.planning/STATE.md` — active milestone state

### Interaction and milestone law
- `.planning/milestones/v0.5.3-ui-system-design-draft/79-INTERACTION-SPEC.md` — phase-specific interaction doctrine
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-MILESTONE-LOCK.md` — locked action, disclosure, hover, and shell behavior rules
- `.planning/milestones/v0.5.3-ui-system-design-draft/00-SOURCE-INTEGRATION-NOTES.md` — imported action grammar and shell model from the Downloads packs

### Upstream product doctrine
- `.planning/milestones/v0.5.3-ui-system-design-draft/78-PARADIGM-SPEC.md` — surface boundaries that interaction law must respect
- `docs/product/now-surface-canonical-contract.md` — `Now` escalation and boundedness constraints

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/core/SurfaceDrawer/` — existing drawer primitive to evaluate against the locked sparse-drawer posture
- `clients/web/src/core/Button/` — existing action affordance base
- `clients/web/src/core/MessageComposer/` — current input/action entry seam
- `clients/web/src/views/now/` and `clients/web/src/views/threads/` — existing interaction patterns to retire or preserve

### Established Patterns
- the current client already has reusable interaction primitives, but they are not governed by one explicit doctrine
- `0.5.2` proved the shell and disclosure substrate, which should be reused rather than reinvented

### Integration Points
- This phase mainly produces docs and planning artifacts
- Later component and page-spec phases will consume these decisions directly

</code_context>

<deferred>
## Deferred Ideas

- precise animation durations
- exact drawer motion curves
- edge-case gesture behavior

</deferred>

---

*Phase: 79-interaction-system-and-ux-state-law*
*Context gathered: 2026-03-23*

# Phase 86: `Now` implementation against page spec - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Implement `Now` against the approved bounded page spec using the rebuilt shell and primitives. This phase must keep `Now` strictly bounded and prevent inbox or dashboard drift.

</domain>

<decisions>
## Implementation Decisions

### `Now` bounds
- **D-01:** `Now` remains strictly bounded and non-inbox-like.
- **D-02:** Allowed `Now` objects are active task, one or two next items, current event, next event, nudges, and trust state.
- **D-03:** Projects are metadata only on `Now`.
- **D-04:** Nudges live in a dedicated lane.
- **D-05:** Trust lane appears only when degraded or critical.
- **D-06:** Completed items disappear immediately except for optional transient acknowledgement.

### Disclosure map
- **D-07:** Task actions and simple nudge actions remain inline.
- **D-08:** Event/trust shallow inspection may use drawer/detail treatment.
- **D-09:** Discussion, reasoning, evidence, and ambiguity escalate to `Threads`.
- **D-10:** Integration/trust/config depth escalates to `System`.

### the agent's Discretion
- Exact task/event/trust layout within the page so long as bounds are preserved
- Whether a minimal transition acknowledgement is implemented inline or with a toast-like pattern

</decisions>

<specifics>
## Specific Ideas

- If `Now` scrolls meaningfully or starts aggregating unrelated objects, the phase failed.

</specifics>

<canonical_refs>
## Canonical References

- `.planning/phases/83-implementation-planning-and-acceptance-lock/83-IMPLEMENTATION-HANDOFF.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-NOW-UI-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-MILESTONE-LOCK.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/prototypes/now-normal.html`
- `.planning/milestones/v0.5.3-ui-system-design-draft/prototypes/now-degraded.html`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/views/now/`
- `clients/web/src/views/now/components/`

### Integration Points
- depends on shell and primitive rebuild
- later proof phase must validate normal and degraded `Now`

</code_context>

<deferred>
## Deferred Ideas

- broader inbox/work-queue behavior
- project grouping
- deeper event/system browsing inside `Now`

</deferred>

---

*Phase: 86-now-implementation-against-page-spec*
*Context gathered: 2026-03-23*

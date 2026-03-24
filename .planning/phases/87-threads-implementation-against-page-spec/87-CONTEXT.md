# Phase 87: `Threads` implementation against page spec - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Implement `Threads` against the approved object/context-first page spec using the rebuilt shell and primitives.

</domain>

<decisions>
## Implementation Decisions

### Threads posture
- **D-01:** Object/context leads and chronology remains secondary.
- **D-02:** Default open state is the continuity stream.
- **D-03:** Filters are sticky per thread.
- **D-04:** Provenance is collapsed by default.
- **D-05:** Run/action blocks are visually distinct from messages.
- **D-06:** Bounded config editing is allowed inline.

### Disclosure map
- **D-07:** Messages, object cards, nudges, run summaries, and bounded config blocks may expand inline.
- **D-08:** Media, artifacts, logs, runs, and richer config detail use focus/detail modes.
- **D-09:** Shared review/detail surfaces handle provenance, action traces, run results, and logs.

### the agent's Discretion
- Exact division of inline expansion versus shared review surfaces where the spec allows bounded choice

</decisions>

<specifics>
## Specific Ideas

- The failure mode is letting `Threads` feel like detached chat or letting it absorb `System` browsing concerns.

</specifics>

<canonical_refs>
## Canonical References

- `.planning/phases/83-implementation-planning-and-acceptance-lock/83-IMPLEMENTATION-HANDOFF.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-THREADS-UI-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-MILESTONE-LOCK.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/prototypes/thread-normal.html`
- `.planning/milestones/v0.5.3-ui-system-design-draft/prototypes/thread-focused.html`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/views/threads/`
- `clients/web/src/views/threads/ProvenanceDrawer/`
- `clients/web/src/core/MessageRenderer/`

### Integration Points
- depends on shell and primitive rebuild
- later proof phase must validate normal and focused `Threads`

</code_context>

<deferred>
## Deferred Ideas

- chat-first framing
- broad config browsers in thread context

</deferred>

---

*Phase: 87-threads-implementation-against-page-spec*
*Context gathered: 2026-03-23*

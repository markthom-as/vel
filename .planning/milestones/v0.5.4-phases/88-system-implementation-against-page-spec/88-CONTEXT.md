# Phase 88: `System` implementation against page spec - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Implement `System` against the approved hybrid read-first / operational page spec using the rebuilt shell and primitives.

</domain>

<decisions>
## Implementation Decisions

### System posture
- **D-01:** `Overview` and `Integrations` are read-first.
- **D-02:** `Operations` and `Control` are more operational.
- **D-03:** `Integrations` uses browse/detail split.
- **D-04:** `Control` is dense but readable.
- **D-05:** Logs are summary-first with drill-down.
- **D-06:** Provider identity is recognizable but subdued and never outranks state.

### Disclosure map
- **D-07:** Inline expansion covers row expansion, toggles, compact settings, and summary states.
- **D-08:** Detail panes cover integrations, object browsers, mappings, log summaries, and control object detail.
- **D-09:** Large logs and complex structures may route to dedicated detail views.

### the agent's Discretion
- Exact browse/detail layout mechanics within the locked section structure

</decisions>

<specifics>
## Specific Ideas

- The failure mode is either provider-brand takeover or fragmentation into disconnected mini-admin views.

</specifics>

<canonical_refs>
## Canonical References

- `.planning/milestones/v0.5.3-phases/83-implementation-planning-and-acceptance-lock/83-IMPLEMENTATION-HANDOFF.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-SYSTEM-UI-SPEC.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/82-MILESTONE-LOCK.md`
- `.planning/milestones/v0.5.3-ui-system-design-draft/prototypes/system-integrations-issue.html`
- `.planning/milestones/v0.5.3-ui-system-design-draft/prototypes/system-control.html`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/views/system/`

### Integration Points
- depends on shell and primitive rebuild
- later proof phase must validate integration issue and control views

</code_context>

<deferred>
## Deferred Ideas

- Apple implementation
- provider expansion
- raw log dumping as default UI

</deferred>

---

*Phase: 88-system-implementation-against-page-spec*
*Context gathered: 2026-03-23*

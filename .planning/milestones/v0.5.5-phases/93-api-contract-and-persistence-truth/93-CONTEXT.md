# Phase 93: API contract and persistence truth - Context

**Gathered:** 2026-03-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Close the transport and persistence seams that `0.5.4` could not finish truthfully in frontend code.

This phase should only implement backend/API changes that are required to make accepted `Now`, `Threads`, `System`, and composer behavior durable. It should not spend itself on browser-only polish.

</domain>

<decisions>
## Implementation Decisions

### Confirmed contract truth
- **D-01:** Assistant-entry `intent`, typed `attachments`, and durable `follow_up` are already present at the transport boundary and should not be re-invented.
- **D-02:** Thread row metadata now includes `message_count`, `last_message_at`, and optional `project_label`, so that seam should be treated as landed unless a broader row contract is truly required.

### Remaining backend-truth work
- **D-03:** `Now` lane truth still needs widening because the current contract privileges one active item, a generic `pending` list, and recent completion only.
- **D-04:** Persisted `System` mutations should be truthful and typed; the UI should stop implying durable edits where the backend only supports local approximation.
- **D-05:** This phase should preserve thin route handlers and keep transport mapping at the boundary.

### the agent's Discretion
- Whether `Now` lane widening is best expressed as an additive DTO shape or a replacement of the current `active/pending/recent_completed` lane structure.
- Whether the `System` settings mutation should stay on `/api/settings` with typed nested sections or move to narrower write surfaces, as long as the web boundary updates in the same slice.

</decisions>

<specifics>
## Specific Ideas

- Smallest-first is the right approach here: land truthful persisted settings and typed `Now` lane truth before attempting any broad frontend rebinding.
- Frontend tests are regression hints only; acceptance for later phases still depends on browser/manual review.

</specifics>

<canonical_refs>
## Canonical References

### Milestone packet
- `.planning/milestones/v0.5.5-api-functionality-polish/ROADMAP.md`
- `.planning/milestones/v0.5.5-api-functionality-polish/REQUIREMENTS.md`

### Carry-forward planning
- `.planning/milestones/v0.5.4-carry-forward-phases/91-second-ui-acceptance-revision/91-ACCEPTANCE-CHECKLIST.md`
- `.planning/milestones/v0.5.4-carry-forward-phases/92-api-contract-support-for-accepted-web-ui/92-CONTEXT.md`
- `.planning/milestones/v0.5.4-carry-forward-phases/92-api-contract-support-for-accepted-web-ui/92-01-PLAN.md`

### Existing code surfaces
- `crates/veld/src/services/now.rs`
- `crates/veld/src/routes/now.rs`
- `crates/veld/src/routes/chat.rs`
- `crates/veld/src/services/chat/settings.rs`
- `clients/web/src/data/operator.ts`
- `clients/web/src/data/context.ts`
- `clients/web/src/views/now/NowView.tsx`
- `clients/web/src/views/system/SystemView.tsx`

</canonical_refs>

<code_context>
## Existing Code Insights

### Assistant entry and thread rows
- assistant-entry explicit `intent`, typed `attachments`, and `follow_up` are already wired through Rust and web DTOs
- conversation rows now expose `message_count`, `last_message_at`, and optional `project_label`

### `Now` lane truth
- `crates/veld/src/services/now.rs` still builds one `active` task plus `pending`, `recent_completed`, and `overflow_count`
- `clients/web/src/views/now/NowView.tsx` compensates with local section reshaping and drag/drop approximation

### `System` persistence
- `/api/settings` already exists and persists a subset of settings
- `SystemView` still renders many fields as editable or quasi-editable without a fully truthful persisted contract for everything shown

</code_context>

<deferred>
## Deferred Ideas

- shell/browser-level badge and pill fidelity
- final `Now` / `Threads` / `System` visual acceptance tuning
- browser proof and closeout work

</deferred>

---

*Phase: 93-api-contract-and-persistence-truth*
*Context gathered: 2026-03-23*

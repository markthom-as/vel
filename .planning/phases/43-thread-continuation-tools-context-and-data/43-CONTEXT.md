# Phase 43: Thread continuation, tools, context, and data - Context

**Gathered:** 2026-03-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 43 makes `Threads` the bounded continuation substrate for work that is no longer safe to keep inline in the MVP loop.

This phase is responsible for defining and implementing how thread-backed follow-through can carry context, bounded tools, and review/apply continuity without turning `Threads` into a generic chat product or a second inbox.

It does not widen into broad agent-platform expansion, unbounded tool access, or shell-owned routing semantics.

</domain>

<decisions>
## Implementation Decisions

### Thread continuation boundary
- **D-01:** Phase 43 should build on the existing `Threads` route, assistant-entry routing, `reflow_edit`, and intervention seams rather than inventing a second continuation model.
- **D-02:** Threads remain the continuation path only when work becomes genuinely multi-step under the locked MVP rule: at least two of explanation, multiple decisions, or tool/context work are required.
- **D-03:** `Threads` must stay archive/search-first and continuity-oriented, not a second triage queue.

### Context and tool posture
- **D-04:** Thread continuation should surface bounded current context, relevant memory/data, and explicit tool capability posture through Rust-owned metadata or typed messages, not shell-local inference.
- **D-05:** Tool or context unavailability must fail closed and remain inspectable from the thread state instead of silently widening capabilities.
- **D-06:** The first implementation should reuse existing bounded chat/tool seams where possible before adding any new tool families.

### Review/apply and provenance
- **D-07:** Thread-mediated follow-through must preserve provenance, lifecycle stage, and explicit review/apply semantics where proposals or staged actions are involved.
- **D-08:** Reflow, planning-profile, assistant proposal, and execution-review continuity should converge on one inspectable thread metadata posture instead of each shell inventing its own status story.
- **D-09:** Thread detail views should expose what the thread is about, what context it carries, and what review gate still exists.

### Shell parity and non-goals
- **D-10:** Web should render thread continuation as a thin shell over backend-owned metadata, conversation history, and intervention state.
- **D-11:** Phase 43 must not widen into a generic chat-first product model or broad assistant product work outside the bounded continuation lane.
- **D-12:** Phase 43 must not introduce ambient tool permissions, local shell routing policy, or a second planner surface.

### the agent's Discretion
- Exact contract/DTO shape as long as the resulting thread continuation state remains typed, bounded, and explainable.
- Exact backend/web split as long as shells only render returned continuation metadata and do not invent routing or capability posture locally.

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/veld/src/routes/threads.rs` already owns thread list/detail transport and preserves lifecycle metadata for `reflow_edit`, `planning_profile_edit`, `day_plan_apply`, and proposal threads.
- `crates/veld/src/services/chat/messages.rs` already owns assistant-entry routing between `Inbox`, `Threads`, and inline flows.
- `clients/web/src/components/ThreadView.tsx` already embodies the continuity-first shell posture and is the natural place to surface bounded thread continuation metadata.
- Phase 42 now escalates ambiguous reflow into typed thread continuity, providing the immediate upstream integration point.

### Established Patterns
- Thread escalation rules are already locked in durable docs through `ThreadEscalation` in `mvp-loop-contracts.md` and the `Now` / `Inbox` / `Threads` boundary docs.
- Existing proposal-style thread metadata already uses lifecycle stages such as `staged`, `applied`, and `failed`.
- Backend-owned routing hints already exist for project and action items, and should be reused instead of reconstructed in shells.

### Integration Points
- `crates/veld/src/services/chat/messages.rs`
- `crates/veld/src/routes/threads.rs`
- `crates/veld/src/services/chat/tools.rs`
- `crates/vel-api-types/src/lib.rs`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/ThreadView.test.tsx`
- `docs/product/now-inbox-threads-boundaries.md`
- `docs/cognitive-agent-architecture/architecture/mvp-loop-contracts.md`

</code_context>

<specifics>
## Specific Ideas

- Phase 43 should turn the currently separate thread-like behaviors into one bounded continuation story: why the work escalated, what context it carries, and what review gate still applies.
- The most credible first slice is to tighten the contract and transport first, then embody that metadata in the existing web thread surface.
- This phase should leave Phase 44 free to focus on minimal shells rather than still deciding what a thread continuation contains.

</specifics>

<deferred>
## Deferred Ideas

- Broad agent-platform expansion
- New open-ended tool families beyond the bounded MVP continuation lane
- Generic chat product work or assistant personality surfaces
- Replacing `Inbox` triage with thread-first workflows

</deferred>

---

*Phase: 43-thread-continuation-tools-context-and-data*
*Context gathered: 2026-03-20*

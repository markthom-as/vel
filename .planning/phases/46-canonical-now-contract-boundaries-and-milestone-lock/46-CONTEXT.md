# Phase 46: Canonical Now contract, boundaries, and milestone lock - Context

**Gathered:** 2026-03-21
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 46 is the contract-and-boundary lock for milestone `v0.3`.

It is responsible for:

- publishing the durable `Now` surface contract
- tying that contract explicitly to platform-portable Rust core ownership
- locking the product-surface boundaries that downstream implementation must preserve
- narrowing the client-mesh and parity story enough that later phases can implement without reopening product scope

It does not implement the Rust DTO/service seam yet, and it does not do the actual web/Apple embodiment work.

</domain>

<decisions>
## Implementation Decisions

### Durable contract authority
- **D-01:** The checked-in `Now` surface contract is durable product authority for post-`v0.2` `Now` work.
- **D-02:** The checked-in Rust-core `Now` contract is durable architecture authority for where `Now` behavior must live.
- **D-03:** Downstream phases must treat `Now` semantics as platform-portable Rust-owned behavior, not shell-local design discretion.

### Surface center of gravity
- **D-04:** `Inbox` remains the canonical owner of daily tasks and leftover tasks from the previous day.
- **D-05:** `Now` may surface the highest-priority subset of inbox-owned work when it represents current operational pressure.
- **D-06:** Surfacing an item in `Now` does not transfer ownership away from `Inbox`.
- **D-07:** `Threads` remains the canonical continuity, explanation, search, and multi-step follow-through surface.
- **D-08:** `Now` owns compressed execution state and lightweight actions; deep edits and explanation-heavy work still escalate to `Threads`.

### Client mesh visibility
- **D-09:** `Now` shows only compact client-mesh and connectivity state.
- **D-10:** Disconnected or wrong-authority cases render in `Now` as a compact warning with a route to fix, not as inline endpoint diagnostics.
- **D-11:** Detailed endpoint management, linked-node inspection, pairing/unpairing, and transport troubleshooting stay in support/settings surfaces.
- **D-12:** Client-mesh/connectivity state belongs both in the header/status system and as an allowed nudge/action-bar type when the problem becomes urgent.

### Header filter semantics
- **D-13:** The day-one operator-facing header filter buckets are:
  - `needs_input`
  - `new_nudges`
  - `snoozed`
  - `review_apply`
  - `reflow`
  - `follow_up`
- **D-14:** `threads_by_type` is an implementation concept and backing model, not the literal user-facing label.
- **D-15:** Tapping a header bucket opens `Threads` filtered to that shared continuation category; it must not mutate read/open state.

### Watch reduction
- **D-16:** Watch is reduced, not divergent.
- **D-17:** Watch must support:
  - top status row
  - top current nudge/bar
  - current task
  - voice entry
  - reduced thread response / confirmation flow
- **D-18:** Watch must not own:
  - full stacked nudge list
  - broad task history / expanded task list
  - header filter bar
  - deep connection management
  - full thread browsing/search
- **D-19:** When deeper follow-through is required, watch opens a reduced thread response screen that can hand off to phone or Mac.

### the agent's Discretion
- Exact doc wording and breakdown across product vs architecture references, as long as the authority chain stays explicit.
- Exact naming of the Rust-owned DTO/read-model seams introduced in later phases.
- Exact visual/icon language for header buckets and compact connectivity warnings, as long as the routing and ownership rules above are preserved.

</decisions>

<specifics>
## Specific Ideas

- `Now` should stay compact and operational; `Inbox` keeps ownership of the broader daily queue.
- Client-mesh state matters in `Now` only when it affects immediate trust or action.
- Header buckets should feel like real operator slices, not technical taxonomy.
- Watch should be capable of meaningful response, but should hand off early when work stops being lightweight.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone authority
- `.planning/PROJECT.md` — milestone goal, scope, and accepted decisions
- `.planning/REQUIREMENTS.md` — `v0.3` requirements and acceptance checklist
- `.planning/ROADMAP.md` — fixed phase boundary and downstream sequencing
- `.planning/STATE.md` — current milestone state and execution position

### Durable product and architecture authority
- `docs/product/now-surface-canonical-contract.md` — canonical post-`v0.2` `Now` surface behavior
- `docs/cognitive-agent-architecture/architecture/now-surface-rust-core-contract.md` — Rust-core ownership rule for `Now`
- `docs/product/now-inbox-threads-boundaries.md` — current surface boundary guidance
- `docs/product/mvp-operator-loop.md` — inherited loop and escalation rules from `v0.2`
- `docs/cognitive-agent-architecture/architecture/mvp-loop-contracts.md` — prior loop contract language that must now stay aligned with the `Now` contract
- `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md` — cross-surface portability boundary
- `docs/MASTER_PLAN.md` — canonical status tracker and active milestone reference

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `clients/web/src/components/NowView.tsx` already contains the main web `Now` embodiment, but it is still too card-heavy and shell-authored for the new contract.
- `clients/web/src/components/ThreadView.tsx` already supports bounded continuation and filter/search posture, which can back the new header bucket routing.
- `clients/web/src/components/SettingsPage.tsx` already contains endpoint and routing configuration for Tailscale/LAN/localhost, plus support copy around shared daemon usage.
- `clients/web/src/data/operatorSurfaces.ts` already defines the current shell taxonomy and can be tightened around the new `Now` center of gravity.
- `clients/apple/Apps/VeliOS/ContentView.swift`, `clients/apple/Apps/VelMac/ContentView.swift`, and `clients/apple/Apps/VelWatch/ContentView.swift` already contain the current Apple shell splits and reduced watch posture.
- `crates/vel-api-types/src/lib.rs` already carries linked-node, queued-write, sync-status, voice-queue, and `Now`-related DTO surfaces that can be tightened instead of replaced wholesale.
- `crates/veld/src/services/linking.rs`, `crates/veld/src/services/client_sync.rs`, `crates/veld/src/services/tailscale.rs`, and related sync/linking routes already provide a starting substrate for the client-mesh lane.

### Established Patterns
- Phase 40 already locked the principle that `Now` is a decision/execution surface rather than a dashboard.
- Phases 41-45 already pushed overview, reflow, thread continuation, and thin-shell behavior toward Rust-owned contracts.
- The repo already prefers typed transport DTOs and Rust-owned services over shell-local product rules.
- Apple offline stores and queued-action plumbing already exist; the next step is to unify their product meaning across clients rather than invent new local semantics.

### Integration Points
- Phase 46 planning must connect the new `Now` product contract to the existing `Now`, `Threads`, linking, sync, and queued-action seams.
- Later phases should reuse existing thread continuation metadata and linked-node/sync surfaces rather than introducing a second connection model.
- Support/settings surfaces remain the place for deep endpoint management; `Now` only gets compact visibility and urgent warnings.

</code_context>

<deferred>
## Deferred Ideas

- broad review analytics or journaling
- multi-day planning beyond the current-day and day-object model
- broad provider/platform expansion beyond client-mesh needs
- generic chat-product widening
- deep connection management directly inside `Now`

</deferred>

---

*Phase: 46-canonical-now-contract-boundaries-and-milestone-lock*
*Context gathered: 2026-03-21*

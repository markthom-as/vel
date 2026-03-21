# Phase 47: Rust-owned Now core models and transport seam - Context

**Gathered:** 2026-03-21
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 47 is the first implementation phase for milestone `v0.3`.

It is responsible for turning the locked Phase 46 contract into one shared Rust-owned transport seam for `Now`.

That means this phase must define and ship the portable core models that every client will read:

- header buckets and routing posture
- status row inputs and context one-liner
- ranked nudge/action bars
- canonical day and task subset outputs
- thread-backed continuity and escalation markers
- docked-input routing and intent taxonomy

It does not embody the full web or Apple UI yet, and it does not implement the deeper client-mesh, sync, governed-config, or repair lanes assigned to Phase 48.

</domain>

<decisions>
## Implementation Decisions

### Transport seam strategy
- **D-01:** Phase 47 should extend the existing `Now` service and `/v1/now` route rather than creating a parallel endpoint for the canonical surface contract.
- **D-02:** Existing `overview`, `schedule`, `tasks`, and related fields may coexist temporarily, but the new canonical `Now` seam must be explicit enough that later client work can stop depending on shell-authored interpretation.
- **D-03:** Shared transport DTOs must land in `crates/vel-api-types/src/lib.rs` and then flow through web and Apple boundary code in the same slice whenever a contract changes.

### Canonical Now model
- **D-04:** The canonical `Now` transport must add distinct Rust-owned blocks for:
  - header buckets and counts
  - status row inputs
  - context one-liner
  - stacked nudge/action bars
  - canonical task lane
  - docked-input routing semantics
- **D-05:** `Now` remains a compressed execution surface; the transport should carry the ranked subset and escalation markers rather than the full queue or full thread history.
- **D-06:** Header buckets are the shared operator-facing categories locked in Phase 46:
  - `needs_input`
  - `new_nudges`
  - `snoozed`
  - `review_apply`
  - `reflow`
  - `follow_up`
- **D-07:** Bucket identity, count, urgency marker, and filtered-thread routing posture are Rust-owned transport, not shell invention.

### Day, task, and ranking substrate
- **D-08:** `day` is the canonical current-day container feeding `Now`.
- **D-09:** The canonical task seam must stop depending on shell-local distinctions between commitments, tasks, and nearby work.
- **D-10:** The task lane should expose the ranked subset needed by `Now`:
  - current task
  - next active items
  - one recent completed item
  - overflow posture
- **D-11:** Carry-forward into the new day is automatic; day-start review adjusts the carried-forward truth rather than inventing a second carryover mechanism in `Now`.
- **D-12:** Nudge/action-bar ordering uses one shared backend-owned priority ladder across types.

### Thread continuity and docked input
- **D-13:** Docked input always creates thread-backed continuity immediately.
- **D-14:** Phase 47 must formalize the canonical `day thread` and `raw capture` thread lanes as shared transport concepts.
- **D-15:** Raw docked capture does not automatically create an inbox item; inbox ownership appears only when backend routing explicitly promotes the result into inbox-owned work.
- **D-16:** Thread continuity transport must expose:
  - primary/open-target thread identity
  - thread category filters
  - continuation chips or escalation markers for `Now`
  - shared metadata filters such as project and tags
- **D-17:** Phase 47 should define the closed v1 public intent taxonomy for docked input routing and keep future extension posture explicit.

### Non-goals
- **D-18:** Phase 47 must not move governed title/count/watch config, approval policy, sync/offline truth, or repair-route logic into shell-local code.
- **D-19:** Phase 47 must not rebuild the web or Apple `Now` surface layout beyond the minimum boundary updates required to keep typed clients aligned.
- **D-20:** Phase 47 must not widen into broad client-mesh implementation that belongs to Phase 48.

### the agent's Discretion
- Exact DTO and enum names, as long as the contract remains visibly aligned to the Phase 46 authority docs.
- Exact migration order between legacy `Now` fields and the new canonical seam, as long as cross-client transport remains coherent and later phases can converge on the new seam without reopening product behavior.

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/veld/src/services/now.rs` already owns the assembled `Now` read model and is the natural landing zone for canonical header/status/context/nudge/task outputs.
- `crates/veld/src/routes/now.rs` already maps the service output into `vel-api-types::NowData`, so the new seam should extend that conversion rather than bypass it.
- `crates/vel-api-types/src/lib.rs` already contains `NowData`, `ThreadData`, thread continuation DTOs, daily-loop DTOs, and assistant-entry DTOs that Phase 47 can tighten into one portable contract.
- `crates/veld/src/services/chat/thread_continuation.rs`, `crates/veld/src/routes/chat.rs`, and `crates/veld/src/routes/threads.rs` already carry bounded continuation metadata and can be extended for `day thread`, `raw capture`, bucket filters, and open-target semantics.
- `clients/web/src/types.ts` and Apple `VelAPI` models already consume typed transport and can absorb contract additions as thin boundary updates.

### Established Patterns
- Earlier phases already added backend-owned overview, continuity, and reflow status to `/v1/now`; Phase 47 should build on that seam instead of adding a second read model.
- The repo already prefers typed DTOs and Rust-owned services over shell-local synthesis.
- Thread continuation and assistant-entry routing already exist in shared transport, which makes docked-input continuity an extension of the current lane rather than a new subsystem.

### Integration Points
- `crates/veld/src/services/now.rs`
- `crates/veld/src/routes/now.rs`
- `crates/vel-api-types/src/lib.rs`
- `crates/veld/src/services/chat/thread_continuation.rs`
- `crates/veld/src/routes/chat.rs`
- `crates/veld/src/routes/threads.rs`
- `clients/web/src/types.ts`
- `clients/web/src/types.test.ts`
- `clients/apple/VelAPI/Sources/VelAPI/Models.swift`

</code_context>

<specifics>
## Specific Ideas

- The current `Now` seam is still organized around the older `overview / summary / schedule / tasks / attention` grouping. Phase 47 should add a canonical seam that maps to the locked compact `Now` contract without forcing the UI phases to infer missing behavior.
- The first implementation win is transport truth, not final visual embodiment.
- This phase should leave Phase 49 and Phase 50 free to focus on presentation and interaction density instead of still negotiating what a nudge, task, header bucket, or docked capture result means.

</specifics>

<deferred>
## Deferred Ideas

- full web layout rewrite
- full Apple/watch embodiment work
- governed config persistence and mutation workflows
- sync/offline authority summary implementation
- deeper linking and recovery flows

</deferred>

---

*Phase: 47-rust-owned-now-core-models-and-transport-seam*
*Context gathered: 2026-03-21*

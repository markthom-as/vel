# Phase 41: Rust-owned overview, commitment flow, and orientation core - Context

**Gathered:** 2026-03-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 41 implements the first live Rust-owned MVP behavior over the Phase 40 contract packet.

This phase is responsible for making overview, commitment flow, and orientation outputs genuinely backend-owned so web and Apple consume the same current-day truth.

It does not widen into broad thread/tool work, broad reflow implementation beyond the overview seam, or shell-specific decision logic.

</domain>

<decisions>
## Implementation Decisions

### Overview implementation boundary
- **D-01:** Phase 41 should implement the canonical overview through the existing `Now` service/read-model seam rather than inventing a parallel product endpoint.
- **D-02:** The shipped overview must preserve the locked Phase 40 behavior: `action + timeline`, one dominant action, one visible nudge, `Why + state`, and 1-3 suggestions when no dominant action exists.
- **D-03:** The overview output should emphasize what to do next and what needs intervention, not dashboard/status clutter.

### Commitment flow
- **D-04:** Commitment flow should reuse and tighten existing backend daily-loop/session seams where possible instead of creating shell-owned commitment state.
- **D-05:** The inline commitment path must preserve `accept / defer / choose / close` across shells.
- **D-06:** Session continuity and commitment state should be backend-owned and transportable rather than reconstructed separately in web or Apple.

### Orientation and nudges
- **D-07:** Suggestions and nudges must stay grounded in persisted context, schedule state, commitments, and thread/history evidence.
- **D-08:** Only one top nudge should be visible by default; deeper explanation belongs in backend-owned `Why + state` outputs.
- **D-09:** Orientation outputs must remain explainable from Rust-owned rules rather than ranking logic hidden in shells.

### Shell parity and non-goals
- **D-10:** Web and Apple should consume the same overview/commitment/orientation transport semantics as thin shells.
- **D-11:** Phase 41 must not widen into broad thread/tool expansion or calendar reflow implementation beyond what overview and commitment seams need.
- **D-12:** Phase 41 must not add shell-specific fallback decision logic.

### the agent's Discretion
- Exact DTO/service naming as long as it stays aligned to the Phase 40 contract language.
- Exact migration sequence between current `Now`, `daily_loop`, and transport DTO seams as long as backend authority stays central.

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `crates/veld/src/services/now.rs` already owns the current read-model assembly seam and is the natural landing zone for canonical overview output.
- `crates/veld/src/services/daily_loop.rs` already owns session lifecycle and can anchor commitment continuity.
- `crates/vel-api-types/src/lib.rs` already carries `NowData`, daily-loop, reflow, and review-related transport seams that Phase 41 can tighten instead of duplicating.
- Existing `reflow` and thread status fields in `Now` can remain supporting context while overview/commitment authority is normalized.

### Established Patterns
- Backend-owned typed DTOs already exist and should be extended rather than bypassed.
- Phase 40 durable docs now define the product loop, contract language, shell boundaries, and anti-drift rules.
- Current architecture docs already require web and Apple to remain thin shells over Rust-owned semantics.

### Integration Points
- `crates/veld/src/services/now.rs`
- `crates/veld/src/services/daily_loop.rs`
- `crates/veld/src/routes/threads.rs`
- `crates/vel-api-types/src/lib.rs`
- web `clients/web/src/types.ts` and data loaders
- Apple `clients/apple/VelAPI` transport consumers

</code_context>

<specifics>
## Specific Ideas

- Phase 41 should make `Now` feel like the live embodiment of the Phase 40 MVP loop contract, not a legacy dashboard with extra fields.
- The easiest path is to tighten backend seams that already exist instead of introducing a second overview model.
- This phase should leave Phase 42 free to focus on reflow quality rather than still arguing about overview truth.

</specifics>

<deferred>
## Deferred Ideas

- Broad thread/tool capabilities beyond bounded continuation
- Full reflow implementation beyond the overview/commitment seam
- Local-calendar ingestion or apply work
- Fresh shell redesign work beyond the transport parity needed for later phases

</deferred>

---

*Phase: 41-rust-owned-overview-commitment-flow-and-orientation-core*
*Context gathered: 2026-03-20*

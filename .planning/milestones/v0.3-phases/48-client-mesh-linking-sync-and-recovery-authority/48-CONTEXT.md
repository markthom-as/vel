# Phase 48: Client mesh, linking, sync, and recovery authority - Context

**Gathered:** 2026-03-21
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 48 turns the remaining `Now` support lanes into shared Rust-owned authority.

Phase 47 established the canonical `Now` transport and continuity vocabulary. Phase 48 now owns the missing support truth that the compact cross-platform `Now` contract depends on:

- shared client-mesh and authority summary
- sync and offline posture
- queued-write and recovery visibility
- governed config for title, count-display, and fast-evolving reduced-watch behavior
- explicit repair-route targets for disconnected or mismatched clients

This phase must keep that truth in the platform-portable Rust core and shared transport. It must not widen into the final web or Apple `Now` embodiment, and it must not move detailed linking/repair workflows into `Now` itself.

</domain>

<decisions>
## Implementation Decisions

### Mesh and recovery posture
- **D-01:** `Now` shows compact client-mesh state only when it affects immediate trust or action.
- **D-02:** Detailed endpoint forms, linking management, and recovery tooling remain support-surface responsibility rather than `Now` surface responsibility.
- **D-03:** Urgent connectivity problems may still surface as `Now` warning/action bars when they affect current trust or action.
- **D-04:** The shared authority summary must expose:
  - connection state
  - sync posture (`synced`, `stale`, `local_only`, `offline`, or equivalent typed state)
  - last sync
  - queued-write count
  - compact recovery-route target

### Governed config
- **D-05:** Title/display/count/watch behavior belongs to governed config, not shell constants.
- **D-06:** Phase 48 must define versioned Rust-owned config support for:
  - `Now` title/display policy
  - count-display policy
  - watch reduction/display knobs needed for fast iteration
  - approval/config-mutation posture where the boundary is already stable
- **D-07:** Governed config remains explicit and reviewable. Shells may consume it; they do not own its policy semantics.

### Client linking and repair
- **D-08:** Clients need one coherent authority/runtime story across web, Apple, and reduced watch, not platform-specific guesses.
- **D-09:** Repair routes must be typed and compact enough that `Now` can point to them without embedding setup logic locally.
- **D-10:** Linking and recovery guidance should be explainable from typed backend state and route targets, not ad hoc shell copy.

### Non-goals
- **D-11:** Phase 48 does not rebuild the compact `Now` layout; that belongs to Phase 49 and Phase 50.
- **D-12:** Phase 48 does not widen into generic chat expansion, provider expansion, or broad settings redesign.
- **D-13:** Phase 48 does not make watch behavior divergent; it only defines governed support for the reduced contract.

### the agent's Discretion
- Exact enum and DTO naming for sync posture, route targets, and governed config, as long as the resulting transport stays explicit, typed, and reusable across clients.
- Exact split between config schema docs and code-adjacent examples, as long as the governed-config boundary ships with durable authority and parseable artifacts once stable.

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `docs/product/now-surface-canonical-contract.md` and `docs/cognitive-agent-architecture/architecture/now-surface-rust-core-contract.md` already lock the product and architecture expectations for compact mesh visibility and governed config.
- `/home/jove/code/vel/.planning/milestones/v0.3-phases/46-canonical-now-contract-boundaries-and-milestone-lock/46-SUBSYSTEM-INVENTORY.md` already assigns Phase 48 the missing support lanes:
  - title/display policy
  - count-display policy
  - sync/offline header summary
  - approval/config mutation posture
  - offline write queue
  - conflict posture
  - linking and recovery guidance
- Phase 47 already landed the transport seam in:
  - `crates/vel-api-types/src/lib.rs`
  - `crates/veld/src/services/now.rs`
  - `crates/veld/src/routes/now.rs`
  - `crates/veld/src/services/chat/thread_continuation.rs`
  - `crates/veld/src/routes/chat.rs`
  - `crates/veld/src/routes/threads.rs`
- Existing settings and linked-node behavior provide source material for the support lanes, but they must be reconciled into Rust-owned typed authority rather than copied forward as shell-local behavior.

### Established Patterns
- Transport changes must land in Rust DTOs and affected client boundary decoders together.
- Support surfaces may present repair/setup work, but the product authority for state and routing belongs in Rust backend layers.
- The repo prefers narrow route handlers and typed services over endpoint-specific JSON blobs or shell-authored recovery logic.

### Integration Points
- `crates/vel-api-types/src/lib.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/routes/now.rs`
- `crates/veld/src/services/*` support lanes related to sync, linking, and settings authority
- `clients/web/src/types.ts`
- `clients/web/src/types.test.ts`
- `clients/apple/VelAPI/Sources/VelAPI/Models.swift`
- `docs/user/daily-use.md`

</code_context>

<specifics>
## Specific Ideas

- The first win is one shared Rust-owned mesh/repair summary that every client can read the same way.
- Governed config should land as explicit typed support before the web and Apple embodiment phases try to iterate on the compact `Now` surface rapidly.
- `Now` should only get the compact subset needed for immediate trust:
  - sync posture
  - queued-write pressure
  - recovery-route target
  - urgent warning-bar eligibility
- Deeper setup and repair detail should remain support-surface material even after the shared authority summary exists.

</specifics>

<deferred>
## Deferred Ideas

- final compact web `Now` layout
- final Apple and watch embodiment
- broad support-surface redesign beyond what typed authority requires
- speculative provider/platform widening unrelated to authority continuity

</deferred>

---

*Phase: 48-client-mesh-linking-sync-and-recovery-authority*
*Context gathered: 2026-03-21*

# Roadmap: Vel

## Archived Milestones

- `v0.1` archived phase packet: [v0.1-phases](/home/jove/code/vel/.planning/milestones/v0.1-phases)
- `v0.2` shipped true-MVP archive: [v0.2-ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.2-ROADMAP.md)

## Active Milestone

Milestone `v0.3` starts at Phase 46 and turns the canonical `Now` surface plus client mesh into one Rust-owned product-core lane.

The goal of `v0.3` is to:

- publish and enforce one cross-platform `Now` contract
- move `Now` semantics into platform-portable Rust core and shared transport
- converge on one canonical task, thread, day, and nudge model for `Now`
- explicitly assign ranking, intent routing, approval policy, and governed config support lanes before implementation spreads
- help clients connect to and recover connection with the same authority runtime
- ship parity across web, iPhone, iPad, Mac, and reduced watch

## Scope Guardrails

`v0.3` is only about the canonical `Now` surface and client mesh:

- compact execution-first `Now`
- shared Rust-owned product semantics
- client-linking, sync/offline visibility, and connection recovery
- cross-platform parity for the same interaction contract

Do not widen this milestone into:

- generic chat product expansion
- broad provider/platform expansion beyond connection needs
- speculative cloud reset
- broad review analytics
- multi-day planning beyond the day-object and current `Now` needs

## Phases

- [x] **Phase 46: Canonical Now contract, boundaries, and milestone lock** - Publish the durable Now surface contract and align product/architecture boundaries before implementation widens
- [x] **Phase 47: Rust-owned Now core models and transport seam** - Build the shared Rust-owned task, day, thread, nudge, and context contracts that feed Now
- [ ] **Phase 48: Client mesh, linking, sync, and recovery authority** - Make cross-client connection, offline state, queued writes, and recovery visible and shared
- [ ] **Phase 49: Web canonical Now embodiment** - Rebuild the web Now surface to match the canonical execution-first contract over shared Rust-owned seams
- [ ] **Phase 50: Apple parity and reduced watch embodiment** - Align iPhone, iPad, Mac, and reduced watch to the same canonical Now contract and mesh behavior
- [ ] **Phase 51: Cross-platform verification and closeout** - Verify parity, mesh continuity, and Rust-core ownership across all shipped shells

## Progress

**Execution Order:** 46 -> 47 -> 48 -> 49 -> 50 -> 51

| Phase | Requirements | Status |
|-------|--------------|--------|
| 46. Canonical Now contract, boundaries, and milestone lock | NOW-01 | Complete |
| 47. Rust-owned Now core models and transport seam | NOW-02, NOW-04, CORE-01, CORE-02, CORE-03, TASK-01, THREAD-01 | Complete |
| 48. Client mesh, linking, sync, and recovery authority | CORE-04, MESH-01, MESH-02, MESH-04, SYNC-01, SYNC-02 | Active |
| 49. Web canonical Now embodiment | NOW-03, PARITY-01, TASK-02, THREAD-02 | Planned |
| 50. Apple parity and reduced watch embodiment | MESH-03, PARITY-02, PARITY-03, PARITY-04, SYNC-03 | Planned |
| 51. Cross-platform verification and closeout | milestone verification and reconciliation | Planned |

## Phase Details

### Phase 46: Canonical Now contract, boundaries, and milestone lock

**Goal:** Publish the durable Now surface contract, tie it to Rust-core authority, and lock product boundaries before implementation starts.
**Requirements:** NOW-01
**Depends on:** archived milestone v0.2 truth
**Success Criteria:**
1. Durable Now product and Rust-core architecture contracts are checked in and linked from the existing authority docs.
2. Surface-boundary docs now point to the canonical Now contract instead of stale MVP-only shell assumptions.
3. The milestone scope is narrow enough to prevent shell-local drift and broad product widening.
4. The supporting subsystems needed for the canonical `Now` behavior are explicitly inventoried and assigned to downstream phases before implementation starts.
5. The checked-in contract packet is explicitly reconciled against the full local source contract in `/home/jove/Downloads/vel-now-surface-contract-codex-final.md`.
**Plans:** 4 plans

### Phase 47: Rust-owned Now core models and transport seam

**Goal:** Build the portable Rust-owned core contracts and transport seam that define `Now` across clients.
**Requirements:** NOW-02, NOW-04, CORE-01, CORE-02, CORE-03, TASK-01, THREAD-01
**Depends on:** Phase 46
**Success Criteria:**
1. Shared Rust-owned DTOs and service outputs exist for header, status row, one-liner, nudge bars, task list, and docked input routing.
2. Canonical task/day/thread/nudge semantics no longer depend on web or Apple local policy.
3. Thread filter categories, intent taxonomy, and `Now` escalation markers are shared transport, not shell invention.
**Plans:** 4 plans

### Phase 48: Client mesh, linking, sync, and recovery authority

**Goal:** Make client connection, sync state, queued writes, and recovery part of the shared product-core model.
**Requirements:** CORE-04, MESH-01, MESH-02, MESH-04, SYNC-01, SYNC-02
**Depends on:** Phase 47
**Success Criteria:**
1. Clients can understand and present the same authority endpoint, sync state, and queued-write posture.
2. Guided linking and recovery no longer rely on platform-specific guesswork.
3. Governing config for `Now` title/display/count policies, approval posture, and mesh state is versioned and Rust-owned.
**Plans:** 4 plans

### Phase 49: Web canonical Now embodiment

**Goal:** Rebuild the web `Now` surface around the canonical compact contract and thread-backed continuity model.
**Requirements:** NOW-03, PARITY-01, TASK-02, THREAD-02
**Depends on:** Phase 48
**Success Criteria:**
1. Web `Now` matches the canonical execution-first structure instead of the current card-heavy summary layout.
2. Docked input, live transcript bubble, nudge bars, and task list work from shared contracts.
3. Web no longer relies on shell-authored fallback product semantics for `Now`.
**Plans:** 4 plans

### Phase 50: Apple parity and reduced watch embodiment

**Goal:** Align Apple shells to the same `Now` contract and client-mesh behavior, with reduced watch density rather than divergent behavior.
**Requirements:** MESH-03, PARITY-02, PARITY-03, PARITY-04, SYNC-03
**Depends on:** Phase 49
**Success Criteria:**
1. iPhone, iPad, and Mac embody the same `Now` structure and routing semantics as web.
2. Watch ships the reduced but contract-faithful subset.
3. Sync/offline/conflict recovery remains understandable across Apple surfaces.
**Plans:** 4 plans

### Phase 51: Cross-platform verification and closeout

**Goal:** Verify parity, client mesh continuity, and Rust-core ownership across the milestone, then close the milestone honestly.
**Requirements:** milestone verification and reconciliation
**Depends on:** Phase 50
**Success Criteria:**
1. Cross-platform verification proves the same `Now` behavior and thread routing across shipped shells.
2. Mesh/linking and offline behavior are evidenced, not just described.
3. The milestone can close without hidden shell-specific carryover.
**Plans:** 3 plans

---
*Last updated: 2026-03-21 for milestone v0.3 canonical Now surface and client mesh planning*

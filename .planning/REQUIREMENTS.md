# Requirements: Vel

**Defined:** 2026-03-21
**Milestone:** v0.3
**Core Value:** Reliable, local-first capture and recall that a solo operator can trust.

## v0.3 Requirements

Requirements for the cross-platform `Now` rebuild and client-mesh milestone. This milestone turns the checked-in `Now` surface contract into Rust-owned product behavior and ensures clients can connect cleanly to the same authority runtime.

## Milestone Acceptance Checklist

v0.3 is only complete if all of these are true:

- [ ] `Now` behaves as one canonical surface across web, iPhone, iPad, Mac, and reduced watch
- [ ] all `Now` product semantics are backed by platform-portable Rust core and shared transport
- [ ] client shells do not define their own task, nudge, thread, or day behavior for `Now`
- [ ] ranking, intent routing, approval posture, and governed config are owned by shared subsystem contracts instead of shell-local policy
- [ ] operators can understand and recover client connection state across surfaces
- [ ] clients can link to the same authority runtime without platform-specific guesswork
- [ ] offline/sync/queued-write behavior is visible and consistent across clients

## Non-Goals

- broad new product-surface expansion outside `Now`, `Threads`, connection, and sync support
- shell-specific redesign work not required for parity
- speculative cloud-first architecture reset
- generic chat-product widening
- broad provider expansion unrelated to the canonical `Now` or client mesh

### Canonical Now Surface

- [ ] **NOW-01**: The canonical `Now` surface contract is published as durable repo authority for all supported shells
- [ ] **NOW-02**: `Now` header, status row, context one-liner, nudge bars, task list, and docked capture/voice bar are implemented from one shared contract
- [ ] **NOW-03**: `Now` remains execution-first and compact rather than re-expanding into a dashboard or second inbox
- [ ] **NOW-04**: `Now` interactions escalate to `Threads` according to one shared routing model across shells

### Rust Core Portability

- [ ] **CORE-01**: `Now` semantics live in platform-portable Rust core and service layers rather than shell-local policy
- [ ] **CORE-02**: canonical task, thread, day, nudge, and sync/offline models are shared across web and Apple transport
- [ ] **CORE-03**: deterministic fallback behavior for `Now` summaries and context resolution exists outside any single shell
- [ ] **CORE-04**: governed config for `Now` title/display/count policies is Rust-owned and versioned

### Client Mesh And Linking

- [ ] **MESH-01**: Clients can discover or enter authority endpoints in a guided way instead of raw guesswork
- [ ] **MESH-02**: Clients surface current connection status, last sync, and queued writes consistently
- [ ] **MESH-03**: Operators can inspect and recover broken or stale client connections from supported shells
- [ ] **MESH-04**: Multi-client continuity rules are explicit enough that state from one client remains understandable on another

### Cross-Platform Parity

- [ ] **PARITY-01**: Web ships the canonical `Now` embodiment over shared contracts
- [ ] **PARITY-02**: iPhone, iPad, and Mac ship the same `Now` information architecture and core actions over shared contracts
- [ ] **PARITY-03**: Apple Watch ships the reduced `Now` subset without inventing divergent product behavior
- [ ] **PARITY-04**: layout density may adapt by device, but behavior and routing remain contract-identical

### Tasks, Threads, And Continuity

- [ ] **TASK-01**: `task` is the canonical work object feeding `Now`, with commitments represented as a task subtype
- [ ] **TASK-02**: task completion, undo posture, metadata, and overflow behavior are consistent across clients
- [ ] **THREAD-01**: `Now` icon-bar buckets and escalation chips route into shared filtered `Threads` views
- [ ] **THREAD-02**: every docked-bar input creates a thread artifact and can route to inline, inbox, or thread outcomes without shell-specific behavior

### Offline And Trust

- [ ] **SYNC-01**: `Now` and support surfaces distinguish `synced`, `local_only`, and `stale` states
- [ ] **SYNC-02**: offline writes remain inspectable and retryable
- [ ] **SYNC-03**: ambiguous multi-client conflicts surface explicit review instead of silent overwrite

## Future Requirements

- [ ] broader review analytics or journaling
- [ ] multi-day planning beyond the current-day and day-object model
- [ ] broad provider or platform expansion beyond client mesh needs
- [ ] speculative cloud-first or hosted-first product reset

## Traceability

| Requirement | Phase |
|-------------|-------|
| NOW-01 | Phase 46 |
| NOW-02 | Phase 47 |
| NOW-03 | Phase 49 |
| NOW-04 | Phase 47 |
| CORE-01 | Phase 47 |
| CORE-02 | Phase 47 |
| CORE-03 | Phase 47 |
| CORE-04 | Phase 48 |
| MESH-01 | Phase 48 |
| MESH-02 | Phase 48 |
| MESH-03 | Phase 50 |
| MESH-04 | Phase 48 |
| PARITY-01 | Phase 49 |
| PARITY-02 | Phase 50 |
| PARITY-03 | Phase 50 |
| PARITY-04 | Phase 50 |
| TASK-01 | Phase 47 |
| TASK-02 | Phase 49 |
| THREAD-01 | Phase 47 |
| THREAD-02 | Phase 49 |
| SYNC-01 | Phase 48 |
| SYNC-02 | Phase 48 |
| SYNC-03 | Phase 50 |

---
*Last updated: 2026-03-21 for milestone v0.3 canonical Now surface and client mesh planning*

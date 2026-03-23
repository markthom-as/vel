# Requirements: Vel

**Defined:** 2026-03-23
**Milestone:** 0.5.1
**Core Value:** Reliable, local-first capture and recall that a solo operator can trust.

## `0.5.1` Requirements

Requirements for the canonical client reconnection line. This milestone reconnects the web operator surfaces to the frozen `0.5` backend without renegotiating backend law or widening into redesign/migration work.

## Milestone Acceptance Checklist

`0.5.1` is only complete if all of these are true:

- [ ] a truthful-surface doctrine governs all client work
- [ ] the web client uses one canonical transport layer for reads and `WriteIntent` mutations
- [ ] only `Now`, `Threads`, and `System` exist as first-class surfaces
- [ ] `Inbox` is absorbed and `Settings` is replaced by `System`
- [ ] no client-side semantic truth remains
- [ ] deprecated routes/shims are deleted or explicitly quarantined
- [ ] web proof shows truthful reads, truthful writes, drift failure, and no silent fallback
- [ ] Apple receives handoff/spec docs only

## Non-Goals

- backend schema negotiation
- framework migration
- broad UI redesign
- new providers
- trigger/scheduler automation
- workflow-builder product work
- Apple implementation

### Truth Doctrine

- [ ] **TRUTH-51-01**: a standalone truthful-surface doctrine defines allowed data, mutation, derivation, optimistic UI, and failure posture

### Canonical Transport

- [ ] **TRANSPORT-51-01**: web reads and writes flow through one typed canonical transport layer with `WriteIntent`-only mutations

### Surfaces

- [ ] **NOW-51-01**: `Now` renders canonical operational truth and absorbs triage behavior without client-side prioritization or semantic drift
- [ ] **THREADS-51-01**: `Threads` is rebound to canonical backend truth and supports only object-scoped manual invocation
- [ ] **SYSTEM-51-01**: `/system` exists as one structural/configuration surface with stable internal sections and no mini-app split

### Cleanup And Verification

- [ ] **CLEANUP-51-01**: deprecated routes and client shims are deleted or explicitly quarantined with named rationale
- [ ] **VERIFY-51-01**: milestone verification proves truthful read flows, canonical mutation flows, drift failure, and no-silent-fallback behavior
- [ ] **APPLE-51-01**: Apple receives handoff/spec docs only, with no partial implementation in this milestone

## Traceability

| Requirement | Phase |
|-------------|-------|
| TRUTH-51-01 | Phase 66 |
| TRANSPORT-51-01 | Phase 68 |
| NOW-51-01 | Phase 69 |
| THREADS-51-01 | Phase 70 |
| SYSTEM-51-01 | Phase 70 |
| CLEANUP-51-01 | Phase 67, Phase 71 |
| VERIFY-51-01 | Phase 69, Phase 70, Phase 71 |
| APPLE-51-01 | Phase 71 |

## Archived Requirement Sets

- [v0.4-REQUIREMENTS.md](/home/jove/code/vel/.planning/milestones/v0.4-REQUIREMENTS.md)
- [v0.5-REQUIREMENTS.md](/home/jove/code/vel/.planning/milestones/v0.5-REQUIREMENTS.md)

---
*Last updated: 2026-03-23 for the active `0.5.1` client reconnection line*

# Roadmap: Vel `0.5.1` Canonical Client Reconnection

## Status

Active milestone packet. `0.5.1` is now the live release line under execution.

## Milestone Framing

Milestone `0.5.1` reconnects the frontend to the frozen `0.5` backend.

This is a truth-alignment milestone, not a UI rebuild, backend rewrite, or platform migration.

The milestone has exactly three surfaces:

- `Now` — temporal / operational
- `Threads` — contextual / interaction
- `System` — structural / object / configuration

`Inbox` is absorbed into `Now` triage and query views. `Settings` is replaced by `System`. Apple is out of implementation scope and receives a handoff/spec packet only.

`Now` remains one surface, but tasks and calendar commitments must render as adjacent canonical sections rather than a merged ranked feed unless the backend later provides an explicit merged model.

## Invariant

- clients must conform to canonical backend contracts
- backend contracts are immutable during this milestone except for provable bugs
- no schema negotiation
- no client-invented semantic truth

The governing authority doc is [0.5.1-truthful-surface-doctrine.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/0.5.1-truthful-surface-doctrine.md).

## Scope Guardrails

`0.5.1` is only about reconnecting operator surfaces to canonical backend truth:

- truthful-surface doctrine and client contract freeze
- bounded audit of stale client/backend touchpoints
- one canonical transport layer for reads and mutations
- rebinding `Now`, `Threads`, and `System` to canonical truth
- direct operator actions only through canonical `WriteIntent` and backend explain outputs
- deletion or explicit quarantine of deprecated client/backend seams
- execution-backed web proof and Apple handoff docs

Do not widen this milestone into:

- backend schema or ontology negotiation
- framework migration
- broad UI redesign
- new provider integration
- scheduler/trigger automation
- workflow-builder product work
- partial Apple implementation

## Requirement Buckets

| ID | Description |
|----|-------------|
| TRUTH-51-01 | A standalone truthful-surface doctrine defines allowed client data, mutation, derivation, optimistic UI, and failure posture. |
| TRANSPORT-51-01 | Web client reads and writes flow through one typed canonical transport layer with `WriteIntent`-only mutations. |
| NOW-51-01 | `Now` renders canonical operational truth and absorbs triage behavior without client-side prioritization or semantic drift. |
| THREADS-51-01 | `Threads` is rebound to canonical backend truth and supports only object-scoped manual invocation. |
| SYSTEM-51-01 | `System` ships as one authoritative structural/configuration surface under `/system` with a fixed section set and a pre-frozen action allow-list. |
| CLEANUP-51-01 | Deprecated routes and client shims are deleted or explicitly quarantined with named rationale. |
| VERIFY-51-01 | Milestone verification proves truthful read flows, canonical mutation flows, drift failure, stale-data posture, browser execution, and no-silent-fallback behavior. |
| APPLE-51-01 | Apple receives a handoff/spec packet only; no implementation work lands in this milestone. |

## Phases

- [ ] **Phase 66: Truth doctrine, contract freeze, and milestone lock** - Freeze the frontend/backend boundary, define truthful-surface law, and lock the reduced surface model before implementation spreads.
- [ ] **Phase 67: Client contract audit and deprecated seam kill list** - Inventory every current client/backend touchpoint, classify rewrite/quarantine/delete decisions, and lock the kill list before rebinding work starts.
- [ ] **Phase 68: Canonical transport layer and query/mutation discipline** - Land the one true typed read/mutation layer and ban direct fetches or local truth-shaping outside it.
- [ ] **Phase 69: Canonical `Now` rebinding and triage truth** - Rebind `Now` to canonical backend truth, absorb `Inbox` behavior into triage/query flows, and remove client-side ranking or semantic inference.
- [ ] **Phase 70: `Threads` and `System` surface reconnection** - Rebind `Threads` to canonical interaction truth and ship the minimal authoritative `/system` surface with object/configuration visibility and invocation affordances.
- [ ] **Phase 71: Cleanup, web proof, and Apple handoff** - Remove or quarantine deprecated seams, prove the truthful-surface line end-to-end on web, and publish the Apple handoff packet.

## Progress

**Planned execution order:** 66 -> 67 -> 68 -> 69 -> 70 -> 71

| Phase | Requirements | Status |
|-------|--------------|--------|
| 66. Truth doctrine, contract freeze, and milestone lock | TRUTH-51-01, TRANSPORT-51-01, NOW-51-01, THREADS-51-01, SYSTEM-51-01, CLEANUP-51-01, VERIFY-51-01, APPLE-51-01 | Active |
| 67. Client contract audit and deprecated seam kill list | TRUTH-51-01, CLEANUP-51-01 | Planned |
| 68. Canonical transport layer and query/mutation discipline | TRANSPORT-51-01, CLEANUP-51-01 | Planned |
| 69. Canonical `Now` rebinding and triage truth | NOW-51-01, TRANSPORT-51-01, VERIFY-51-01 | Planned |
| 70. `Threads` and `System` surface reconnection | THREADS-51-01, SYSTEM-51-01, TRANSPORT-51-01, VERIFY-51-01 | Planned |
| 71. Cleanup, web proof, and Apple handoff | CLEANUP-51-01, VERIFY-51-01, APPLE-51-01 | Planned |

## Phase Details

### Phase 66: Truth doctrine, contract freeze, and milestone lock

**Goal:** freeze the client truth model and surface scope before any rebinding work begins.
**Requirements:** TRUTH-51-01, TRANSPORT-51-01, NOW-51-01, THREADS-51-01, SYSTEM-51-01, CLEANUP-51-01, VERIFY-51-01, APPLE-51-01
**Depends on:** closed milestone `0.5`
**Success Criteria:**
1. Truthful-surface law is published as durable authority.
2. Surface scope is reduced to `Now`, `Threads`, and `System` only.
3. Backend immutability during the milestone is explicit.
4. No schema-negotiation loophole remains in the packet.

### Phase 67: Client contract audit and deprecated seam kill list

**Goal:** identify all stale client/backend touchpoints and lock whether each is rewritten, quarantined, or deleted.
**Requirements:** TRUTH-51-01, CLEANUP-51-01
**Depends on:** Phase 66
**Success Criteria:**
1. Every known legacy client/backend touchpoint is inventoried.
2. Deprecated route kill list is explicit and named up front.
3. Temporary shims, if any, are explicitly bounded rather than vague.

### Phase 68: Canonical transport layer and query/mutation discipline

**Goal:** create a single typed client read/write boundary that consumes canonical backend truth without local renegotiation.
**Requirements:** TRANSPORT-51-01, CLEANUP-51-01
**Depends on:** Phase 67
**Success Criteria:**
1. One typed query layer exists.
2. One typed mutation layer exists and uses `WriteIntent` only.
3. Direct fetches outside this layer are forbidden by design and verification.

### Phase 69: Canonical `Now` rebinding and triage truth

**Goal:** make `Now` the truthful temporal/operational surface over canonical task/calendar state.
**Requirements:** NOW-51-01, TRANSPORT-51-01, VERIFY-51-01
**Depends on:** Phase 68
**Success Criteria:**
1. `Now` uses only canonical operational inputs.
2. Triage mutations are direct but always canonical `WriteIntent`.
3. No client-side prioritization or semantic inference survives.
4. `Inbox` behavior is absorbed without reviving `Inbox` as a first-class surface.
5. Tasks and calendar commitments remain adjacent canonical sections rather than a synthetic merged ranking feed.

### Phase 70: `Threads` and `System` surface reconnection

**Goal:** reconnect contextual interaction and structural/configuration surfaces to canonical truth.
**Requirements:** THREADS-51-01, SYSTEM-51-01, TRANSPORT-51-01, VERIFY-51-01
**Depends on:** Phase 69
**Success Criteria:**
1. `Threads` consumes canonical backend truth and supports only object-scoped manual invocation.
2. `/system` ships as one surface with stable sections, not route sprawl.
3. `Modules`, `Integrations`, `Accounts`, and `Scopes` expose only pre-frozen named canonical actions; no additional, inferred, composite, or ambiguous actions are allowed.
4. No builders, editors, or client-side workflow simulation land in this milestone.

### Phase 71: Cleanup, web proof, and Apple handoff

**Goal:** finish the truth-alignment line by deleting stale seams, proving the web line, and handing Apple a faithful spec packet.
**Requirements:** CLEANUP-51-01, VERIFY-51-01, APPLE-51-01
**Depends on:** Phase 70
**Success Criteria:**
1. Deprecated routes are removed or explicitly quarantined.
2. Web proof demonstrates truthful reads, truthful mutations, drift failure, stale-data posture, and no-silent-fallback.
3. Browser-executed evidence and short human-readable proof notes exist for each major flow.
4. Apple receives behavior/contract handoff docs only, with no partial implementation drift.

---
*Drafted: 2026-03-23 from post-`0.5` milestone direction lock*

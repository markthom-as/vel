# Roadmap: Vel `0.5.2` Operator Surface Embodiment

## Status

Active milestone packet.

## Milestone Framing

Milestone `v0.5.2` turns the truthful `0.5.1` client line into the intended operator UI state.

This is a web-first embodiment milestone. It is allowed to substantially improve layout, information density, interaction rhythm, and perceived speed, but it is not allowed to renegotiate backend truth or widen product scope into new surfaces, providers, or workflow-builder behavior.

The milestone still recognizes exactly three first-class surfaces:

- `Now` — temporal / operational
- `Threads` — contextual / interaction
- `System` — structural / configuration

The governing authority doc is [0.5.2-operator-surface-doctrine.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/0.5.2-operator-surface-doctrine.md).

## Invariant

- backend truth remains frozen unless a provable bug forces a fix
- no new backend endpoints unless they are explicitly pre-declared in this milestone packet
- client-side semantic invention remains forbidden
- web is the implementation target
- Apple is handoff/parity-only in this milestone
- no new top-level surfaces are introduced

## Scope Guardrails

`0.5.2` is only about ideal-state surface embodiment:

- lock the operator-surface doctrine and UI direction before implementation spreads
- strengthen shared shell rhythm, layout primitives, and interaction hierarchy
- make `Now` feel like the intended execution-first operating surface
- make `Threads` feel grounded, legible, and actionably contextual
- make `System` feel authoritative without turning into a sprawling admin UI
- cut obvious perceived-latency regressions in the active operator path
- prove the shipped surface in the browser and refresh Apple parity/handoff notes

Do not widen this milestone into:

- backend schema or ontology renegotiation
- framework migration
- new providers
- workflow-builder product work
- trigger/scheduler widening
- Apple implementation

## Requirement Buckets

| ID | Description |
|----|-------------|
| DOCTRINE-52-01 | A standalone operator-surface doctrine defines the ideal-state UI law for `Now`, `Threads`, and `System`. |
| SHELL-52-01 | Shared shell, layout, disclosure, and navigation rhythm support the three-surface model without reintroducing route sprawl or shell noise. |
| NOW-52-01 | `Now` becomes the intended execution-first surface with higher density, clearer triage, and canonical task/calendar presentation without synthetic cross-type ranking. |
| THREADS-52-01 | `Threads` becomes a grounded contextual work surface with clearer object truth, provenance, and invocation posture. |
| SYSTEM-52-01 | `System` becomes a legible structural/configuration surface with stable sections, detail states, and only named canonical actions. |
| PERF-52-01 | The active web operator path reduces visible latency and unnecessary churn without reopening backend law. |
| VERIFY-52-01 | Browser-executed proof, focused tests, build verification, and milestone evidence close the line honestly. |
| APPLE-52-01 | Apple receives updated parity/handoff documentation reflecting the new surface embodiment without implementation work. |

## Phases

- [x] **Phase 72: Operator-surface doctrine, UI contract, and milestone lock** - Freeze the ideal-state UI law, define the embodiment target, and activate the milestone without reopening backend truth.
- [ ] **Phase 73: Shared shell rhythm, layout primitives, and disclosure system** - Repair the chrome, section rhythm, information hierarchy, and reusable primitives that every surface will depend on.
- [ ] **Phase 74: `Now` ideal-state embodiment and operator-speed repair** - Rework `Now` into the intended dense execution-first surface and cut the most obvious active-path latency regressions.
- [ ] **Phase 75: `Threads` ideal-state embodiment and grounded interaction clarity** - Rework `Threads` so object grounding, provenance, and bounded invocation feel clear rather than incidental.
- [ ] **Phase 76: `System` ideal-state embodiment and structural legibility** - Rework `/system` into the intended structural/configuration surface without widening its authority or action set.
- [ ] **Phase 77: Cross-surface proof, cleanup, and parity handoff** - Close the line with browser proof, cleanup, evidence, and Apple parity/handoff refresh.

## Progress

**Planned execution order:** 72 -> 73 -> 74 -> 75 -> 76 -> 77

| Phase | Requirements | Status |
|-------|--------------|--------|
| 72. Operator-surface doctrine, UI contract, and milestone lock | DOCTRINE-52-01, SHELL-52-01, NOW-52-01, THREADS-52-01, SYSTEM-52-01, PERF-52-01, VERIFY-52-01, APPLE-52-01 | Complete |
| 73. Shared shell rhythm, layout primitives, and disclosure system | SHELL-52-01, PERF-52-01, VERIFY-52-01 | Planned |
| 74. `Now` ideal-state embodiment and operator-speed repair | NOW-52-01, PERF-52-01, VERIFY-52-01 | Planned |
| 75. `Threads` ideal-state embodiment and grounded interaction clarity | THREADS-52-01, PERF-52-01, VERIFY-52-01 | Planned |
| 76. `System` ideal-state embodiment and structural legibility | SYSTEM-52-01, PERF-52-01, VERIFY-52-01 | Planned |
| 77. Cross-surface proof, cleanup, and parity handoff | VERIFY-52-01, APPLE-52-01, PERF-52-01 | Planned |

## Phase Details

### Phase 72: Operator-surface doctrine, UI contract, and milestone lock

**Goal:** define the ideal-state operator surface target before implementation spreads.
**Requirements:** DOCTRINE-52-01, SHELL-52-01, NOW-52-01, THREADS-52-01, SYSTEM-52-01, PERF-52-01, VERIFY-52-01, APPLE-52-01
**Depends on:** closed milestone `0.5.1`
**Success Criteria:**
1. Operator-surface doctrine exists as durable authority.
2. `Now`, `Threads`, and `System` are restated as the only first-class surfaces.
3. UI embodiment scope is clear without reopening backend law.
4. Performance and proof expectations are explicit from the start.
5. The `Now` priority gradient, `Threads` emphasis model, `System` grouping, and backend-endpoint freeze are explicitly locked.
6. `Now` layout/design definition is locked before implementation and cannot drift without explicit approval.

### Phase 73: Shared shell rhythm, layout primitives, and disclosure system

**Goal:** repair the shell and shared UI substrate before surface-specific work diverges.
**Requirements:** SHELL-52-01, PERF-52-01, VERIFY-52-01
**Depends on:** Phase 72
**Success Criteria:**
1. Shared shell rhythm reinforces the three-surface model.
2. Reusable layout and disclosure primitives exist for dense but legible operator UI.
3. The shell no longer contributes unnecessary latency or structural noise.

### Phase 74: `Now` ideal-state embodiment and operator-speed repair

**Goal:** make `Now` feel like the intended execution-first surface.
**Requirements:** NOW-52-01, PERF-52-01, VERIFY-52-01
**Depends on:** Phase 73
**Success Criteria:**
1. `Now` uses a denser, clearer canonical layout over adjacent task/calendar sections.
2. Direct triage actions remain canonical and feel immediate.
3. Visible active-path latency is improved relative to the `0.5.1` line.
4. No synthetic cross-type ranking or local semantic truth sneaks back in.
5. `Focus` remains singular and dominant rather than degrading into “just another list.”
6. Any material `Now` layout change follows the explicitly approved layout/design contract rather than implementation drift.

### Phase 75: `Threads` ideal-state embodiment and grounded interaction clarity

**Goal:** make `Threads` feel grounded, inspectable, and actionably contextual.
**Requirements:** THREADS-52-01, PERF-52-01, VERIFY-52-01
**Depends on:** Phase 74
**Success Criteria:**
1. Object grounding, provenance, and eligibility are legible without clutter.
2. Invocation posture stays bounded and explicit.
3. Interaction flow feels intentional rather than a leftover shell seam.
4. Bound object state is primary and chronology remains secondary.

### Phase 76: `System` ideal-state embodiment and structural legibility

**Goal:** turn `/system` into the intended structural/configuration surface without widening authority.
**Requirements:** SYSTEM-52-01, PERF-52-01, VERIFY-52-01
**Depends on:** Phase 75
**Success Criteria:**
1. `/system` remains one route with clearer section hierarchy and detail states.
2. Read-heavy structural truth is easier to inspect quickly.
3. Only pre-frozen named canonical actions remain visible.
4. The surface becomes authoritative without becoming a sprawling admin product.
5. `Domain`, `Capabilities`, and `Configuration` remain visibly distinct.

### Phase 77: Cross-surface proof, cleanup, and parity handoff

**Goal:** close the milestone honestly with proof, cleanup, and parity documentation.
**Requirements:** VERIFY-52-01, APPLE-52-01, PERF-52-01
**Depends on:** Phase 76
**Success Criteria:**
1. Browser-executed proof exists for the shipped surfaces and major operator flows.
2. Focused tests and `clients/web` build pass.
3. Cleanup removes or explicitly documents any stale seams touched by the line.
4. Apple parity/handoff docs reflect the embodied surface model without implying implementation.
5. Evidence includes per-surface browser proof and one full operator-loop proof.

---
*Drafted: 2026-03-22 from post-`0.5.1` direction lock*

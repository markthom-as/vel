# Roadmap: Vel

## Overview

Phase 1 (structural decomposition, auth hardening, canonical schemas, self-awareness contracts) is complete. The remaining roadmap now covers nine ordered phases: distributed state and offline-capable clients with agent supervision (Phase 2), deterministic verification and continuous alignment (Phase 3), autonomous swarm execution with semantic memory and zero-trust sandboxing (Phase 4), then a product-shaping sequence focused on `Now + Inbox`, project substrate, high-value write-back integrations, Apple action loops, coding-centric supervised execution, and backup-first trust surfaces (Phases 5-9). Each phase produces a verifiable capability boundary before the next begins.

## Phases

**Phase Numbering:**
- Phases 2–4 continue from completed Phase 1
- Integer phases only; decimal phases created via `/gsd:insert-phase` if urgent work is needed

- [x] **Phase 1: Structural Foundation** - Layered crates, auth hardening, canonical schemas, self-awareness (COMPLETE)
- [x] **Phase 1.1: Preflight — Pre-Phase 2 Hardening** (INSERTED) - Integration startup panic fixes, SQLite WAL mode, app.rs decomposition (COMPLETE)
- [ ] **Phase 2: Distributed State, Offline Clients & System-of-Systems** - Signal ingestion, HLC sync, agent connect, capability brokering, operator accessibility
- [x] **Phase 3: Deterministic Verification & Continuous Alignment** - Day-simulation harness, LLM-as-a-Judge eval, execution tracing, user documentation (COMPLETE)
- [ ] **Phase 4: Autonomous Swarm, Graph RAG & Zero-Trust Execution** - Semantic memory graph, WASM sandboxing, swarm execution SDK
- [ ] **Phase 5: Now + Inbox core and project substrate** - Keep `Now + Inbox` primary while adding durable project structure and shared workspace contracts
- [ ] **Phase 6: High-value write-back integrations and lightweight people graph** - Add notes, reminders, GitHub, email, transcripts, and minimal people identity with upstream write-back
- [ ] **Phase 7: Apple action loops and behavioral signal ingestion** - Prioritize fast iOS/watch actions and directly useful behavior signals
- [ ] **Phase 8: Coding-centric supervised execution with GSD and local agents** - Launch and supervise coding-first runtimes with direct GSD integration and local-agent support
- [ ] **Phase 9: Backup-first trust surfaces and simple operator control** - Add backup-first trust workflows and keep control/config surfaces simple

## Phase Details

### Phase 1.1: Preflight — Pre-Phase 2 Hardening (INSERTED)
**Goal**: Eliminate crash-risk technical debt and close infrastructure gaps that will become more expensive once distributed complexity grows. This phase has no new feature surface — it is purely hardening. Must complete before Phase 2 execution begins.
**Depends on**: Phase 1 (complete)
**Success Criteria** (what must be TRUE):
  1. Daemon starts cleanly with missing or corrupt Todoist/Google Calendar settings — no panics, graceful degradation with warning logs
  2. SQLite WAL mode is enabled; concurrent readers do not block background writers
  3. `app.rs` auth middleware and exposure gate logic extracted to a separate module; file is under 3,000 lines
**Work items**:
  - Fix `expect()` panics on integration settings load in `integrations_todoist.rs` and `integrations_google.rs`
  - Enable WAL mode in `vel-storage/src/infra.rs` database initialization
  - Extract auth middleware + `HttpExposurePolicy` from `app.rs` to `crates/veld/src/middleware/`
**Plans**: 1 plan
Plans:
- [ ] 1.1-01-PLAN.md — TDD: Wave 0 failing tests + WAL mode + integration settings hardening + middleware extraction

### Phase 2: Distributed State, Offline Clients & System-of-Systems
**Goal**: The system can ingest signals from pluggable sources, maintain consistent distributed state across nodes, launch and supervise agent processes, broker capabilities without exposing raw credentials, and present clear effective configuration to the operator.
**Depends on**: Phase 1
**Requirements**: SIG-01, SIG-02, SYNC-01, SYNC-02, CONN-01, CONN-02, CONN-03, CONN-04, CAP-01, CAP-02, OPS-01, OPS-02
**Tickets**: `docs/tickets/phase-2/` — 004, 005, 006, 012, 016, 019
**Parallel board**: `docs/tickets/phase-2/parallel-execution-board.md`
**Success Criteria** (what must be TRUE):
  1. Operator can register a new signal source and see its data flowing into unified context state without code changes to the core pipeline
  2. Two nodes with divergent event logs converge to the same state after sync, with no manual conflict resolution required
  3. An agent process can be launched via the connect protocol, heartbeats are recorded, and the process is cleanly terminated on operator command
  4. A new node discovers the system and completes onboarding diagnostics without manual configuration
  5. Agent capability requests resolve to scoped tokens; no raw credentials appear in prompts or logs
  6. The operator CLI and web dashboard both display effective (resolved) configuration state, not raw config file values
**Plans**: 7 plans
Plans:
- [ ] 02-01-PLAN.md — SP1: Contract alignment, operator diagnostics, connect surface consistency (Wave 1)
- [ ] 02-02-PLAN.md — SP2 Lane A: Signal reducer extraction — SignalReducer trait + ReducerRegistry (Wave 2, TDD)
- [ ] 02-03-PLAN.md — SP2 Lane B: Connect lifecycle MVP — launch/heartbeat/terminate/expiry (Wave 2)
- [ ] 02-04-PLAN.md — SP2 Lane C: Capability broker MVP — CapabilityDescriptor + BrokerService (Wave 2, TDD)
- [ ] 02-05-PLAN.md — SP3 Lane A+B: HLC sync ordering primitive + node link CLI/web/Apple (Wave 3)
- [ ] 02-05b-PLAN.md — SP3 Pairing backend: POST /api/node/pair/issue token generation + storage (Wave 3, after 02-05)
- [ ] 02-06-PLAN.md — SP3 Lane C: Accessibility/config clarity — vel config show + canonical terminology (Wave 4)

### Phase 3: Deterministic Verification & Continuous Alignment
**Goal**: The system can replay any recorded day deterministically to verify correctness, evaluate agent reasoning outputs via an LLM judge, and expose complete execution traces to the operator — giving the operator confidence that agent behavior is auditable and regressions are detectable.
**Depends on**: Phase 2
**Requirements**: VERIFY-01, VERIFY-02, EVAL-01, EVAL-02, TRACE-01, TRACE-02, TRACE-03, DOCS-01, DOCS-02
**Tickets**: `docs/tickets/phase-3/` — 007, 008, 017, 013
**Parallel board**: `docs/tickets/phase-3/parallel-execution-board.md`
**Success Criteria** (what must be TRUE):
  1. Running the day-simulation harness twice against the same recorded event log produces identical output both times
  2. Operator can submit an agent reasoning output for evaluation and receive a structured judge verdict stored in the database
  3. LLM eval results are queryable by date range and model so regressions can be spotted across runs
  4. Every agent run has a stable run ID; the operator can pull a full trace including inter-agent handoffs from the dashboard
  5. Operator-facing workflows are covered by documentation accessible from a searchable wiki
**Plans**: 5 plans
Plans:
- [x] 03-01-PLAN.md — SP1 Lane A/B entry: shared trace contract + run inspection linkage + contract docs
- [x] 03-02-PLAN.md — SP1 Lane B: CLI/web trace inspection surfaces
- [x] 03-03-PLAN.md — SP1 Lane C: user docs/support parity + recovery architecture
- [x] 03-04-PLAN.md — SP2: deterministic day-simulation harness + replay assertions
- [x] 03-05-PLAN.md — SP3: eval runner, judge integration, and reporting gates

### Phase 4: Autonomous Swarm, Graph RAG & Zero-Trust Execution
**Goal**: The system maintains a semantic memory graph over captured entities, uses graph-based retrieval to surface relevant context during reasoning, executes untrusted agents in WASM sandboxes with zero-trust defaults, and provides a first-class SDK for building external agent Limbs that communicate via a standardized swarm contract.
**Depends on**: Phase 3
**Requirements**: MEM-01, MEM-02, SAND-01, SAND-02, SDK-01, SDK-02, SDK-03
**Tickets**: `docs/tickets/phase-4/` — 009, 010, 014
**Parallel board**: `docs/tickets/phase-4/parallel-execution-board.md`
**Success Criteria** (what must be TRUE):
  1. Captured entities are reflected in the semantic memory graph; a reasoning step can retrieve contextually relevant memories via graph RAG
  2. An untrusted agent binary executes inside the WASM sandbox and can only interact with veld through declared capability contracts — any attempt to exceed declared permissions is rejected at the boundary
  3. A developer can import `vel-agent-sdk`, implement the swarm execution contract, and have their agent communicate with veld without touching veld internals
  4. The SDK ships with at least one working reference implementation and documentation covering the full integration contract
**Plans**: 5 plans
Plans:
- [x] 04-01-PLAN.md — SP1: semantic-memory, sandbox-ABI, and swarm-protocol contract foundations
- [x] 04-02-PLAN.md — SP2 Lane A: semantic index backend seam, provenance-preserving records, and retrieval lifecycle
- [x] 04-03-PLAN.md — SP2 Lane B: WASM sandbox runtime, deny-by-default policies, and operator-visible decisions
- [ ] 04-04-PLAN.md — SP3 Lane A: swarm protocol crate, fixtures, and versioned serialization/validation
- [ ] 04-05-PLAN.md — SP3 Lane B: reference SDK limb and end-to-end scoped capability flow

## Progress

**Execution Order:**
Phases execute in numeric order: 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Structural Foundation | - | Complete | 2026-03-18 |
| 1.1. Preflight — Pre-Phase 2 Hardening | 1/1 | Complete | 2026-03-18 |
| 2. Distributed State, Offline Clients & System-of-Systems | 0/7 | Not started | - |
| 3. Deterministic Verification & Continuous Alignment | 5/5 | Complete | 2026-03-18 |
| 4. Autonomous Swarm, Graph RAG & Zero-Trust Execution | 3/5 | In progress | - |
| 5. Now + Inbox core and project substrate | 0/0 | Not planned | - |
| 6. High-value write-back integrations and lightweight people graph | 0/0 | Not planned | - |
| 7. Apple action loops and behavioral signal ingestion | 0/0 | Not planned | - |
| 8. Coding-centric supervised execution with GSD and local agents | 0/0 | Not planned | - |
| 9. Backup-first trust surfaces and simple operator control | 0/0 | Not planned | - |

### Phase 5: Now + Inbox core and project substrate

**Goal:** [To be planned]
**Requirements**: TBD
**Depends on:** Phase 4
**Plans:** 0 plans

Plans:
- [ ] TBD (run /gsd:plan-phase 5 to break down)

### Phase 6: High-value write-back integrations and lightweight people graph

**Goal:** [To be planned]
**Requirements**: TBD
**Depends on:** Phase 5
**Plans:** 0 plans

Plans:
- [ ] TBD (run /gsd:plan-phase 6 to break down)

### Phase 7: Apple action loops and behavioral signal ingestion

**Goal:** [To be planned]
**Requirements**: TBD
**Depends on:** Phase 6
**Plans:** 0 plans

Plans:
- [ ] TBD (run /gsd:plan-phase 7 to break down)

### Phase 8: Coding-centric supervised execution with GSD and local agents

**Goal:** [To be planned]
**Requirements**: TBD
**Depends on:** Phase 7
**Plans:** 0 plans

Plans:
- [ ] TBD (run /gsd:plan-phase 8 to break down)

### Phase 9: Backup-first trust surfaces and simple operator control

**Goal:** [To be planned]
**Requirements**: TBD
**Depends on:** Phase 8
**Plans:** 0 plans

Plans:
- [ ] TBD (run /gsd:plan-phase 9 to break down)

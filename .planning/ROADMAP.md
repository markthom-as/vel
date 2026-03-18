# Roadmap: Vel

## Overview

Phase 1 (structural decomposition, auth hardening, canonical schemas, self-awareness contracts) is complete. The remaining work delivers three capabilities in sequence: distributed state and offline-capable clients with agent supervision (Phase 2), deterministic verification and continuous alignment so the system can be trusted and audited (Phase 3), and finally autonomous swarm execution with semantic memory and zero-trust sandboxing (Phase 4). Each phase produces a verifiable capability boundary before the next begins.

## Phases

**Phase Numbering:**
- Phases 2–4 continue from completed Phase 1
- Integer phases only; decimal phases created via `/gsd:insert-phase` if urgent work is needed

- [x] **Phase 1: Structural Foundation** - Layered crates, auth hardening, canonical schemas, self-awareness (COMPLETE)
- [x] **Phase 1.1: Preflight — Pre-Phase 2 Hardening** (INSERTED) - Integration startup panic fixes, SQLite WAL mode, app.rs decomposition (COMPLETE)
- [ ] **Phase 2: Distributed State, Offline Clients & System-of-Systems** - Signal ingestion, HLC sync, agent connect, capability brokering, operator accessibility
- [ ] **Phase 3: Deterministic Verification & Continuous Alignment** - Day-simulation harness, LLM-as-a-Judge eval, execution tracing, user documentation
- [ ] **Phase 4: Autonomous Swarm, Graph RAG & Zero-Trust Execution** - Semantic memory graph, WASM sandboxing, swarm execution SDK

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

---

### Phase 2: Distributed State, Offline Clients & System-of-Systems
**Goal**: The system can ingest signals from pluggable sources, maintain consistent distributed state across nodes, launch and supervise agent processes, broker capabilities without exposing raw credentials, and present clear effective configuration to the operator.
**Depends on**: Phase 1 (complete)
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
**Plans**: TBD

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
**Plans**: TBD

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
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Structural Foundation | - | Complete | 2026-03-18 |
| 1.1. Preflight — Pre-Phase 2 Hardening | 1/1 | Complete | 2026-03-18 |
| 2. Distributed State, Offline Clients & System-of-Systems | 0/TBD | Not started | - |
| 3. Deterministic Verification & Continuous Alignment | 0/TBD | Not started | - |
| 4. Autonomous Swarm, Graph RAG & Zero-Trust Execution | 0/TBD | Not started | - |

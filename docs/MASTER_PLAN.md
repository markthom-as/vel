# Vel: The Master Plan
**Status**: Canonical Truth (v1.0.5)
**Last Updated**: 2026-03-17

---

## High-Level Status Summary

| Phase | Status | Focus |
| :--- | :--- | :--- |
| **v0: MVP** | **[75% - BLOCKED]** | Local core functionality and reliable capture. |
| **Phase 1** | **[COMPLETE]** | Monolith decomposition and structural foundations. |
| **Phase 2** | **[PARTIAL]** | Distributed swarm, offline sync, and agent connect. |
| **Phase 3** | **[PARTIAL]** | Deterministic verification, tracing, and reasoning eval closure. |
| **Phase 4** | **[PLANNED]** | Semantic memory, WASM sandboxing, and P2P sync. |

---

## Agentic Infrastructure & Developer Experience (ADX)
**Goal**: Make Vel the premier platform for autonomous agent development by providing high-signal tooling, standardized protocols, and a safe execution sandbox.

1.  **Standardized Ticket Implementation**: Every task is defined by a technical, "agent-optimized" ticket in `docs/tickets/`.
2.  **Implementation Protocol**: Autonomous coding agents follow the **`docs/templates/agent-implementation-protocol.md`**.
3.  **Execution-Backed Verification**: Agent output is not trusted until it has been tested, executed, or manually verified through a real surface.
4.  **Capability Mediation**: Agents should use scoped capabilities, brokered requests, and secret indirection instead of prompt-visible raw credentials.
5.  **Execution Observability**: Agent runs, handoffs, external calls, and major workflow transitions should produce stable run IDs, traces, or equivalent event linkage.
6.  **Cross-Cutting Trait Discipline**: New subsystem work should explicitly account for modularity, accessibility, configurability, logging, rewind/replay, and composability.
7.  **Canonical Contracts First**: Schema docs, manifests, config templates, and architecture contracts should be defined before broad cross-layer implementation work.
8.  **Unified Agent SDK**: Provide a `vel-agent-sdk` for building new "Limbs" that can safely interact with the "Brain."
9.  **Local LLM Eval Harness**: Automated verification of agent reasoning using a "Judge" model, paired with deterministic and execution-backed checks.

---
## Architecture Lock-In Queue (Execute First, Parallelizable)

Before broad implementation expansion, run the architecture-first queue in `docs/tickets/architecture-first-parallel-queue.md`.
Use the phase execution boards for current lane ownership and write-scope boundaries:
- `docs/tickets/phase-2/parallel-execution-board.md`
- `docs/tickets/phase-3/parallel-execution-board.md`
- `docs/tickets/phase-4/parallel-execution-board.md`

The minimum phase-1 architecture lock-in lane is:

- `011-documentation-truth-repair.md`
- `018-cross-cutting-system-traits-baseline.md`
- `020-documentation-catalog-single-source.md`
- `021-canonical-schema-and-config-contracts.md`
- `022-data-sources-and-connector-architecture.md`
- `023-self-awareness-and-supervised-self-modification.md`
- `024-machine-readable-schema-and-manifest-publication.md`
- `025-config-and-contract-fixture-parity.md`

Primary outcome:
- docs, schemas, manifests, templates, and self-awareness contracts are canonical before deeper runtime spread.

---

## Phase 1: Structural Foundations & Monolith Decomposition
**Current Status: [COMPLETE]**

### 1.1 Storage Repository Pattern & Transaction Lifecycles **[COMPLETE]**
*   *Ticket*: `001-storage-modularization.md`

### 1.2 The "Pure Core" & Typed Context Mandate **[COMPLETE]**
*   *Ticket*: `002-typed-context-transition.md`

### 1.3 Service/DTO Boundary & Standardized Error Handling **[COMPLETE]**
*   *Ticket*: `003-service-dto-layering.md`

### 1.4 Documentation Truth Repair & Architecture Mapping **[COMPLETE]**
*   *Ticket*: `011-documentation-truth-repair.md`

### 1.5 Auth-By-Default HTTP Surfaces & Deny-By-Default Routing **[COMPLETE]**
*   *Ticket*: `015-http-surface-auth-hardening.md`

### 1.6 Cross-Cutting Trait Baseline & Subsystem Audit **[COMPLETE]**
*   *Ticket*: `018-cross-cutting-system-traits-baseline.md`

### 1.7 Documentation Catalog Single Source & Surface Parity **[COMPLETE]**
*   *Ticket*: `020-documentation-catalog-single-source.md`

### 1.8 Canonical Schemas, Config Contracts & Templates **[COMPLETE]**
*   *Ticket*: `021-canonical-schema-and-config-contracts.md`

### 1.9 Canonical Data Sources, Integrations & Connector Contracts **[COMPLETE]**
*   *Ticket*: `022-data-sources-and-connector-architecture.md`

### 1.10 Self-Awareness, Repo Visibility & Supervised Self-Modification **[COMPLETE]**
*   *Ticket*: `023-self-awareness-and-supervised-self-modification.md`

### 1.11 Machine-Readable Contract Manifest Publication **[COMPLETE]**
*   *Ticket*: `024-machine-readable-schema-and-manifest-publication.md`

### 1.12 Config Template And Fixture Parity **[COMPLETE]**
*   *Ticket*: `025-config-and-contract-fixture-parity.md`

### Evidence Dashboard

- See the consolidated evidence tracker for phase-1 status + proof: [docs/tickets/phase-1/phase-1-evidence-matrix.md](tickets/phase-1/phase-1-evidence-matrix.md)

---

## Phase 2: Distributed State, Offline Clients & System-of-Systems
**Current Status: [IN PROGRESS — SP1 active]**

Execution board: `docs/tickets/phase-2/parallel-execution-board.md`

Phase 2 is organized into three sub-phases (SP1/SP2/SP3). See the parallel execution board for current lane ownership and merge gates.

- **SP1 (Contract Alignment & Visibility Closure)** — active: normalize ticket scope, close operator diagnostics gaps, align CLI/runtime behavior.
- **SP2 (Core Runtime Delivery)** — queued: reducer extraction, connect lifecycle MVP, capability broker MVP.
- **SP3 (Onboarding, Conflict Hardening & Cross-Surface Closure)** — queued: depends on SP2 seams.

### 2.1 Pluggable Signal Ingestion & Context Reducer Pipeline **[PLANNED]**
*   *Ticket*: `004-signal-reducer-pipeline.md`
*   *Status*: planned — no reducer infrastructure shipped yet. SP2 Lane A.

### 2.2 Sync Ordering & Conflict Resolution Baseline **[PLANNED]**
*   *Ticket*: `005-hlc-sync-implementation.md`
*   *Status*: planned — ordering primitive not yet implemented. SP3 Lane B. Requires `NodeIdentity` type (vel-core) and WAL mode (confirmed in Phase 1.1).

### 2.3 Agent Connect Launch Protocol & Supervision **[IN PROGRESS — shell only]**
*   *Ticket*: `006-connect-launch-protocol.md`
*   *Status*: in-progress (shell only) — heartbeat infra exists in client_sync.rs, connect routes registered in app.rs but all return 403 via deny_undefined_route. CLI stubs exist. All 4 acceptance criteria unmet. SP2 Lane B.

### 2.4 Tester-Readiness Onboarding & Node Discovery **[IN PROGRESS — Slice A partial]**
*   *Ticket*: `012-tester-readiness-onboarding.md`
*   *Status*: in-progress — Slice A (source discovery, freshness visibility) partial. Slice B (vel node link, pairing tokens, guided wizard) not started. SP3 Lane A.

### 2.5 Capability Broker & Secret Mediation **[PLANNED]**
*   *Ticket*: `016-capability-broker-secret-mediation.md`
*   *Status*: planned — scope: agents-only (integration-level brokering deferred per 2026-03-18 decision). SP2 Lane C.

### 2.6 Operator Surface Accessibility & Effective Config Clarity **[IN PROGRESS — baseline present]**
*   *Ticket*: `019-operator-accessibility-config-clarity.md`
*   *Status*: in-progress — baseline operator clarity exists (freshness/degraded state in web, readable config in CLI, endpoint resolution in Apple). Remaining work: terminology alignment, accessibility gap closure, config discoverability. SP1 Lane B (diagnostics) + SP3 Lane C (full closure).

---

## Phase 3: Deterministic Verification & Continuous Alignment
**Current Status: [PARTIAL]**

Execution board: `docs/tickets/phase-3/parallel-execution-board.md`

### 3.1 The Deterministic Replay Engine (Day-Simulation Harness) **[PLANNED]**
*   *Ticket*: `007-day-simulation-harness.md`

### 3.2 LLM-as-a-Judge Evaluation Pipeline (Evals) **[PLANNED]**
*   *Ticket*: `008-llm-eval-pipeline.md`

### 3.3 Execution Tracing, Handoff Telemetry & Reviewability **[PARTIAL]**
*   *Ticket*: `017-execution-tracing-reviewability.md`

### 3.4 Comprehensive User Documentation & Support Wiki **[PARTIAL]**
*   *Ticket*: `013-user-documentation-architecture.md`

---

## Phase 4: Autonomous Swarm, Graph RAG & Zero-Trust Execution
**Current Status: [PLANNED]**

Execution board: `docs/tickets/phase-4/parallel-execution-board.md`

### 4.1 Semantic Memory & Graph RAG **[PLANNED]**
*   *Ticket*: `009-semantic-memory-rag.md`

### 4.2 Zero-Trust WASM Agent Sandboxing **[PLANNED]**
*   *Ticket*: `010-wasm-agent-sandboxing.md`

### 4.3 Swarm Execution SDK & Contract **[PLANNED]**
*   **Goal**: Provide a first-class SDK for external agents to communicate with Vel.
*   **Ticket**: `014-swarm-execution-sdk.md`

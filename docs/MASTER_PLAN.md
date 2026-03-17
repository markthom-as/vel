# Vel: The Master Plan
**Status**: Canonical Truth (v1.0.2)
**Last Updated**: 2026-03-17

---

## High-Level Status Summary

| Phase | Status | Focus |
| :--- | :--- | :--- |
| **v0: MVP** | **[75% - BLOCKED]** | Local core functionality and reliable capture. |
| **Phase 1** | **[PARTIAL]** | Monolith decomposition and structural foundations. |
| **Phase 2** | **[PARTIAL]** | Distributed swarm, offline sync, and agent connect. |
| **Phase 3** | **[PLANNED]** | Deterministic verification and reasoning evals. |
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
7.  **Unified Agent SDK**: Provide a `vel-agent-sdk` for building new "Limbs" that can safely interact with the "Brain."
8.  **Local LLM Eval Harness**: Automated verification of agent reasoning using a "Judge" model, paired with deterministic and execution-backed checks.

---

## Phase 1: Structural Foundations & Monolith Decomposition
**Current Status: [PARTIAL]**

### 1.1 Storage Repository Pattern & Transaction Lifecycles **[IN_PROGRESS]**
*   *Ticket*: `001-storage-modularization.md`

### 1.2 The "Pure Core" & Typed Context Mandate **[PLANNED]**
*   *Ticket*: `002-typed-context-transition.md`

### 1.3 Service/DTO Boundary & Standardized Error Handling **[PARTIAL]**
*   *Ticket*: `003-service-dto-layering.md`

### 1.4 Documentation Truth Repair & Architecture Mapping **[IN_PROGRESS]**
*   *Ticket*: `011-documentation-truth-repair.md`

### 1.5 Auth-By-Default HTTP Surfaces & Deny-By-Default Routing **[PLANNED]**
*   *Ticket*: `015-http-surface-auth-hardening.md`

### 1.6 Cross-Cutting Trait Baseline & Subsystem Audit **[PLANNED]**
*   *Ticket*: `018-cross-cutting-system-traits-baseline.md`

---

## Phase 2: Distributed State, Offline Clients & System-of-Systems
**Current Status: [PARTIAL]**

### 2.1 Pluggable Signal Ingestion & Context Reducer Pipeline **[PLANNED]**
*   *Ticket*: `004-signal-reducer-pipeline.md`

### 2.2 Offline-First Apple Clients & HLC Synchronization **[PLANNED]**
*   *Ticket*: `005-hlc-sync-implementation.md`

### 2.3 Agent Connect Launch Protocol & Supervision **[PARTIAL]**
*   *Ticket*: `006-connect-launch-protocol.md`

### 2.4 Tester-Readiness Onboarding & Node Discovery **[PLANNED]**
*   *Ticket*: `012-tester-readiness-onboarding.md`

### 2.5 Capability Broker & Secret Mediation **[PLANNED]**
*   *Ticket*: `016-capability-broker-secret-mediation.md`

### 2.6 Operator Surface Accessibility & Effective Config Clarity **[PLANNED]**
*   *Ticket*: `019-operator-accessibility-config-clarity.md`

---

## Phase 3: Deterministic Verification & Continuous Alignment
**Current Status: [PLANNED]**

### 3.1 The Deterministic Replay Engine (Day-Simulation Harness) **[PLANNED]**
*   *Ticket*: `007-day-simulation-harness.md`

### 3.2 LLM-as-a-Judge Evaluation Pipeline (Evals) **[PLANNED]**
*   *Ticket*: `008-llm-eval-pipeline.md`

### 3.3 Execution Tracing, Handoff Telemetry & Reviewability **[PLANNED]**
*   *Ticket*: `017-execution-tracing-reviewability.md`

### 3.4 Comprehensive User Documentation & Support Wiki **[PLANNED]**
*   *Ticket*: `013-user-documentation-architecture.md`

---

## Phase 4: Autonomous Swarm, Graph RAG & Zero-Trust Execution
**Current Status: [PLANNED]**

### 4.1 Semantic Memory & Graph RAG **[PLANNED]**
*   *Ticket*: `009-semantic-memory-rag.md`

### 4.2 Zero-Trust WASM Agent Sandboxing **[PLANNED]**
*   *Ticket*: `010-wasm-agent-sandboxing.md`

### 4.3 Swarm Execution SDK & Contract **[PLANNED]**
*   **Goal**: Provide a first-class SDK for external agents to communicate with Vel.
*   **Ticket**: `014-swarm-execution-sdk.md`

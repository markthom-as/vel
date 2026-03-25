# Vel: The Master Plan
**Status**: Canonical Truth (`0.3.0` shipped, `0.4.x`, `0.5`, `0.5.1`, and `0.5.2` closed)
**Last Updated**: 2026-03-25

---

## High-Level Status Summary

| Phase | Status | Focus |
| :--- | :--- | :--- |
| **v0.2: True MVP** | **[SHIPPED]** | Closed the strict operator loop, Rust-owned contracts, and thin-shell architecture. |
| **v0.3: Canonical Now Surface And Client Mesh** | **[SHIPPED]** | Closed the canonical compact Now contract, Rust-owned transport/core lane, and cross-platform parity baseline. |
| **Phase 1** | **[COMPLETE]** | Monolith decomposition and structural foundations. |
| **Phase 2** | **[CLOSED / RE-SCOPED]** | Distributed swarm baseline shipped; unfinished sync ordering, guided onboarding, and external connect work moved to Phases 5, 6, and 8. |
| **Phase 3** | **[COMPLETE]** | Deterministic verification, tracing, and reasoning eval closure. |
| **Phase 4** | **[CLOSED / RE-SCOPED]** | Semantic/broker/protocol baselines shipped; unfinished graph expansion, direct WASM guest runtime, and external SDK transport work moved to Phases 6 and 8. |
| **Phase 5** | **[COMPLETE]** | Local harness and operator runtime MVP (`vel run`, policy gates, artifacts, explainability). |

Milestones `v0.2` and `v0.3` are now archived in [`.planning/milestones/v0.2-ROADMAP.md`](/home/jove/code/vel/.planning/milestones/v0.2-ROADMAP.md) and [`.planning/milestones/v0.3-ROADMAP.md`](/home/jove/code/vel/.planning/milestones/v0.3-ROADMAP.md). `0.4.x`, `0.5`, `0.5.1`, and `0.5.2` are now closed. `0.5.2` completed the operator-surface embodiment line on top of the frozen `0.5` backend and truthful `0.5.1` client boundary:

- [`.planning/milestones/v0.5-core-rewrite/ROADMAP.md`](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/ROADMAP.md)
- [`.planning/milestones/v0.5-core-rewrite/65-MILESTONE-EVIDENCE.md`](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/65-MILESTONE-EVIDENCE.md)
- [`.planning/milestones/v0.5.1-client-reconnection/ROADMAP.md`](/home/jove/code/vel/.planning/milestones/v0.5.1-client-reconnection/ROADMAP.md)
- [`docs/cognitive-agent-architecture/architecture/0.5.1-truthful-surface-doctrine.md`](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/0.5.1-truthful-surface-doctrine.md)
- [`.planning/milestones/v0.5.1-ROADMAP.md`](/home/jove/code/vel/.planning/milestones/v0.5.1-ROADMAP.md)
- [`.planning/v0.5.1-MILESTONE-AUDIT.md`](/home/jove/code/vel/.planning/v0.5.1-MILESTONE-AUDIT.md)
- [`.planning/milestones/v0.5.2-ROADMAP.md`](/home/jove/code/vel/.planning/milestones/v0.5.2-ROADMAP.md)
- [`.planning/v0.5.2-MILESTONE-AUDIT.md`](/home/jove/code/vel/.planning/v0.5.2-MILESTONE-AUDIT.md)
- [`docs/cognitive-agent-architecture/architecture/0.5.2-operator-surface-doctrine.md`](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/0.5.2-operator-surface-doctrine.md)
- [`docs/cognitive-agent-architecture/architecture/0.5.2-apple-client-handoff.md`](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/0.5.2-apple-client-handoff.md)

Planning notation from this point forward:

- shipped releases use semver, for example `0.3.0`
- in-flight roadmap work stays inside a release line, for example `0.5.0-beta`
- execution slices may be referenced as `<major>.<minor>.<phase>.<plan>`, for example `0.5.57.1`
- the existing integer phase directories remain valid historical identifiers and should not be renumbered retroactively

Durable product authority for the shipped MVP starts with `docs/product/mvp-operator-loop.md`, while post-`v0.2` `Now` behavior is now governed by `docs/product/now-surface-canonical-contract.md`. Durable architecture authority continues in `docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md` and `docs/cognitive-agent-architecture/architecture/now-surface-rust-core-contract.md`.

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
**Current Status: [CLOSED / RE-SCOPED]**

Execution board: `docs/tickets/phase-2/parallel-execution-board.md`

Phase 2 shipped meaningful baseline work but did not fully close its original scope. The execution board and ticket files remain as historical implementation records only. Unfinished original-scope work is re-scoped into later phases so no active roadmap work remains before Phase 5.

### 2.1 Pluggable Signal Ingestion & Context Reducer Pipeline **[COMPLETE]**
*   *Ticket*: `004-signal-reducer-pipeline.md`
*   *Status*: complete — historical implementation lane only.

### 2.2 Sync Ordering & Conflict Resolution Baseline **[RE-SCOPED]**
*   *Ticket*: `005-hlc-sync-implementation.md`
*   *Status*: re-scoped — the live tree does not show the promised `NodeIdentity` / ordering primitive baseline. Deterministic multi-client reconciliation follow-on now belongs to Phase 6.

### 2.3 Agent Connect Launch Protocol & Supervision **[RE-SCOPED]**
*   *Ticket*: `006-connect-launch-protocol.md`
*   *Status*: re-scoped — live `/v1/connect` routes still return `deny_undefined_route`. External connect/auth transport and delegated-runtime launch closure now belong to Phase 8.

### 2.4 Tester-Readiness Onboarding & Node Discovery **[RE-SCOPED]**
*   *Ticket*: `012-tester-readiness-onboarding.md`
*   *Status*: re-scoped — guided linking (`vel node link`, pairing token flow, web/CLI wizard) is not present in the live tree. User-facing multi-client linking closure now belongs to Phase 5.

### 2.5 Capability Broker & Secret Mediation **[COMPLETE]**
*   *Ticket*: `016-capability-broker-secret-mediation.md`
*   *Status*: complete — historical implementation lane only.

### 2.6 Operator Surface Accessibility & Effective Config Clarity **[COMPLETE]**
*   *Ticket*: `019-operator-accessibility-config-clarity.md`
*   *Status*: complete — historical implementation lane only.

---

## Phase 3: Deterministic Verification & Continuous Alignment
**Current Status: [COMPLETE]**

Execution board: `docs/tickets/phase-3/parallel-execution-board.md`

Phase 3 is complete and closed. The execution board and ticket files remain as historical implementation records only. No remaining roadmap work exists before Phase 5.

### 3.1 The Deterministic Replay Engine (Day-Simulation Harness) **[COMPLETE]**
*   *Ticket*: `007-day-simulation-harness.md`

### 3.2 LLM-as-a-Judge Evaluation Pipeline (Evals) **[COMPLETE]**
*   *Ticket*: `008-llm-eval-pipeline.md`

### 3.3 Execution Tracing, Handoff Telemetry & Reviewability **[COMPLETE]**
*   *Ticket*: `017-execution-tracing-reviewability.md`

### 3.4 Comprehensive User Documentation & Support Wiki **[COMPLETE]**
*   *Ticket*: `013-user-documentation-architecture.md`

---

## Phase 4: Autonomous Swarm, Graph RAG & Zero-Trust Execution
**Current Status: [CLOSED / RE-SCOPED]**

Execution board: `docs/tickets/phase-4/parallel-execution-board.md`

Phase 4 shipped contract and runtime baselines, but did not fully close the original scope of graph memory, direct WASM guest execution, or external SDK/connect transport. The execution board and ticket files remain as historical implementation records only. That unfinished work is re-scoped into later phases so no active roadmap work remains before Phase 5.

### 4.1 Semantic Memory & Graph RAG **[RE-SCOPED]**
*   *Ticket*: `009-semantic-memory-rag.md`
*   *Status*: re-scoped — the live tree ships a capture-backed local semantic baseline with provenance, not full graph expansion. Richer entity/link indexing now belongs to Phase 6.

### 4.2 Zero-Trust WASM Agent Sandboxing **[RE-SCOPED]**
*   *Ticket*: `010-wasm-agent-sandboxing.md`
*   *Status*: re-scoped — the live tree ships a deny-by-default host executor over decoded ABI envelopes, not the direct WASM guest runtime promised by the ticket. That follow-on now belongs to Phase 8.

### 4.3 Swarm Execution SDK & Contract **[RE-SCOPED]**
*   **Goal**: Provide a first-class SDK for external agents to communicate with Vel.
*   **Ticket**: `014-swarm-execution-sdk.md`
*   *Status*: re-scoped — the Rust SDK/protocol baseline exists, but live external connect/auth transport is not exposed. External limb runtime closure now belongs to Phase 8.

---

## Phase 5: Local Harness & Operator Runtime MVP
**Current Status: [COMPLETE]**

Phase 5 opens the local-first harness lane for a single-operator runtime baseline before distributed execution expansion. The first slice is a v0.1 ticket pack that prioritizes canonical run/event truth, policy-gated execution, explainability surfaces, and security/observability hardening.

### 5.1 Canonical Run/Event and Storage Foundations **[COMPLETE]**
*   *Ticket*: `026-core-run-event-schema.md`
*   *Ticket*: `027-sqlite-run-store.md`
*   *Ticket*: `028-artifact-store.md`

### 5.2 Policy, Capability, and Tool Execution Boundaries **[COMPLETE]**
*   *Ticket*: `029-policy-config-loader.md`
*   *Ticket*: `030-capability-resolution-engine.md`
*   *Ticket*: `031-tool-runner-abstraction.md`
*   *Ticket*: `032-mutation-protocol-discipline.md`

### 5.3 Runtime Orchestration and Operator Surfaces **[COMPLETE]**
*   *Ticket*: `033-llm-provider-interface.md`
*   *Ticket*: `034-vel-run-command.md`
*   *Ticket*: `035-vel-dry-run-command.md`
*   *Ticket*: `036-explainability-history-commands.md`

### 5.4 Release-Critical Hardening **[COMPLETE]**
*   *Ticket*: `037-security-observability-hardening.md`

### 5.5 Morning Standup Overdue-Task Workflow **[PLANNED]**
*   *Ticket*: `038-standup-overdue-workflow-slice.md`

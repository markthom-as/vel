# Vel Ticket Queue

This file is the index for execution tickets under `docs/tickets/`.

Use it to find the right active or planned work item after you read [MASTER_PLAN.md](../MASTER_PLAN.md).

## Authority

- [MASTER_PLAN.md](../MASTER_PLAN.md) is the source of truth for shipped behavior, phase status, and queue shape.
- This file is the queue navigation aid for contributors and coding agents.
- Individual tickets define the intended implementation target for a bounded change.

## How To Use The Queue

1. Start with [MASTER_PLAN.md](../MASTER_PLAN.md).
2. Check [architecture-first-parallel-queue.md](architecture-first-parallel-queue.md) if the work touches docs, contracts, schemas, manifests, integrations, or self-awareness.
3. Pick the phase closest to the boundary you are changing.
4. Open the specific ticket before editing code or docs.
5. If you add a new ticket or change queue shape, update both this file and the Master Plan.

## Architecture-First Queue

Use [architecture-first-parallel-queue.md](architecture-first-parallel-queue.md) when the work spans architecture docs, schemas, contracts, manifests, or queue sequencing. That queue is the canonical “do these first” lane before broader implementation breadth.

## Phase 1: Structural Foundations

- [001-storage-modularization.md](phase-1/001-storage-modularization.md) `[in-progress]` Storage repository pattern and transaction lifecycles
- [002-typed-context-transition.md](phase-1/002-typed-context-transition.md) `[planned]` Pure-core and typed-context transition
- [003-service-dto-layering.md](phase-1/003-service-dto-layering.md) `[partial]` Service/DTO boundary and standardized error handling
- [011-documentation-truth-repair.md](phase-1/011-documentation-truth-repair.md) `[in-progress]` Documentation truth repair and architecture mapping
- [015-http-surface-auth-hardening.md](phase-1/015-http-surface-auth-hardening.md) `[planned]` Auth-by-default HTTP surfaces and deny-by-default routing
- [018-cross-cutting-system-traits-baseline.md](phase-1/018-cross-cutting-system-traits-baseline.md) `[planned]` Cross-cutting trait baseline and subsystem audit
- [020-documentation-catalog-single-source.md](phase-1/020-documentation-catalog-single-source.md) `[planned]` Single-source documentation catalog and surfaced-doc parity
- [021-canonical-schema-and-config-contracts.md](phase-1/021-canonical-schema-and-config-contracts.md) `[planned]` Canonical schema catalog, object definitions, and config templates
- [022-data-sources-and-connector-architecture.md](phase-1/022-data-sources-and-connector-architecture.md) `[planned]` Canonical data sources, integration families, and connector contracts
- [023-self-awareness-and-supervised-self-modification.md](phase-1/023-self-awareness-and-supervised-self-modification.md) `[planned]` Vel self-awareness, repo visibility, and supervised self-modification
- [024-machine-readable-schema-and-manifest-publication.md](phase-1/024-machine-readable-schema-and-manifest-publication.md) `[planned]` Publish machine-readable schema and manifest resources for shared contract surfaces
- [025-config-and-contract-fixture-parity.md](phase-1/025-config-and-contract-fixture-parity.md) `[planned]` Ensure template and fixture parity for config and contract artifacts used by tests and docs

## Phase 2: Distributed State, Offline Clients & System-Of-Systems

- [004-signal-reducer-pipeline.md](phase-2/004-signal-reducer-pipeline.md) `[planned]` Pluggable signal ingestion and context reducer pipeline
- [005-hlc-sync-implementation.md](phase-2/005-hlc-sync-implementation.md) `[planned]` Offline-first Apple clients and HLC synchronization
- [006-connect-launch-protocol.md](phase-2/006-connect-launch-protocol.md) `[partial]` Agent connect launch protocol and supervision
- [012-tester-readiness-onboarding.md](phase-2/012-tester-readiness-onboarding.md) `[planned]` Tester-readiness onboarding and node discovery
- [016-capability-broker-secret-mediation.md](phase-2/016-capability-broker-secret-mediation.md) `[planned]` Capability broker and secret mediation
- [019-operator-accessibility-config-clarity.md](phase-2/019-operator-accessibility-config-clarity.md) `[planned]` Operator-surface accessibility and effective-config clarity

## Phase 3: Deterministic Verification & Continuous Alignment

- [007-day-simulation-harness.md](phase-3/007-day-simulation-harness.md) `[planned]` Deterministic replay engine and day simulation harness
- [008-llm-eval-pipeline.md](phase-3/008-llm-eval-pipeline.md) `[planned]` LLM-as-a-judge evaluation pipeline
- [013-user-documentation-architecture.md](phase-3/013-user-documentation-architecture.md) `[planned]` Comprehensive user documentation and support wiki
- [017-execution-tracing-reviewability.md](phase-3/017-execution-tracing-reviewability.md) `[planned]` Execution tracing, handoff telemetry, and reviewability

## Phase 4: Autonomous Swarm, Graph RAG & Zero-Trust Execution

- [009-semantic-memory-rag.md](phase-4/009-semantic-memory-rag.md) `[planned]` Semantic memory and graph RAG
- [010-wasm-agent-sandboxing.md](phase-4/010-wasm-agent-sandboxing.md) `[planned]` Zero-trust WASM agent sandboxing
- [014-swarm-execution-sdk.md](phase-4/014-swarm-execution-sdk.md) `[planned]` Swarm execution SDK and contract

## Queue Maintenance Rules

- Prefer extending an existing ticket when the boundary is already clear.
- Add a new ticket only when the work introduces a new seam, abstraction, or milestone that deserves independent ownership.
- Keep ticket names concrete and boundary-oriented rather than aspirational.
- Close or merge stale ticket concepts instead of leaving parallel plans to drift.

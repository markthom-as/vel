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

For active lane ownership and non-overlapping write scopes, use the phase execution boards:
- [phase-2/parallel-execution-board.md](phase-2/parallel-execution-board.md)
- [phase-3/parallel-execution-board.md](phase-3/parallel-execution-board.md)
- [phase-4/parallel-execution-board.md](phase-4/parallel-execution-board.md)

## Phase 1: Structural Foundations

Execution evidence: [phase-1/phase-1-evidence-matrix.md](phase-1/phase-1-evidence-matrix.md)

- [001-storage-modularization.md](phase-1/001-storage-modularization.md) `[complete]` Storage repository pattern and transaction lifecycles
- [002-typed-context-transition.md](phase-1/002-typed-context-transition.md) `[complete]` Pure-core and typed-context transition
- [003-service-dto-layering.md](phase-1/003-service-dto-layering.md) `[complete]` Service/DTO boundary and standardized error handling
- [011-documentation-truth-repair.md](phase-1/011-documentation-truth-repair.md) `[complete]` Documentation truth repair and architecture mapping
- [015-http-surface-auth-hardening.md](phase-1/015-http-surface-auth-hardening.md) `[complete]` Auth-by-default HTTP surfaces and deny-by-default routing
- [018-cross-cutting-system-traits-baseline.md](phase-1/018-cross-cutting-system-traits-baseline.md) `[complete]` Cross-cutting trait baseline and subsystem audit
- [020-documentation-catalog-single-source.md](phase-1/020-documentation-catalog-single-source.md) `[complete]` Single-source documentation catalog and surfaced-doc parity
- [021-canonical-schema-and-config-contracts.md](phase-1/021-canonical-schema-and-config-contracts.md) `[complete]` Canonical schema catalog, object definitions, and config templates
- [022-data-sources-and-connector-architecture.md](phase-1/022-data-sources-and-connector-architecture.md) `[complete]` Canonical data sources, integration families, and connector contracts
- [023-self-awareness-and-supervised-self-modification.md](phase-1/023-self-awareness-and-supervised-self-modification.md) `[complete]` Vel self-awareness, repo visibility, and supervised self-modification
- [024-machine-readable-schema-and-manifest-publication.md](phase-1/024-machine-readable-schema-and-manifest-publication.md) `[complete]` Publish machine-readable schema and manifest resources for shared contract surfaces
- [025-config-and-contract-fixture-parity.md](phase-1/025-config-and-contract-fixture-parity.md) `[complete]` Ensure template and fixture parity for config and contract artifacts used by tests and docs

## Phase 2: Distributed State, Offline Clients & System-Of-Systems

Historical lane. Some unfinished original-scope work from this phase was re-scoped into active Phases 5, 6, and 8.

- [004-signal-reducer-pipeline.md](phase-2/004-signal-reducer-pipeline.md) `[baseline shipped]` Pluggable signal ingestion and context reducer pipeline
- [005-hlc-sync-implementation.md](phase-2/005-hlc-sync-implementation.md) `[re-scoped to Phase 6]` Sync ordering primitive and deterministic conflict resolution baseline
- [006-connect-launch-protocol.md](phase-2/006-connect-launch-protocol.md) `[re-scoped to Phase 8]` Agent connect launch protocol and supervision
- [012-tester-readiness-onboarding.md](phase-2/012-tester-readiness-onboarding.md) `[re-scoped to Phase 5]` Tester-readiness onboarding and node discovery closure
- [016-capability-broker-secret-mediation.md](phase-2/016-capability-broker-secret-mediation.md) `[baseline shipped]` Capability broker and secret mediation
- [019-operator-accessibility-config-clarity.md](phase-2/019-operator-accessibility-config-clarity.md) `[baseline shipped]` Operator-surface accessibility and effective-config clarity closure

## Phase 3: Deterministic Verification & Continuous Alignment

- [007-day-simulation-harness.md](phase-3/007-day-simulation-harness.md) `[complete]` Deterministic replay engine and day simulation harness
- [008-llm-eval-pipeline.md](phase-3/008-llm-eval-pipeline.md) `[complete]` LLM-as-a-judge evaluation pipeline
- [013-user-documentation-architecture.md](phase-3/013-user-documentation-architecture.md) `[complete]` Comprehensive user documentation and support wiki closure
- [017-execution-tracing-reviewability.md](phase-3/017-execution-tracing-reviewability.md) `[complete]` Execution tracing, handoff telemetry, and reviewability closure

## Phase 4: Autonomous Swarm, Graph RAG & Zero-Trust Execution

Historical lane. Some unfinished original-scope work from this phase was re-scoped into active Phases 6 and 8.

- [009-semantic-memory-rag.md](phase-4/009-semantic-memory-rag.md) `[re-scoped follow-on in Phase 6]` Semantic memory and graph RAG
- [010-wasm-agent-sandboxing.md](phase-4/010-wasm-agent-sandboxing.md) `[re-scoped follow-on in Phase 8]` Zero-trust WASM agent sandboxing
- [014-swarm-execution-sdk.md](phase-4/014-swarm-execution-sdk.md) `[re-scoped follow-on in Phase 8]` Swarm execution SDK and contract

## Phase 5: Local Harness & Operator Runtime MVP

Execution order and v0.1 release gate: [phase-5/README.md](phase-5/README.md)

- [026-core-run-event-schema.md](phase-5/026-core-run-event-schema.md) `[complete]` Canonical run and event schema for harness execution truth
- [027-sqlite-run-store.md](phase-5/027-sqlite-run-store.md) `[complete]` SQLite run store and migration baseline
- [028-artifact-store.md](phase-5/028-artifact-store.md) `[complete]` Filesystem artifact store with run linkage
- [029-policy-config-loader.md](phase-5/029-policy-config-loader.md) `[complete]` YAML/TOML policy loader with fail-closed validation
- [030-capability-resolution-engine.md](phase-5/030-capability-resolution-engine.md) `[complete]` Capability resolver with persisted policy decisions
- [031-tool-runner-abstraction.md](phase-5/031-tool-runner-abstraction.md) `[complete]` Structured tool runner abstraction with lifecycle events
- [032-mutation-protocol-discipline.md](phase-5/032-mutation-protocol-discipline.md) `[complete]` Mutation proposal/confirmation/commit discipline
- [033-llm-provider-interface.md](phase-5/033-llm-provider-interface.md) `[complete]` Provider-neutral LLM synthesis contract
- [034-vel-run-command.md](phase-5/034-vel-run-command.md) `[complete]` End-to-end `vel run` MVP command path
- [035-vel-dry-run-command.md](phase-5/035-vel-dry-run-command.md) `[complete]` `vel dry-run` policy and mutation preview mode
- [036-explainability-history-commands.md](phase-5/036-explainability-history-commands.md) `[complete]` `vel explain`, `vel runs`, and `vel artifacts` trust surfaces
- [037-security-observability-hardening.md](phase-5/037-security-observability-hardening.md) `[complete]` v0.1 hardening for redaction, write scope, and observability
- [038-standup-overdue-workflow-slice.md](phase-5/038-standup-overdue-workflow-slice.md) `[planned]` Morning standup overdue-task action workflow (`menu -> confirm -> apply -> undo`)

## Queue Maintenance Rules

- Prefer extending an existing ticket when the boundary is already clear.
- Add a new ticket only when the work introduces a new seam, abstraction, or milestone that deserves independent ownership.
- Keep ticket names concrete and boundary-oriented rather than aspirational.
- Close or merge stale ticket concepts instead of leaving parallel plans to drift.

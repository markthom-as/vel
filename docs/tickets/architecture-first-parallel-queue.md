# Architecture-First Parallel Queue

This file is the canonical sequencing guide for architecture, documentation, schema, and contract work that should happen before broad implementation expansion.

Use this queue when a task spans multiple layers or when the repo needs clearer contracts before code should spread further.

## Operating Rule

Documentation, schema, contract, template, and architecture work should lead implementation work, not trail behind it.

That means:

- define the contract first
- define the canonical examples or templates second
- wire queue coverage third
- widen implementation only after the boundary is clear

## Wave 0: Authority And Documentation Repair

These can run in parallel:

- [011-documentation-truth-repair.md](phase-1/011-documentation-truth-repair.md)
- [018-cross-cutting-system-traits-baseline.md](phase-1/018-cross-cutting-system-traits-baseline.md)
- [020-documentation-catalog-single-source.md](phase-1/020-documentation-catalog-single-source.md)

Primary outcome:

- one coherent authority chain and one coherent documentation surface

## Wave 1: Canonical Contracts And Schemas

These can run in parallel after Wave 0 is stable enough for contributors to navigate reliably:

- [021-canonical-schema-and-config-contracts.md](phase-1/021-canonical-schema-and-config-contracts.md)
- [022-data-sources-and-connector-architecture.md](phase-1/022-data-sources-and-connector-architecture.md)
- [023-self-awareness-and-supervised-self-modification.md](phase-1/023-self-awareness-and-supervised-self-modification.md)
- [024-machine-readable-schema-and-manifest-publication.md](phase-1/024-machine-readable-schema-and-manifest-publication.md)
- [025-config-and-contract-fixture-parity.md](phase-1/025-config-and-contract-fixture-parity.md)

Primary outcome:

- explicit object definitions, config contracts, policy schemas, connector contracts, and self-model boundaries
- machine-readable schemas and manifests with shared publication semantics
- parseable templates and canonical fixtures that stay in sync with contract docs

## Wave 2: Core Structural Hardening

These can run in parallel once the contract layer exists:

- [002-typed-context-transition.md](phase-1/002-typed-context-transition.md)
- [003-service-dto-layering.md](phase-1/003-service-dto-layering.md)
- [015-http-surface-auth-hardening.md](phase-1/015-http-surface-auth-hardening.md)

Primary outcome:

- cleaner code boundaries enforced against the documented contracts

## Wave 3: Distributed And Capability Foundations

These can run in parallel after core contracts and boundaries are in place:

- [004-signal-reducer-pipeline.md](phase-2/004-signal-reducer-pipeline.md)
- [016-capability-broker-secret-mediation.md](phase-2/016-capability-broker-secret-mediation.md)
- [006-connect-launch-protocol.md](phase-2/006-connect-launch-protocol.md)

Primary outcome:

- distributed execution and external capability work built on stable contracts rather than implicit assumptions

## Wave 4: Reviewability And Deterministic Confidence

These can run in parallel after the earlier waves have produced stable boundaries:

- [017-execution-tracing-reviewability.md](phase-3/017-execution-tracing-reviewability.md)
- [007-day-simulation-harness.md](phase-3/007-day-simulation-harness.md)
- [008-llm-eval-pipeline.md](phase-3/008-llm-eval-pipeline.md)

Primary outcome:

- trustworthy verification, replay, and evaluation for the resulting system

## Gating Questions

Before pulling work from a later wave, ask:

1. Is the contract for this work already explicit and canonical?
2. Is the writable scope and safety boundary already documented?
3. Can another contributor understand the seam without reverse-engineering code drift?
4. Is the earlier-wave gap small enough to defer, or will deferring it multiply confusion?

If the answer to those questions is mostly “no”, stay in an earlier wave.

---
title: Swarm Execution SDK & Contract
status: planned
owner: staff-eng
type: architecture
priority: medium
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 006-connect-launch-protocol
  - 010-wasm-agent-sandboxing
  - 016-capability-broker-secret-mediation
  - 017-execution-tracing-reviewability
  - 022-data-sources-and-connector-architecture
  - 023-self-awareness-and-supervised-self-modification
  - 024-machine-readable-schema-and-manifest-publication
  - 025-config-and-contract-fixture-parity
labels:
  - vel-core
  - distributed
  - agent-sdk
  - phase-4
---

# Context & Objectives

Connect and delegated-execution foundations are evolving, but Vel does not yet ship a stable external SDK/protocol boundary for third-party limbs.

This ticket defines the protocol contracts and reference SDKs required for safe, traceable external agent participation.

# Impacted Files & Symbols

- **Crate (new)**: `vel-protocol`
  - **Symbols**: protocol envelopes, manifest contracts, message serialization
- **Crate (new or expanded)**: `vel-agent-sdk`
  - **Symbols**: heartbeat loop, capability negotiation, request helpers
- **Crate**: `veld`
  - **Symbols**: connect protocol handling and lease supervision integration
- **Docs**: protocol, SDK usage, and capability/trace expectations

# Technical Requirements

- **Protocol Contract**: explicit message envelopes and versioned compatibility rules.
- **Transport**: support selected runtime-safe transport(s) with clear auth semantics.
- **Lease + Heartbeat**: SDK handles lease lifecycle and heartbeat renewal.
- **Capability Negotiation**: requested capabilities declared up front and scoped grants returned.
- **Trace Linkage**: SDK requests/responses carry stable run/trace identifiers.

# Cross-Cutting Trait Impact

- **Modularity**: required — protocol and SDK boundaries should be independent of app routes.
- **Accessibility**: affected — developer-facing SDK errors and docs should be clear.
- **Configurability**: required — transport, auth, capability defaults, and retry policies explicit.
- **Data Logging**: required — protocol operations are traceable.
- **Rewind/Replay**: affected — protocol exchanges should be reproducible in tests/fixtures.
- **Composability**: required — SDK composes with connect, broker, and sandbox contracts.

# Implementation Steps (The How)

1. **Protocol design**: define manifest/envelope schema and versioning rules.
2. **Shared crate**: implement `vel-protocol` serialization/validation core.
3. **Reference SDK**: implement Rust and TypeScript SDK paths.
4. **Runtime integration**: wire protocol contract into connect lifecycle.

# Acceptance Criteria

1. [ ] External limb process can authenticate, connect, and maintain lease heartbeats.
2. [ ] SDK can request scoped actions and submit action batches through protocol contract.
3. [ ] Protocol serialization/validation is unit-tested with versioned fixtures.
4. [ ] SDK usage never requires broad provider credential exposure for mediated actions.

# Verification & Regression

- **Unit Test**: protocol envelope serialization, validation, and version checks.
- **Integration Test**: SDK-to-runtime handshake, heartbeat, and scoped action flow.
- **Smoke Check**: run reference SDK sample against local authority node.
- **Invariants**: protocol-level capability claims cannot bypass runtime policy mediation.

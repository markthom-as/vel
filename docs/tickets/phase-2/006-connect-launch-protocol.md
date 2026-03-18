---
title: Connect: Agent Launch Protocol & Supervision
status: in-progress
owner: staff-eng
type: architecture
priority: medium
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 022-data-sources-and-connector-architecture
  - 023-self-awareness-and-supervised-self-modification
  - 024-machine-readable-schema-and-manifest-publication
labels:
  - veld
  - agentic
  - connect
  - phase-2
---

# Context & Objectives

Connect runtime foundations exist (types and worker routing primitives), but runtime routes remain deny-by-default and end-to-end launch supervision is not shipped.

This ticket ships Connect in supervised slices with explicit capability scopes, denial paths, and traceability.

**Status: in-progress (shell only)**

# Current Baseline

The following infrastructure exists but does not yet implement the full protocol:

- **Heartbeat infra**: heartbeat-related structures and registration paths exist in `crates/veld/src/services/client_sync.rs`.
- **Route reservations**: `/v1/connect` and `/v1/connect/*path` routes exist in `crates/veld/src/app.rs` but currently call `deny_undefined_route`, returning 403 for all requests.
- **CLI stubs**: connect subcommands exist in `crates/vel-cli/src/commands/connect.rs` but do not call active backend routes.

All four acceptance criteria are unmet. This ticket is in-progress at the shell/stub level only.

# Impacted Files & Symbols

- **File**: `crates/veld/src/app.rs`
  - **Symbols**: connect route registration and auth policy
- **Directory**: `crates/veld/src/routes/`
  - **Symbols**: connect route handlers
- **File**: `crates/veld/src/services/client_sync.rs`
  - **Symbols**: placement/capability routing for launch targets
- **Crate**: `vel-api-types`
  - **Symbols**: connect transport DTOs

# Technical Requirements

- **Protocol Messages**: support launch, heartbeat, status update, and termination semantics.
- **Lease Supervision**: launched runtimes require heartbeat to keep execution lease.
- **Scoped Capabilities**: launch requests use explicit allowlists; no ambient full authority.
- **Trace Linkage**: launch/heartbeat/denial/termination emit stable run or trace IDs.
- **Isolation**: delegated execution runs in sandboxed or isolated worker envelopes where available.

# Cross-Cutting Trait Impact

- **Modularity**: required — route handlers thin, service logic centralized.
- **Accessibility**: affected — operator surfaces need readable connect state.
- **Configurability**: required — capability scopes and connect policy must be inspectable.
- **Data Logging**: required — lifecycle event trail is mandatory.
- **Rewind/Replay**: affected — run history should reconstruct launch outcomes.
- **Composability**: required — connect should compose with capability broker ticket (`016`).

# Implementation Steps (The How)

1. **MVP route activation**: replace blanket `403` reservation with authenticated connect endpoints.
2. **Lifecycle plumbing**: implement launch/heartbeat/terminate flow with lease expiry.
3. **Capability enforcement**: enforce scoped launch allowlists and denial records.
4. **Operator visibility**: expose connect status in inspectable API/CLI outputs.

# Acceptance Criteria

1. [ ] Authority can launch and supervise delegated runtime instances through authenticated connect routes.
2. [ ] Missing heartbeat causes lease expiry and terminal run state.
3. [ ] Launches enforce explicit scoped capabilities and reject out-of-scope requests.
4. [ ] Connect lifecycle transitions are persisted and traceable.

# Verification & Regression

- **Unit Test**: lease and capability-denial logic.
- **Integration Test**: launch, heartbeat, expiry, and termination flow.
- **Smoke Check**: CLI connect commands against local daemon.
- **Invariants**: no delegated runtime can self-escalate permissions.

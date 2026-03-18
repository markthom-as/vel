---
title: Capability Broker & Secret Mediation
status: planned
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 015-http-surface-auth-hardening
  - 006-connect-launch-protocol
  - 022-data-sources-and-connector-architecture
  - 024-machine-readable-schema-and-manifest-publication
labels:
  - veld
  - connect
  - integrations
  - security
  - phase-2
---

# Context & Objectives

Vel already separates public integration settings from secret values in core integration flows, but delegated runtime execution still lacks a first-class broker boundary for scoped external actions.

This ticket introduces that broker boundary so agent requests are mediated by explicit capability scope checks and point-of-use secret injection.

## Scope Decision Record

**Scope: agents-only.** Integration-level capability brokering (integrations delegating to broker) is deferred to a later milestone per 2026-03-18 decision. The broker boundary applies exclusively to delegated agent runtimes launched via the connect protocol (ticket 006). Direct integration paths remain outside broker mediation until a future milestone explicitly expands scope.

# Impacted Files & Symbols

- **Directory**: `crates/veld/src/services/`
  - **Symbols**: broker service, integration execution mediation
- **Directory**: `crates/veld/src/routes/`
  - **Symbols**: capability/connect request entrypoints
- **Crate**: `vel-core`
  - **Symbols**: capability descriptors, grants, denials
- **Docs**: `docs/cognitive-agent-architecture/agents/tool-access.md`
  - **Symbols**: capability boundary policy language

# Technical Requirements

- **Brokered Access**: execute through brokered capabilities instead of raw provider credentials in prompts.
- **Scope Model**: support scope at tool/action/resource boundary (host/path or equivalent).
- **Fail Closed**: unmatched external requests deny by default.
- **Secret Hygiene**: decrypt only at point-of-use and never log decrypted values.
- **Auditability**: grants/denials/executions emit traceable run events.

# Cross-Cutting Trait Impact

- **Modularity**: required — mediation boundary isolates policy from route handlers.
- **Accessibility**: affected — denial reasons should be operator-readable.
- **Configurability**: required — capability policy must be inspectable.
- **Data Logging**: required — denial/grant traces are mandatory.
- **Rewind/Replay**: affected — capability decisions should be reconstructable.
- **Composability**: required — broker composes with connect launch supervision.

# Implementation Steps (The How)

1. **Capability model**: define descriptors and matcher semantics.
2. **Broker service**: implement mediation layer between intent and execution.
3. **Secret boundary**: resolve and inject secrets only inside broker execution path.
4. **Trace emission**: persist grant/deny/execution events with stable IDs.

# Acceptance Criteria

1. [ ] Delegated runtimes can perform approved actions without receiving raw provider credentials.
2. [ ] Capability checks enforce scoped access and deny out-of-scope requests.
3. [ ] Denials are fail-closed and inspectable.
4. [ ] No decrypted secret appears in prompts, traces, logs, or response DTOs.

# Verification & Regression

- **Unit Test**: scope matching and denial cases.
- **Integration Test**: approved path, denied path, and secret-leak regression cases.
- **Smoke Check**: delegated action through scoped broker capability.
- **Invariants**: provider master credentials never leave broker-only execution path.

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

Vel is moving toward delegated workers, external runtimes, and broader integration support. The queue currently lacks a first-class ticket for secret mediation and scoped capability access.

This ticket introduces a capability broker model so agents and worker runtimes can perform useful external actions without receiving prompt-visible raw credentials or ambient network authority.

# Impacted Files & Symbols

- **Directory**: `crates/veld/src/services/`
  - **Symbols**: integration services, connect services, future broker services
- **Directory**: `crates/veld/src/routes/`
  - **Symbols**: future capability or connect endpoints
- **Crate**: `vel-core`
  - **Symbols**: capability descriptors, scoped grants, deny records
- **Docs**: `docs/cognitive-agent-architecture/agents/tool-access.md`
  - **Symbols**: capability boundary rules

# Technical Requirements

- **Brokered Access**: Prefer brokered actions, scoped tokens, or point-of-use injection over handing raw provider secrets to agents.
- **Scope Model**: Capabilities must be scoped by tool, action, host, path, or resource where possible.
- **Fail Closed**: Unknown or unmatched external-access requests must reject safely.
- **Secret Hygiene**: Decrypt secrets only at the narrowest point of use; never log or return decrypted values.
- **Auditability**: Capability grants, denials, and executions should produce run events or equivalent traces.

# Implementation Steps (The "How")

1. **Capability Model**: Define capability descriptors and scope semantics.
2. **Broker Layer**: Introduce a mediation service between agent intent and external execution.
3. **Secret Boundary**: Move secret resolution and point-of-use injection behind the broker boundary.
4. **Tracing**: Emit grant, denial, and execution events with stable IDs.

# Acceptance Criteria

1. [ ] Delegated agents can perform approved external actions without receiving raw provider credentials.
2. [ ] Capability checks can scope access by at least host/path or equivalent resource boundary.
3. [ ] Rejected external requests fail closed and leave an inspectable denial record.
4. [ ] No raw decrypted secret appears in prompts, traces, logs, or returned payloads.

# Verification & Regression

- **Unit Test**: capability matching and denial cases
- **Integration Test**: approved action, denied action, and secret-leak regression cases
- **Smoke Check**: manual execution through a delegated path using a scoped capability
- **Invariants**: agents never receive provider master credentials as ordinary payload data

# Agent Guardrails

- **No Secret Shortcuts**: Do not move secrets into prompts, fixtures, or client-visible DTOs.
- **Scoped First**: Prefer the narrowest practical capability scope.
- **Honest Failure**: If a request is out of scope, deny it clearly rather than degrading into a broad allow.

---
title: Security and Observability Hardening for v0.1 Harness
status: complete
owner: staff-eng
type: hardening
priority: high
created: 2026-03-24
updated: 2026-03-25
depends_on:
  - 029-policy-config-loader
  - 031-tool-runner-abstraction
  - 032-mutation-protocol-discipline
  - 034-vel-run-command
  - 036-explainability-history-commands
labels:
  - security
  - observability
  - redaction
  - phase-5
---

# Context & Objectives

Close v0.1 release-critical guardrails: secret redaction, bounded write scope enforcement, and high-value execution observability for tool/mutation/run boundaries.

# Impacted Files & Symbols

- **Crate**: `crates/veld`
  - **Symbols**: logging/tracing boundaries, redaction middleware, write-scope checks
- **Crate**: `crates/vel-cli`
  - **Symbols**: safe output rendering and warning surfaces
- **Docs/Config**: `config/` and security-adjacent docs
  - **Symbols**: policy examples and redaction expectations

# Technical Requirements

- Redact secret-like values from logs/events/operator output.
- Enforce explicit write scopes for mutation commits.
- Emit traceable lifecycle events for run start/end, tool invocation, policy decisions, and mutation commits.
- Preserve explainability while preventing sensitive payload leakage.

# Implementation Steps (The How)

1. Add redaction pass for logs/event payloads and CLI rendering.
2. Enforce write-scope boundary checks in commit path.
3. Add structured observability events at key runtime boundaries.
4. Add hardening regression tests for secret leakage and boundary bypass.

# Acceptance Criteria

1. [x] Secret-bearing environment or credential fields are redacted in logs and output.
2. [x] Out-of-scope mutation targets are blocked and audited.
3. [x] Run/tool/policy/mutation lifecycle events are queryable by run id.
4. [x] Hardened paths preserve operator-readable failure reasons.

# Verification & Regression

- **Security Test**: leak regression suite for secret-shaped values.
- **Integration Test**: write-scope violation is denied and logged.
- **Manual Smoke**: inspect run event trail for complete lifecycle observability.

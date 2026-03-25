---
title: Policy Config Loader and Fail-Closed Validation
status: complete
owner: staff-eng
type: architecture
priority: high
created: 2026-03-24
updated: 2026-03-25
labels:
  - policy
  - config
  - security
  - phase-5
---

# Context & Objectives

Define v0.1 harness policy configuration for confirmation gates, tool classes, write scopes, and mutation defaults using YAML/TOML parsing with fail-closed validation.

# Impacted Files & Symbols

- **Directory**: `config/`
  - **Symbols**: policy templates, examples, schema/manifest entries
- **Crate**: `crates/vel-core`
  - **Symbols**: policy config model types
- **Crate**: `crates/veld`
  - **Symbols**: startup policy loading and validation

# Technical Requirements

- Support YAML/TOML policy input with one canonical typed model.
- Validation must deny unknown/unsupported capabilities by default.
- Include config template/example and machine-readable schema once boundary is stable.
- Emit operator-readable validation diagnostics.

# Implementation Steps (The How)

1. Define typed policy model and parser entrypoints.
2. Add strict validation with deny-by-default fallback.
3. Add canonical template/example in `config/` and schema linkage.
4. Wire loader into runtime startup and CLI policy checks.

# Acceptance Criteria

1. [x] Valid policy files load into typed config at startup.
2. [x] Invalid policy files fail startup/policy-check with actionable diagnostics.
3. [x] Unknown capability/tool entries are denied by default.
4. [x] Template/example + schema/manifest entries are shipped together.

# Verification & Regression

- **Unit Test**: parse valid YAML and TOML policy fixtures.
- **Unit Test**: invalid/unknown fields fail with deterministic errors.
- **CLI Smoke**: `vel policy check` returns pass/fail with reason detail.

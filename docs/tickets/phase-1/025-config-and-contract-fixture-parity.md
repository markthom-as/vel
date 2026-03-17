---
title: Config And Contract Fixture Parity
status: planned
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 021-canonical-schema-and-config-contracts
  - 024-machine-readable-schema-and-manifest-publication
labels:
  - templates
  - fixtures
  - testing
  - phase-1
---

# Context & Objectives

Vel contract docs now define templates and schema ownership, but test coverage still leans on inline config snippets and scattered examples.

This ticket makes checked-in templates and fixtures first-class artifacts consumed by tests and verification so docs, examples, and loaders stay synchronized.

# Impacted Files & Symbols

- **Config Surfaces**: runtime config, policy config, agent specs, model profiles, routing config
  - **Symbols**: parseable templates and canonical fixture examples
- **Docs**: `docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md`
  - **Symbols**: template and fixture parity rules
- **Tests**: loader/parser tests in owning crates
  - **Symbols**: parsing canonical templates and fixtures instead of ad hoc inline-only samples

# Technical Requirements

- **Template Coverage**: every human-authored config contract has a checked-in template.
- **Fixture Coverage**: every major config/manifest contract has at least one canonical fixture used by tests.
- **Parse Validation**: templates and fixtures must parse in automated checks.
- **Parity Rule**: when contract fields change, templates, fixtures, and parser tests update in the same change.
- **Model Config Inclusion**: model profile and routing contracts must follow the same parity rules as runtime/policy/agent configs.

# Cross-Cutting Trait Impact
- **Modularity**: required — templates and fixtures should be owned at contract boundaries.
- **Accessibility**: required — contributors need discoverable, runnable examples.
- **Configurability**: required — effective defaults and optional fields stay explicit.
- **Data Logging**: affected — fixtures for event and manifest contracts should carry realistic metadata shapes.
- **Rewind/Replay**: affected — fixture versions should remain aligned with schema versioning rules.
- **Composability**: required — shared fixtures reduce drift across runtime, clients, and scripts.

# Implementation Steps (The "How")

1. Enumerate config and contract surfaces that require templates and fixtures.
2. Add missing checked-in templates and canonical fixtures.
3. Update parser/loader tests to consume canonical artifacts.
4. Enforce parity rules in docs and queue references.

# Acceptance Criteria

1. [ ] Canonical templates exist for core config surfaces, including model profile and routing config.
2. [ ] Canonical fixtures exist for core config and manifest contracts and are used in tests.
3. [ ] Parser/loader tests validate checked-in templates and fixtures.
4. [ ] Contract changes that skip template/fixture updates are treated as incomplete.

# Verification & Regression

- **Rust Test**: parser/loader tests for config and contract artifacts.
- **Repo Check**: canonical templates and fixtures are discoverable from contract docs and queue references.
- **Invariants**: no major config or contract surface remains example-less or test-uncovered.

# Agent Guardrails

- **No Inline-Only Contracts**: do not rely solely on inline test strings for shared contract examples.
- **Keep Fixtures Realistic**: fixtures should model real-world values and boundary cases.
- **One Update Set**: field additions/removals must update docs, templates, fixtures, and tests together.

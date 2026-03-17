---
title: Canonical Schema And Config Contracts
status: complete
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 018-cross-cutting-system-traits-baseline
  - 020-documentation-catalog-single-source
labels:
  - schemas
  - config
  - contracts
  - phase-1
---

# Context & Objectives

Vel now has real schema-bearing objects across runtime config, policy config, agent specs, model profiles, integration records, handoff envelopes, and typed-context targets, but the ownership and examples are still fragmented.

This ticket makes the contract layer explicit and keeps templates in-repo so docs, code, and configuration have one shared reference point.

# Impacted Files & Symbols

- **Docs**: `docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md`
  - **Symbols**: contract inventory, versioning rules, template references
- **Config**: `vel.toml`, `config/agent-specs.yaml`, `config/policies.yaml`
  - **Symbols**: `AppConfig`, `AgentSpec`, `PolicyConfig`
- **Crates**: `crates/vel-config`, `crates/veld/src/policy_config.rs`, `crates/vel-core/src/integration.rs`
  - **Symbols**: typed config loaders, validators, integration records

# Technical Requirements

- **Contract Catalog**: Document the owner, boundary, and versioning rule for each major schema-bearing object.
- **Template Coverage**: Keep checked-in templates for primary human-authored config surfaces.
- **Validation First**: Templates should parse successfully in automated checks.
- **Schema-on-Write**: Replay-sensitive state objects should move toward typed, versioned structs instead of unconstrained JSON blobs.
- **No Shadow Schemas**: Do not let docs, clients, and code invent parallel meanings for the same object.

# Cross-Cutting Trait Impact
- **Modularity**: required — each contract needs a named owner and boundary.
- **Accessibility**: required — contributors need discoverable schema docs and examples.
- **Configurability**: required — config schemas and defaults must be explicit and inspectable.
- **Data Logging**: affected — manifests and envelopes should support stable trace/evidence fields.
- **Rewind/Replay**: required — stateful schemas need versioning and migration discipline.
- **Composability**: required — schemas should compose across backend, CLI, web, Apple, and delegated runtimes.

# Implementation Steps (The "How")

1. **Catalog**: Enumerate the current major contract-bearing objects and files.
2. **Template**: Add valid checked-in templates for runtime config, agent specs, and policy config.
3. **Validation**: Add automated checks that parse the templates successfully.
4. **Queue Sync**: Reference the schema catalog from affected tickets and docs instead of duplicating contract descriptions ad hoc.
5. **Publication Handoff**: Route machine-readable contract publication to `024-machine-readable-schema-and-manifest-publication.md`.
6. **Fixture Handoff**: Route template/fixture parity enforcement to `025-config-and-contract-fixture-parity.md`.

# Acceptance Criteria

1. [ ] Canonical contract docs exist for runtime config, policy config, agent specs, and major manifests.
2. [ ] Checked-in config templates parse successfully in tests.
3. [ ] Contributors can identify schema ownership and boundary rules without reverse-engineering code.
4. [ ] Later tickets can reference the schema catalog instead of restating object definitions.

# Verification & Regression

- **Rust Test**: `cargo test -p vel-config`
- **Rust Test**: `cargo test -p veld policy_config`
- **Repo Check**: `node scripts/verify-repo-truth.mjs`
- **Invariants**: template files must stay valid and must not become shadow runtime truth

# Agent Guardrails

- **No Codegen Rush**: Do not introduce external schema tooling before ownership and boundaries are clear.
- **Typed Where It Matters**: Use typed contracts for replay-sensitive and shared state.
- **Examples Must Be Valid**: Checked-in templates should load cleanly.

---
title: Machine-Readable Schema And Manifest Publication
status: complete
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 021-canonical-schema-and-config-contracts
  - 022-data-sources-and-connector-architecture
  - 023-self-awareness-and-supervised-self-modification
labels:
  - schemas
  - manifests
  - contracts
  - phase-1
---

# Context & Objectives

Vel now has canonical prose for schemas, connector contracts, and self-awareness envelopes, but publication is still mostly human-readable markdown.

This ticket adds machine-readable schema and manifest publication so CLI, web, Apple, scripts, and verification tooling can consume one contract source instead of re-deriving shape rules per surface.

# Impacted Files & Symbols

- **Docs**: `docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md`
  - **Symbols**: schema ownership and publication rules
- **Docs**: `docs/cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md`
  - **Symbols**: connector manifest contract
- **Docs**: `docs/cognitive-agent-architecture/cognition/self-awareness-and-supervised-self-modification.md`
  - **Symbols**: self-model envelope contract
- **Future Artifacts**: canonical schema/manifest files under `config/schemas/` or equivalent
  - **Symbols**: config schemas, connector manifests, handoff/self-model envelopes

# Technical Requirements

- **Schema Publication**: Publish machine-readable schemas for runtime config, policy config, agent specs, model profiles, routing config, connector manifests, and self-model envelopes.
- **Manifest Registry**: Provide one discoverable contract manifest that lists the published schema resources and versioning metadata.
- **Versioning Rule**: Breaking schema changes require explicit version increments and migration notes.
- **Consumer Discipline**: CLI, web, Apple, and scripts should consume published schema artifacts rather than bespoke inline assumptions.
- **Traceability**: Schema publication must leave inspectable links between authority docs, schema resources, and ticketed changes.

# Cross-Cutting Trait Impact
- **Modularity**: required — shared schema publication prevents per-surface contract forks.
- **Accessibility**: required — machine-readable resources make contract lookup and validation tooling straightforward.
- **Configurability**: required — config defaults and optional fields become inspectable and enforceable.
- **Data Logging**: affected — manifests should expose schema versions used by runs and artifacts when relevant.
- **Rewind/Replay**: required — replay-sensitive schema versions need deterministic identification.
- **Composability**: required — multiple clients and runtimes can compose around the same published contracts.

# Implementation Steps (The "How")

1. Define the contract publication format and registry.
2. Publish the first schema set for the highest-value config and envelope surfaces.
3. Wire consuming surfaces to published resources.
4. Add doc and queue references so future schema work extends this system instead of bypassing it.

# Acceptance Criteria

1. [x] A machine-readable schema/manifest publication mechanism exists and is documented.
2. [x] Published artifacts cover core config and envelope contracts.
3. [x] Consumers can discover schema resources from one canonical registry.
4. [x] Versioning and migration rules are explicit for breaking changes.

# Verification & Regression

- **Repo Check**: schema/manifest files are discoverable from one canonical registry.
- **Surface Check**: at least one runtime and one client/tooling surface consume published contract artifacts.
- **Invariants**: no new major contract surface is added without publication coverage.

# Agent Guardrails

- **No Parallel Truth**: do not publish a schema that disagrees with the owning code contract.
- **Version Explicitly**: breaking changes must never ship as silent in-place edits.
- **Keep It Consumable**: prioritize stable machine-readable output over ad hoc generated text dumps.

---
title: Canonical Data Sources And Connector Architecture
status: planned
owner: staff-eng
type: architecture
priority: high
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 021-canonical-schema-and-config-contracts
  - 016-capability-broker-secret-mediation
labels:
  - integrations
  - connectors
  - data-sources
  - phase-1
---

# Context & Objectives

Vel has real shipped local-source and credential-backed integrations plus planned capability-broker and plugin work, but there is no one canonical connector contract that every current and future integration follows.

This ticket defines and operationalizes that contract so new integrations extend one model instead of inventing new per-provider rules.

# Impacted Files & Symbols

- **Docs**: `docs/cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md`
  - **Symbols**: family list, source modes, connector contract
- **Docs**: `docs/cognitive-agent-architecture/integrations/data-source-catalog.md`
  - **Symbols**: concrete provider inventory, source mode mapping, rollout status
- **Crate**: `crates/vel-core/src/integration.rs`
  - **Symbols**: `IntegrationFamily`, `IntegrationProvider`, `IntegrationConnection`, `IntegrationSourceRef`
- **Services**: `crates/veld/src/services/integrations*.rs`
  - **Symbols**: sync entrypoints, local-source mapping, runtime status

# Technical Requirements

- **Canonical Family List**: Maintain one stable list of data-source families.
- **Source Modes**: Every connector must declare how it gets data: file, directory, snapshot, OAuth API, brokered tool, or delegated runtime.
- **Manifest Shape**: Connectors must declare capabilities, secret mode, freshness semantics, provenance, and write permissions.
- **Provenance**: Connector outputs must attach stable source references where applicable.
- **Trust Discipline**: Local-first source modes remain preferred for core loops; remote/brokered modes must stay scoped and inspectable.

# Cross-Cutting Trait Impact
- **Modularity**: required — connector implementations should conform to one family/provider contract.
- **Accessibility**: required — operators need clear visibility into source mode, freshness, and status.
- **Configurability**: required — source paths, provider settings, and participation controls must be explicit.
- **Data Logging**: required — sync, auth, and failure events need structured evidence.
- **Rewind/Replay**: affected — connector outputs need replay-safe provenance and time semantics.
- **Composability**: required — connectors should compose through shared manifests and source refs.

# Implementation Steps (The "How")

1. **Catalog**: Freeze the canonical family and source-mode list.
2. **Manifest**: Define the connector contract and example manifest.
3. **Inventory**: Keep the concrete data source catalog aligned with shipped and planned providers.
4. **Mapping**: Align current shipped integrations to the canonical family/source-mode model.
5. **Queue Sync**: Point future integration tickets and docs to the connector contract.

# Acceptance Criteria

1. [ ] The repo has one canonical list of integration families and source modes.
2. [ ] New integration work references one connector contract instead of inventing new shape.
3. [ ] Provenance and connector health expectations are explicit.
4. [ ] Ticket coverage exists for later code alignment work.

# Verification & Regression

- **Doc Check**: integration docs and user docs agree on the family list and source modes
- **Repo Check**: `node scripts/verify-repo-truth.mjs`
- **Invariants**: no new integration family or connector mode appears in code/docs without canonical documentation

# Agent Guardrails

- **No Provider Sprawl**: Do not treat every vendor as a new family.
- **Local-First Default**: Prefer file, directory, and snapshot connectors when they satisfy the use case.
- **Write Caution**: Write-capable connectors require explicit capability and review semantics.

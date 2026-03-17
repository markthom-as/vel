---
title: Operator Surface Accessibility & Effective Config Clarity
status: planned
owner: staff-eng
type: architecture
priority: medium
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 018-cross-cutting-system-traits-baseline
  - 012-tester-readiness-onboarding
labels:
  - clients
  - web
  - apple
  - cli
  - accessibility
  - configurability
  - phase-2
---

# Context & Objectives

Vel has operator surfaces across web, Apple clients, CLI, and docs, but the queue does not yet treat accessibility and effective configuration clarity as one coherent cross-surface architecture concern.

This ticket establishes a baseline for accessible operator flows and inspectable configuration across all primary surfaces.

# Impacted Files & Symbols

- **Directory**: `clients/web/src/`
  - **Symbols**: navigation, settings, context, onboarding, diagnostics
- **Directory**: `clients/apple/`
  - **Symbols**: primary capture, settings, diagnostics, documentation views
- **Crate**: `vel-cli`
  - **Symbols**: `config show`, diagnostics, node/link commands
- **Docs**: `docs/user/`
  - **Symbols**: setup, troubleshooting, surfaces

# Technical Requirements

- **Accessibility Baseline**: Critical operator paths should support semantic labels, keyboard-first use where applicable, readable status messaging, and platform accessibility affordances.
- **Config Visibility**: Users should be able to inspect effective configuration and important defaults without reading source code.
- **Cross-Surface Consistency**: Web, Apple, CLI, and docs should use compatible language for source state, sync freshness, denial states, and degraded modes.
- **Diagnostics Reachability**: Configuration and recovery paths should be discoverable from the surface where the problem appears.

# Cross-Cutting Trait Impact
- **Modularity**: affected — surface-specific accessibility fixes should sit on top of shared contracts, not introduce policy forks.
- **Accessibility**: required — this is the primary trait under work.
- **Configurability**: required — effective config and defaults must be inspectable.
- **Data Logging**: affected — diagnostics and state messaging should connect to actual logs or inspect surfaces.
- **Rewind/Replay**: affected — queued-action and retry flows should explain replay behavior clearly.
- **Composability**: affected — docs, CLI, web, and Apple surfaces should reuse shared terminology and contracts.

# Implementation Steps (The "How")

1. **Audit**: Identify critical operator journeys across web, Apple, CLI, and docs.
2. **Config Mapping**: Define the set of config and status values users must be able to inspect.
3. **Surface Fixes**: Patch the highest-value accessibility and config-visibility gaps.
4. **Terminology Alignment**: Normalize wording for settings, freshness, denial, and degraded states across surfaces.

# Acceptance Criteria

1. [ ] Critical operator flows have an explicit accessibility baseline across the relevant surfaces.
2. [ ] Effective configuration is inspectable from at least one primary operator surface and documented for the others.
3. [ ] Settings, diagnostics, and degraded-mode messaging use consistent terminology across web, Apple, CLI, and docs.
4. [ ] Recovery paths for configuration issues are reachable from the surface that exposed the issue.

# Verification & Regression

- **Web Check**: keyboard and semantic-label smoke checks on the highest-value web flows
- **Apple Check**: platform accessibility review of primary capture/settings/diagnostics paths
- **CLI Check**: operator commands remain readable and structured where appropriate
- **Doc Check**: user docs explain the same effective-config and recovery model shown in product surfaces

# Agent Guardrails

- **No Accessibility Theater**: Do not equate visible polish with accessibility.
- **No Config Hide-and-Seek**: Do not bury effective settings behind dev-only commands unless that limitation is documented and intentional.
- **No Surface Drift**: Avoid inventing different words for the same state across clients and docs.

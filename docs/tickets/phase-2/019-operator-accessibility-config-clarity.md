---
title: Operator Surface Accessibility & Effective Config Clarity
status: in-progress
owner: staff-eng
type: architecture
priority: medium
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 018-cross-cutting-system-traits-baseline
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

Operator clarity and accessibility work already exists across web, CLI, Apple, and docs, but the remaining gaps are not explicitly tracked as closure work.

This ticket is narrowed to close those remaining gaps and align language/config diagnostics across surfaces.

# Current Baseline (Already Present)

- Web exposes freshness/degraded state and recovery affordances.
- CLI provides readable config and cluster summary outputs.
- Apple client surfaces endpoint resolution and offline queue state.

# Remaining Work Focus

- Align terminology across surfaces for freshness, denial, degraded mode, and recovery.
- Close accessibility gaps in high-friction journeys.
- Ensure effective config values are discoverable from the surface where issues appear.

# Impacted Files & Symbols

- **Directory**: `clients/web/src/`
  - **Symbols**: settings, diagnostics, context state messaging
- **Directory**: `clients/apple/`
  - **Symbols**: settings/diagnostics labels and recovery hints
- **Crate**: `vel-cli`
  - **Symbols**: config and diagnostics command output text
- **Docs**: `docs/user/`
  - **Symbols**: setup/troubleshooting wording and config guidance

# Technical Requirements

- **Accessibility Baseline**: critical operator flows support keyboard/readability/platform accessibility affordances.
- **Config Visibility**: effective values and defaults are inspectable without source diving.
- **Terminology Consistency**: same state words across web, Apple, CLI, and docs.
- **Reachable Recovery**: each surface exposes next-step recovery from where failure appears.

# Cross-Cutting Trait Impact

- **Modularity**: affected — UI labels should depend on shared contracts/state meaning.
- **Accessibility**: required — primary trait under work.
- **Configurability**: required — effective config visibility is a core output.
- **Data Logging**: affected — diagnostics text should map to inspectable state.
- **Rewind/Replay**: affected — queued/retry messaging should describe replay behavior consistently.
- **Composability**: required — language and diagnostics model shared across surfaces.

# Implementation Steps (The How)

1. **Gap audit**: list remaining accessibility/config-clarity deltas by surface.
2. **Terminology map**: define canonical wording table for key runtime states.
3. **Patch pass**: fix highest-value deltas in web/CLI/Apple/docs.
4. **Regression checks**: run focused surface tests and docs consistency checks.

# Acceptance Criteria

1. [ ] Critical operator journeys meet baseline accessibility checks.
2. [ ] Effective configuration is visible on at least one direct operator surface per journey.
3. [ ] Runtime state terminology is consistent across web, Apple, CLI, and docs.
4. [ ] Recovery guidance is reachable from the failure surface.

# Verification & Regression

- **Web Check**: focused keyboard/label tests in settings and diagnostics paths.
- **Apple Check**: platform accessibility review for settings/diagnostics.
- **CLI Check**: config/diagnostic command outputs stay readable and structured.
- **Doc Check**: user docs match shipped terminology and recovery paths.

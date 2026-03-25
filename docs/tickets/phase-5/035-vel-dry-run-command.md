---
title: vel dry-run Planning and Policy Preview
status: complete
owner: staff-eng
type: implementation
priority: high
created: 2026-03-24
updated: 2026-03-25
depends_on:
  - 030-capability-resolution-engine
  - 032-mutation-protocol-discipline
  - 033-llm-provider-interface
labels:
  - vel-cli
  - dry-run
  - policy
  - phase-5
---

# Context & Objectives

Add `vel dry-run` as an execution-disabled mode that previews plan, capability decisions, and mutation proposals without committing side effects.

# Impacted Files & Symbols

- **Crate**: `crates/vel-cli`
  - **Symbols**: `vel dry-run` command and preview output
- **Crate**: `crates/veld`
  - **Symbols**: no-commit execution mode and event marking
- **Crate**: `crates/vel-core`
  - **Symbols**: dry-run event/result markers

# Technical Requirements

- Use the same planning and policy pipeline as `vel run`.
- Explicitly disable mutation commits and tool side effects.
- Persist preview run with clear dry-run mode marker.
- Surface what would require confirmation in execute mode.

# Implementation Steps (The How)

1. Add dry-run mode flag to orchestration pipeline.
2. Short-circuit commit and side-effect execution paths.
3. Persist dry-run event stream and predicted decisions.
4. Add CLI output for proposed mutations and gate requirements.

# Acceptance Criteria

1. [x] `vel dry-run` produces plan and policy preview from real resolver path.
2. [x] Dry-run mode never commits mutations.
3. [x] Persisted run clearly indicates dry-run mode.
4. [x] Output lists confirmation-required operations.

# Verification & Regression

- **CLI Integration Test**: dry-run records plan without side effects.
- **Integration Test**: attempted commit path is skipped in dry-run mode.
- **Regression Test**: dry-run and execute mode produce consistent policy decisions.

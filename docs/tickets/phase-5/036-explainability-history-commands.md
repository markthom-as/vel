---
title: Explainability and Run History Commands
status: complete
owner: staff-eng
type: implementation
priority: medium
created: 2026-03-24
updated: 2026-03-25
depends_on:
  - 027-sqlite-run-store
  - 028-artifact-store
  - 034-vel-run-command
  - 035-vel-dry-run-command
labels:
  - vel-cli
  - explainability
  - history
  - phase-5
---

# Context & Objectives

Implement baseline trust surfaces: `vel explain <run-id>`, `vel runs`, and `vel artifacts <run-id>` using persisted run/event/artifact data only.

# Impacted Files & Symbols

- **Crate**: `crates/vel-cli`
  - **Symbols**: explain/history/artifact command handlers
- **Crate**: `crates/veld`
  - **Symbols**: query service/read models for run projection

# Technical Requirements

- Explain output must be derivable from persisted events and policy decisions.
- Run list must support recency ordering and status filtering.
- Artifact list must show typed refs and fetchable metadata.
- Missing/unknown run IDs must return clear errors.

# Implementation Steps (The How)

1. Add CLI subcommands and output formatting.
2. Add query service projections from canonical event/store data.
3. Add guardrails for missing IDs and incomplete runs.
4. Add golden tests for stable operator-facing output.

# Acceptance Criteria

1. [x] `vel runs` returns recent runs with status and timestamps.
2. [x] `vel explain <run-id>` includes policy and execution decision trail.
3. [x] `vel artifacts <run-id>` lists artifact refs and metadata.
4. [x] Unknown run IDs return non-zero exit and explicit guidance.

# Verification & Regression

- **CLI Integration Test**: run listing and filtering behavior.
- **CLI Integration Test**: explain output for success and denied runs.
- **CLI Integration Test**: artifacts listing for run with and without artifacts.

---
title: LLM-as-a-Judge Evaluation Pipeline (Evals)
status: planned
owner: staff-eng
type: verification
priority: medium
created: 2026-03-17
updated: 2026-03-17
depends_on:
  - 007-day-simulation-harness
  - 017-execution-tracing-reviewability
labels:
  - veld
  - llm-eval
  - reasoning
  - phase-3
---

# Context & Objectives

Vel currently has runtime evaluation orchestration (`/v1/evaluate`) but no dedicated reasoning-quality eval pipeline crate that combines deterministic checks with model-judged quality scoring.

This ticket introduces a dedicated eval toolchain that reports judge scores alongside deterministic and execution-backed failures.

# Impacted Files & Symbols

- **Crate (new)**: `crates/veld-evals`
  - **Symbols**: eval runner CLI, scenario loader, report writer
- **Crate**: `vel-llm`
  - **Symbols**: model provider integration used by judge mode
- **Crate**: `veld`
  - **Symbols**: runtime endpoints/fixtures used by execution-backed checks
- **Docs**: `docs/` eval operation and interpretation guidance

# Technical Requirements

- **Eval Runner**: provide a standalone CLI for repeatable eval execution.
- **Dataset Fixtures**: store versioned eval scenarios and expected deterministic checks.
- **Judge Mode**: support provider-configurable LLM judgment with explicit rubric.
- **Paired Gates**: judge scores must be paired with deterministic and execution assertions.
- **Structured Reports**: emit machine-readable report with per-scenario breakdowns.

# Cross-Cutting Trait Impact

- **Modularity**: required — eval crate should not entangle runtime request handlers.
- **Accessibility**: affected — report summaries should be readable for operators/reviewers.
- **Configurability**: required — judge model/provider thresholds and rubric are explicit.
- **Data Logging**: required — eval outcomes and failure reasons are inspectable.
- **Rewind/Replay**: required — eval fixtures must be reproducible and versioned.
- **Composability**: required — eval scenarios should compose with simulation harness fixtures.

# Implementation Steps (The How)

1. **Scaffold crate**: add `crates/veld-evals` with CLI entrypoint.
2. **Fixture model**: define scenario schema and deterministic expectation fields.
3. **Judge integration**: add pluggable judge invocation with rubric and thresholds.
4. **Reporting + CI**: emit structured output and add CI-friendly failure policy.

# Acceptance Criteria

1. [ ] Eval runner executes fixture scenarios and outputs structured reports.
2. [ ] Reports separate deterministic failures from model-judged quality scores.
3. [ ] Judge threshold regressions can fail configured CI gates.
4. [ ] Eval runs are reproducible with pinned fixture and config inputs.

# Verification & Regression

- **Unit Test**: fixture parsing and report generation.
- **Integration Test**: end-to-end eval run with deterministic + judge checks.
- **Smoke Check**: run eval command on a small fixture set locally.
- **Invariants**: no pass result is produced when deterministic hard-gates fail.

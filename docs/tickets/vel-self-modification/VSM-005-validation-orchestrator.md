---
id: VSM-005
title: Validation Orchestrator
status: proposed
priority: P0
owner: platform
labels: [validation, testing, sandbox, self-modification]
---

## Summary
Build an orchestrator that resolves required checks from the protected surface registry and executes them in an isolated environment.

## Why
Self-modification without validation is just automated regression farming.

## Scope
- Resolve validation requirements per changed path and class.
- Execute checks in sandbox/worktree.
- Capture structured results and artifacts.

## Validation types
- schema
- lint/format
- typecheck
- unit
- integration
- policy_enforcement
- prompt_eval
- eval_regression
- benchmark_regression
- security_scan

## Implementation tasks
1. Map registry validations to concrete runners.
2. Add orchestration layer with per-check status.
3. Capture logs/artifacts and attach to proposal/ledger.
4. Support check timeouts and retries policy.
5. Produce aggregate pass/fail decision with explanation.

## Acceptance criteria
- Required validations derive from changed paths, not ad hoc guessing.
- Validation runs in isolation.
- Result object records each check, duration, artifact path, and outcome.
- Failure blocks apply.

## Dependencies
- VSM-001, VSM-002, VSM-004, VSM-008.


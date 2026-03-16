# Ticket 006 — Agentic CI guardrails

## Goal

Add lightweight guardrails that make agent-written changes safer, smaller, and easier to review.

This ticket turns current repo discipline into enforceable checks.

## Why now

The repo has reached the stage where the main danger is not “missing architecture,” but “subsystems quietly diverging.” Guardrails help keep agents from free-associating all over the codebase.

## Current starting point

- no visible `.github/workflows/`
- no visible `Makefile`, `justfile`, or scripts
- `docs/status.md` is the implementation truth source
- `AGENTS.md` already encodes repo boundaries in prose

## Deliverable

Introduce minimal CI or CI-like validation for:

1. formatting / lint / tests
2. docs-status consistency
3. no side effects in explain paths
4. patch-size hygiene guidance

## Implementation plan

### 1. Add a CI workflow if the repo uses GitHub
Recommended jobs:
- test workspace
- docs sanity checks
- optional grep-style architecture assertions

### 2. Add static architecture checks
Cheap but valuable examples:
- `vel-storage` must not depend on `vel-api-types`
- explain routes must not call mutating evaluation functions directly
- status docs should not claim unimplemented features as implemented

### 3. Add PR checklist docs
Even if no CI is added immediately, add a review checklist under `docs/` or `.github/` covering:
- tests added or updated
- status docs updated
- small patch preferred
- explain endpoints read-only

## Files likely touched

- `.github/workflows/ci.yml` (new) if applicable
- `AGENTS.md`
- `docs/agentic-dev-process.md`
- maybe a simple validation script under `scripts/`

## Tests

The workflow itself is the test.
If adding grep-based checks, keep them narrow and stable.

## Acceptance criteria

- there is an automated or semi-automated quality gate
- explain-path mutation regressions become harder to introduce
- repo-boundary rules are not only prose
- the checks are minimal and not brittle theater

## Out of scope

- enterprise CI complexity
- flaky style policing
- giant custom linters

## Suggested agent prompt

Implement Ticket 006.

Add the smallest useful CI/guardrail layer for agent work in Vel.
Prioritize architecture safety over ceremony.
Prefer a few sharp checks over a sprawling workflow.

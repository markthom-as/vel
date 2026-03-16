---
id: VEL-META-011
title: Background jobs retries idempotency and concurrency safety
status: proposed
priority: P1
estimate: 2-3 days
dependencies: [VEL-META-003, VEL-META-006]
---

# Goal

Make scanning and application safe under retries, crashes, and concurrent source edits.

# Scope

- Job types for:
  - source scan
  - gap detect
  - candidate generate
  - apply writeback
  - retry failed actions
- Deterministic idempotency keys.
- Concurrency/version checks.
- Dead-letter or failed-action review state.

# Deliverables

- job records and workers
- retry/backoff policies
- conflict detection behavior
- operator-facing failure states

# Acceptance criteria

- Retried jobs do not duplicate writeback.
- Version conflicts surface as resolvable states.
- Batch jobs support partial failure accounting.

# Notes

Nothing says “agentic system” like accidentally tagging the same 400 tasks three times.

---
id: VSM-014
title: Eval Replay Harness
status: proposed
priority: P1
owner: platform
labels: [evals, replay, regression, testing]
---

## Summary
Build a replay harness that runs proposed changes against prior failures and representative scenarios.

## Why
A patch that fixes today’s paper cut by amputating tomorrow’s hand is not an improvement.

## Scope
- Capture fixtures from relevant failures/tasks.
- Replay before/after behavior.
- Emit regression report with pass/fail deltas.

## Implementation tasks
1. Define fixture capture format.
2. Add replay executor for targeted subsystems.
3. Add before/after comparison reporting.
4. Persist reports as validation artifacts.
5. Integrate into required checks for Class B/C paths.

## Acceptance criteria
- Harness can replay historical cases deterministically enough to be useful.
- Reports show deltas, not just raw logs.
- Replay results are attached to proposals.
- At least one real subsystem is covered initially.

## Dependencies
- VSM-005, VSM-010.


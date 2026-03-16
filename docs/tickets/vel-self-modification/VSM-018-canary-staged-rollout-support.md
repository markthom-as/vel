---
id: VSM-018
title: Canary and Staged Rollout Support
status: proposed
priority: P2
owner: platform
labels: [deploy, canary, rollout, regression]
---

## Summary
Add staged rollout policies for higher-risk bounded logic changes.

## Why
Some changes deserve probation before parole.

## Scope
- Canary target selection.
- Health thresholds and automatic rollback.
- Promotion from canary to wider rollout.

## Implementation tasks
1. Define rollout policy objects.
2. Add canary deployment hooks.
3. Connect health metrics and regression detection.
4. Add auto-rollback thresholds.
5. Surface rollout state in UI/ledger.

## Acceptance criteria
- Class C bounded changes can roll out gradually.
- Health regressions halt promotion.
- Rollbacks are automatic when thresholds breach.
- Operators can inspect rollout stage and outcome.

## Dependencies
- VSM-009, VSM-019.


# Phase 74 Latency Note — Now Completion Path

## Claim

The active `Now` mutation path is more stable and perceptibly faster because the surface now reconciles locally before background refetch.

## Comparison

Before action:

- focus heading rendered: `Write weekly review`
- centered loading-state count: `0`
- `Recently completed` section count: `0`

After action:

- one canonical PATCH executed to `/v1/commitments/commit_local_1`
- centered loading-state count remained `0`
- `Recently completed` section count became `1`
- completed-state control rendered for `Write weekly review`

## Evidence

- [dom-diff.json](/home/jove/code/vel/.planning/milestones/v0.5.2-operator-surface-embodiment/74-evidence/now-complete/dom-diff.json)
- [operations.json](/home/jove/code/vel/.planning/milestones/v0.5.2-operator-surface-embodiment/74-evidence/now-complete/operations.json)
- [NOTE.md](/home/jove/code/vel/.planning/milestones/v0.5.2-operator-surface-embodiment/74-evidence/now-complete/NOTE.md)

## Interpretation

This is not a benchmark claim. It is an operator-visible stability claim: the surface no longer waits for a full post-mutation refetch before showing the new state, and it no longer blanks into the centered loading shell during the completion flow.

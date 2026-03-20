# Phase 26 Validation

## Goal

Make `reflow` a real backend-owned day-repair lane without creating a parallel planner, weakening explainability, or pushing schedule logic into shells.

## Required Truths

- `reflow` produces a typed remaining-day proposal rather than only warning/status posture
- stale schedule, missed event, slipped block, and "didn't fit" outcomes share one canonical reconciliation path
- scheduler/tagging rules from `codex-workspace` are mapped intentionally into Vel-owned semantics
- shells consume typed reflow output and explanation rather than deriving scheduling logic locally
- Settings exposes recovery/freshness posture in a summary-first operator way

## Plan Shape

Phase 26 should be executed in four slices:

1. canonical reflow/reconciliation contract and scheduler-rule mapping
2. backend remaining-day recomputation with explicit moved/unscheduled outcomes
3. shell embodiment in `Now`, `Threads`, and `Settings`
4. docs/verification closure with honest limits

## Block Conditions

Block if any slice:

- invents a second planner subsystem outside the current daily-loop / `Now` / `Threads` model
- treats raw upstream tags as the durable product model instead of canonical Vel semantics
- pushes schedule-diff or reconciliation ownership into web or Apple shells
- claims autonomous or multi-day planning behavior that is not actually implemented
- makes recovery posture noisier without improving trust or actionability

## Exit Condition

Phase 26 is complete when the product can honestly say:

- Vel can recompute the remaining day when the schedule goes stale
- operators can inspect what moved, what did not fit, and what still needs supervision
- Settings and the default shell surfaces reflect real recovery posture and trust guidance

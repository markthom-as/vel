# Phase 28 Validation

## Goal

Make same-day planning proactive and explainable by combining routine blocks, calendar anchors, and canonical scheduler rules into one backend-owned planning lane.

## Required Truths

- bounded same-day planning exists before `reflow` is needed
- routine blocks are explicit backend-owned planning inputs
- calendar anchors, commitments, and canonical scheduler rules feed the same typed plan output
- plan output explains what was scheduled, deferred, and did not fit
- shells remain presentation surfaces and do not become planner owners

## Plan Shape

Phase 28 should be executed in four slices:

1. canonical routine-block and day-plan contract
2. backend-owned initial day-plan shaping
3. shell embodiment over the typed planning output
4. docs/examples/verification closure

## Block Conditions

Block if any slice:

- claims multi-day planning or broad autonomous optimization that is not implemented
- creates shell-local or assistant-local planner rules
- treats raw provider labels as planning truth instead of canonical scheduler semantics
- makes routine blocks implicit or untyped
- weakens explainability compared to the current `reflow` contract

## Exit Condition

Phase 28 is complete when the product can honestly say:

- Vel shapes a bounded same-day plan before drift
- routine blocks and calendar anchors are typed planning inputs
- planning and `reflow` share one coherent scheduler-rule substrate
- docs and examples teach the bounded day-planning model and its limits clearly

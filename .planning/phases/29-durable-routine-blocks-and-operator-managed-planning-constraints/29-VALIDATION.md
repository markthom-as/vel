# Phase 29 Validation

## Goal

Make routine blocks and bounded planning constraints durable backend-owned records that improve same-day planning without widening Vel into a speculative planner.

## Required Truths

- durable routine-block records exist outside transient current-context inference
- bounded planning constraints are typed backend-owned inputs
- `day_plan` and `reflow` consume the same durable planning substrate
- shells remain summary and management surfaces, not planner owners
- the shipped model remains same-day, explainable, and intentionally bounded

## Plan Shape

Phase 29 should be executed in four slices:

1. durable routine-block and planning-constraint contract
2. backend/storage persistence seams
3. planning/runtime and shell embodiment over durable records
4. docs/examples/verification closure

## Block Conditions

Block if any slice:

- creates a parallel planner model separate from `day_plan` / `reflow`
- turns shells into the authority for routine or constraint semantics
- widens into multi-day optimization or broad autonomous calendar mutation
- treats provider-specific labels as the durable routine/planning model
- claims a complete habit/routine product that is not implemented

## Exit Condition

Phase 29 is complete when the product can honestly say:

- routine blocks and bounded planning constraints persist durably as backend-owned records
- same-day planning and recovery consume them coherently
- operators can manage them through shipped surfaces without reverse-engineering backend state
- docs and checked-in examples teach the durable routine-backed planning model and its current limits

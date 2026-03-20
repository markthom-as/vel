# Phase 30 Validation

## Goal

Make the durable routine/planning profile operator-manageable through typed backend seams and summary-first shipped surfaces without widening Vel into a second planner.

## Required Truths

- durable routine blocks and bounded planning constraints remain one canonical backend-owned planning profile
- planning-profile inspection and mutation happen through typed service/route/storage seams
- `day_plan` and `reflow` continue consuming the same canonical profile
- `Settings`, `Now`, and `Threads` stay summary-first over that profile instead of becoming planner owners
- the shipped model remains same-day, explainable, and intentionally bounded

## Plan Shape

Phase 30 should be executed in four slices:

1. planning-profile management contract and transport seam
2. backend/storage mutation and validation paths
3. shipped-surface embodiment over the canonical planning profile
4. docs/examples/verification closure

## Block Conditions

Block if any slice:

- creates shell-owned planning semantics or a second planner model
- stores routine/profile edits in generic JSON blobs instead of typed seams
- widens into multi-day planning, broad calendar editing, or autonomous upstream mutation
- bypasses validation for routine blocks or planning constraints
- claims a full routine-management product beyond the bounded same-day planning substrate

## Exit Condition

Phase 30 is complete when the product can honestly say:

- operators can inspect and manage the planning profile through typed backend-owned seams
- same-day planning and recovery continue to consume that profile coherently
- shipped surfaces expose summary-first management without reverse-engineering backend state
- docs and checked-in assets teach the operator-managed planning-profile model and its current limits

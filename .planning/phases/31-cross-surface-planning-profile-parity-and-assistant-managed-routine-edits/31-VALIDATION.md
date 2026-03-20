# Phase 31 Validation

## Goal

Make planning-profile inspection and bounded routine/profile edits coherent across web, CLI, Apple, and assistant/voice entry without creating a second planner.

## Required Truths

- one canonical backend-owned `RoutinePlanningProfile` remains the planning authority
- CLI, Apple, and assistant entry consume or mutate that profile through typed backend seams
- assistant/voice routine edits preserve confirmation, provenance, and fail-closed behavior
- `day_plan` and `reflow` keep reading the same planning substrate after parity work lands
- shells remain thin over the same planning-profile truth

## Plan Shape

Phase 31 should be executed in four slices:

1. parity/edit contract and transport widening
2. CLI and Apple inspection/parity over the canonical profile
3. assistant/voice-driven routine/profile edit flow over typed mutations
4. docs/examples/verification closure

## Block Conditions

Block if any slice:

- creates shell-local routine/profile state or a second planner model
- lets assistant/voice mutate the planning profile without explicit confirmation where needed
- bypasses typed planning-profile mutation and provenance tracking
- widens into autonomous planner mutation, broad calendar editing, or multi-day planning

## Exit Condition

Phase 31 is complete when the product can honestly say:

- the planning profile is a shared cross-surface inspect/manage seam rather than a web-only surface
- assistant and voice flows can stage bounded routine/profile edits through the same canonical mutation model
- confirmation and continuity remain explicit and inspectable
- docs teach the shared cross-surface planning-profile story and its current limits

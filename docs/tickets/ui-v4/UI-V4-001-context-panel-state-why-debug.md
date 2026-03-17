# UI-V4-001 — Refactor context panel into State / Why / Debug modes

Status: todo
Priority: P0
Lane: A

## Why

The screenshot set shows the current right rail mixing compact state, explanation, and raw state fields in one uninterrupted stack.

Evidence:

- `~/Downloads/localhost_5173_.png`
- `~/Downloads/localhost_5173_ (1).png`
- `~/Downloads/localhost_5173_ (2).png`
- `~/Downloads/localhost_5173_ (3).png`

## Goal

Split the context surface into explicit modes so the operator can choose between:

- `State`
- `Why`
- `Debug`

## Ownership / likely write scope

- web context panel and related components
- any read-model/view-model work needed to support the split
- docs for the context surface contract

## Deliverables

- mode switcher and empty/degraded states
- distinct content contracts for each mode
- preserve explainability value while reducing always-visible debug weight
- tests for mode switching and mode-specific rendering

## Acceptance criteria

- `State` answers what Vel currently believes
- `Why` answers why Vel believes it
- `Debug` exposes raw/system detail without polluting the other two modes
- the context rail no longer reads like one undifferentiated diagnostic stack

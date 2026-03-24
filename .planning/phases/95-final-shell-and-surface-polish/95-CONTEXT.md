# Phase 95 Context

## Goal

Close the remaining browser-level fidelity issues now that the runtime behavior seams are in place.

## Preconditions now satisfied

- `Now` lane truth is transport-backed
- nudge acknowledge/defer behavior has a truthful intervention seam where the accepted UI needs one
- `Threads` default selection and archive behavior are truthful
- `System` exposes persisted operator settings and sticky third-level navigation

## Main implementation targets

- final pill/tag/badge centering and radius consistency
- navbar badge temperament, docs affordance styling, and non-`Now` context-slot rhythm
- nudge gutter geometry, ring centering, chip density, and title/header polish
- `Now` progress/header/task-row/icon rhythm
- `Threads` row-state/avatar/meta/bubble-tail polish
- `System` density, stat layout, and final single-document readability pass

## Risks

- polish work must not reintroduce fake behavior or local-only truth
- browser-visible issues must be judged in-browser instead of by frontend tests alone

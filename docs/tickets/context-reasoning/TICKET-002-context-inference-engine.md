---
title: Context Inference Engine
status: open
---

## Boundary

This ticket must refine the current context reducer/inference flow, not introduce a second inference system that competes with `current_context`.

- Reuse the existing signals -> inference -> current context path wherever possible.
- New logic should improve inspectability, confidence handling, or explainability around that path.
- Do not create a separate belief engine that can disagree with the shipped context runtime without an explicit architectural decision.

# Goal

Generate clearer derived context outputs from signals within the existing runtime.

# Tasks

1. Extend or refactor the current rule-based inference layer instead of introducing a parallel one.
2. Integrate signals from:

- calendar
- device activity
- location
- prior tasks

3. Assign confidence weights or confidence bands where they improve explainability.
4. Persist derived outputs in a way that remains consistent with the current context/explain flow.

# Acceptance

- Inference produces derived context outputs that remain aligned with the existing context runtime.
- Confidence values or bands are available where added.

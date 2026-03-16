---
title: Implement confidence normalization and scoring
status: todo
priority: P0
owner: core
labels: [uncertainty, scoring, policy]
---

# Goal

Turn raw model-emitted assessment data into deterministic scores the policy engine can trust.

# Deliverables

- `packages/core/uncertainty/normalizer.ts`
- `packages/core/uncertainty/scoring.ts`
- bounded normalization for all confidence vector fields
- derived scoring functions for `safe_to_proceed_score`, `interrupt_user_score`, `ask_agent_score`, `retrieve_more_score`, `block_score`

# Requirements

- Clamp all model-derived numeric values into `[0,1]`.
- Handle missing values with explicit fallback logic, not silent NaNs.
- Keep coefficients centralized and configurable.
- Emit structured debug output so calibration is inspectable.

# Acceptance criteria

- Unit tests cover threshold boundaries and malformed input.
- A test demonstrates deterministic routing inputs for the same assessment.
- Scoring functions are side-effect free.

# Notes

Do not let the model directly choose policy outcomes. It gets to suggest. The code gets to decide.

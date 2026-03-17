---
title: Ticket 500 - Formalize the unified cognition pipeline
status: proposed
owner: codex
priority: critical
---

# Goal

Make the layered cognition model explicit in code and docs.

# Files

## New
- `docs/specs/vel-cognition-pipeline.md`

## Changed
- `crates/veld/src/services/evaluate.rs`
- `crates/veld/src/services/inference.rs`
- `crates/veld/src/services/suggestions.rs`
- `crates/veld/src/worker.rs`

# Implementation

Document and enforce the sequence:
1. ingest external items
2. emit signals
3. reconcile commitments / project state
4. compute current context
5. compute risks
6. compute suggestions
7. compute uncertainties
8. emit nudges / questions / sync proposals

Add comments and service boundaries so no later service silently reaches back around the pipeline.

# Acceptance criteria

- service call graph follows the documented order
- operators can explain where each output type comes from

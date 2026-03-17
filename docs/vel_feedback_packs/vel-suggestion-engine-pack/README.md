---
title: Vel Suggestion Engine Pack
status: proposed
owner: codex
generated_on: 2026-03-16
---

# Vel Suggestion Engine Pack

This pack turns the current `services/suggestions.rs` scaffold into a real steering subsystem.

## Current starting point

Existing code:
- `crates/veld/src/services/suggestions.rs`

Existing behavior:
- repeated resolved commute danger nudges create `increase_commute_buffer`
- repeated resolved prep-window nudges create `increase_prep_window`

That is a valid bootstrap. It is not yet a durable suggestion engine.

## Desired result

Suggestions should become:
- evidence-backed
- deduplicated
- policy-aware
- explainable
- actionable
- safe to surface repeatedly without spam

## Pack contents

1. `00-spec-suggestion-engine.md`
2. `tickets/001-persistence-and-evidence.md`
3. `tickets/002-evaluation-and-ranking.md`
4. `tickets/003-policy-integration.md`
5. `tickets/004-api-and-cli-surfaces.md`
6. `tickets/005-feedback-learning-loop.md`
7. `tickets/006-tests-and-fixtures.md`

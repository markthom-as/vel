---
title: Test Matrix and Canonical Fixtures Expansion
status: proposed
priority: critical
owner: codex
---

# Goal

Add scenario tests that guard the repo's real operating behavior instead of only local unit logic.

# Why this matters

Vel is becoming a temporal system. Unit tests alone will not catch regressions in:
- evaluate order
- read-only boundaries
- current context shape
- nudge/suggestion interactions

# Concrete code changes

## A. Add a canonical backend fixture helper
Create:
- `crates/veld/tests/support/mod.rs`

Put seed helpers there for:
- captures
- commitments
- calendar signals
- activity signals
- message-thread signals

## B. Add scenario tests
Create:
- `crates/veld/tests/evaluate_pipeline.rs`
- `crates/veld/tests/read_routes_are_read_only.rs`
- `crates/veld/tests/current_context_roundtrip.rs`

## C. Add assertions for evaluate ordering
A test should prove:
1. risk runs first
2. inference sees latest risk rows
3. nudge engine reads current context
4. suggestion engine runs after nudge evaluation

# Acceptance criteria

- canonical fixture helpers exist
- scenario tests cover the actual runtime loop
- the evaluate pipeline fails loudly if sequencing changes accidentally

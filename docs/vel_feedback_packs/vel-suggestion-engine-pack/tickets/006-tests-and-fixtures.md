---
title: Suggestion Engine Tests and Fixtures
status: proposed
priority: critical
owner: codex
---

# Goal

Protect the steering layer with scenario tests.

# Concrete code changes

Create:
- `crates/veld/tests/suggestion_engine.rs`

Cover:
1. repeated commute danger creates one suggestion
2. same evidence does not spam duplicate pending suggestions
3. rejected recent suggestion suppresses recreation
4. config thresholds change creation behavior
5. evidence rows are written and inspectable
6. suggestions are ranked deterministically

# Acceptance criteria

- suggestion behavior is verified end-to-end
- suppression and policy integration are covered
- evidence persistence is tested

---
title: Uncertainty Runtime Tests
status: proposed
priority: critical
owner: codex
---

# Goal

Protect uncertainty behavior with deterministic tests.

# Concrete code changes

Create:
- `crates/veld/tests/uncertainty_runtime.rs`

Cover:
1. missing travel minutes on a high-risk event emits an uncertainty record
2. low-confidence suggestion candidate is deferred, not persisted as a suggestion
3. score-to-band normalization is stable
4. policy thresholds alter resolution mode
5. uncertainty records can be listed and resolved

# Acceptance criteria

- uncertainty behavior is covered end-to-end
- critical low-confidence cases do not silently pass

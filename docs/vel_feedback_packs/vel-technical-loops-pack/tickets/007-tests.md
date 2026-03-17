---
title: Runtime Loop Tests
status: proposed
priority: critical
owner: codex
---

# Goal

Verify loop scheduling and execution deterministically.

# Concrete code changes

Create:
- `crates/veld/tests/runtime_loops.rs`

Cover:
1. due evaluate loop is claimed and run once
2. disabled loop does not run
3. failed loop records error and next due time
4. sync loop can trigger evaluate follow-up
5. retry loop remains functional after worker refactor

# Acceptance criteria

- loop runtime is covered by integration-style tests
- the worker refactor does not regress existing retry behavior

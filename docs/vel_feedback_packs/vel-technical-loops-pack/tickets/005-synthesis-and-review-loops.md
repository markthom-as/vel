---
title: Synthesis and Review Loops
status: proposed
priority: medium
owner: codex
---

# Goal

Add slower loops for review-oriented work:
- weekly synthesis
- stale nudge reconciliation
- uncertainty review

# Concrete file targets

- `crates/veld/src/worker.rs`
- `crates/veld/src/services/synthesis.rs`
- `crates/veld/src/services/uncertainty.rs`
- `config/policies.yaml`

# Concrete code changes

## Weekly synthesis loop
Once per week:
- generate `weekly_synthesis`
- only if enough recent data exists
- avoid duplicate synthesis for same window

## Stale nudge reconciliation
Periodically:
- expire or downgrade stale nudges
- resolve nudges whose triggering context no longer exists

## Uncertainty review
Periodically:
- summarize unresolved uncertainty
- optionally create a review artifact or operator nudge

# Acceptance criteria

- slower review loops are explicit runtime work
- weekly synthesis is no longer only manual
- stale nudges and unresolved uncertainty are cleaned up systematically

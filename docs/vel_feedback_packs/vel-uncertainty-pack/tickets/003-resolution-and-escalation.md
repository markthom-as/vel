---
title: Uncertainty Resolution and Escalation Rules
status: proposed
priority: high
owner: codex
---

# Goal

Teach Vel what to do when confidence is low.

# Concrete file targets

- `crates/veld/src/services/uncertainty.rs`
- `crates/veld/src/services/evaluate.rs`
- `crates/veld/src/routes/chat.rs`
- `config/policies.yaml`

# Concrete code changes

## Add policy section
```yaml
uncertainty:
  ask_user_threshold: 0.45
  defer_threshold: 0.25
  high_risk_force_ask_user: true
  low_confidence_suggestion_mode: defer
```

## Apply rules
Examples:
- low-confidence suggestion candidate -> do not persist suggestion; record deferred uncertainty
- missing critical event data for a high-risk commitment -> emit uncertainty and create a clarifying nudge or prompt
- chat answer with low evidence -> respond with explicit uncertainty language and record why

## Keep deterministic boundaries
Do not make uncertainty handling a magical LLM reflection pass. It should be policy-first.

# Acceptance criteria

- low-confidence cases cause explicit resolution behavior
- escalation thresholds are configurable
- behavior differs for low-stakes vs high-stakes decisions

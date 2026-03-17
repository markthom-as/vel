---
title: Policy-Driven Suggestion Thresholds and Suppression
status: proposed
priority: high
owner: codex
---

# Goal

Move thresholds, windows, and suppression rules into policy/config.

# Concrete file targets

- `crates/veld/src/policy_config.rs`
- `config/policies.yaml`
- `crates/veld/src/services/suggestions.rs`

# Concrete code changes

## Add policy shape

Suggested config section:
```yaml
suggestions:
  enabled: true
  window_days: 7
  suppression_days: 14
  max_new_per_evaluate: 3
  commute:
    threshold: 2
    increment_minutes: 10
  prep:
    threshold: 2
    increment_minutes: 15
  response_debt:
    threshold: 3
    followup_block_minutes: 20
  morning_drift:
    threshold: 3
```

## Use policy in evaluation
Remove:
- `const REPEAT_THRESHOLD`
- `const WINDOW_DAYS`

Replace with config reads and defaults.

## Add suppression semantics
Suppress when:
- same `dedupe_key` exists in pending state
- same `dedupe_key` was rejected recently
- same `dedupe_key` was accepted recently and underlying policy already changed

# Acceptance criteria

- suggestion thresholds are not magic constants
- suppression rules are explicit and configurable
- evaluate creates at most a bounded number of new suggestions per pass

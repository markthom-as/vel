---
title: Automatic Evaluate Loop
status: proposed
priority: critical
owner: codex
---

# Goal

Run risk -> inference -> nudges -> suggestions automatically on a bounded schedule.

# Concrete file targets

- `crates/veld/src/worker.rs`
- `crates/veld/src/services/evaluate.rs`
- `config/policies.yaml`
- `docs/status.md`

# Concrete code changes

## Add evaluate loop config
Example:
```yaml
loops:
  evaluate_current_state:
    enabled: true
    interval_seconds: 300
```

## Add loop runner
The loop should:
1. claim loop execution
2. call evaluate service
3. emit a concise loop result event or log
4. schedule next due time

## Optional trigger heuristics
Later you can trigger evaluate more aggressively after adapter syncs, but v1 should just be interval-based plus manual API trigger.

# Acceptance criteria

- Vel evaluates current state automatically
- operator can still trigger manual evaluate
- loop status is inspectable

---
title: Replay + Reflection
status: open
---

# Goal

Support replay of prior runs against frozen context and reflection jobs that produce improvement suggestions.

# Tasks

1. Run replay harness — replay a run using recorded inputs, recorded/mocked tool outputs, frozen memory snapshot; validate against expected output contract.
2. Reflection jobs — scheduled jobs that analyze runs (e.g. suggestion success, false alarms, failures) and produce improvement suggestions.
3. Improvement suggestion feed — store and expose reflection output (e.g. "reduce commute alerts by 10 minutes") for operators or vel-coder.

# Acceptance Criteria

- A prior run can be replayed with frozen context for regression and policy evaluation.
- Reflection produces structured improvement suggestions.
- Improvement feed is queryable and can drive self-improvement pipeline (propose → test → replay → approve → stage → merge).

# Spec reference

[docs/specs/vel-agent-runtime-spec.md](../../specs/vel-agent-runtime-spec.md) — Replay and Simulation, Self-Improvement Hooks.

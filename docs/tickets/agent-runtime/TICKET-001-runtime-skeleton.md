---
title: Runtime Skeleton
status: open
---

# Goal

Implement the agent runtime skeleton: spec loader, run lifecycle, spawn validation, and structured return contracts.

# Tasks

1. Agent spec loader — load and validate agent YAML specs (id, mission, allowed_tools, memory_scope, return_contract, TTL, budgets).
2. Run lifecycle manager — create, queue, run, wait, complete/fail/expire/cancel with TTL and waiting-reason.
3. Spawn validation — validate spawn requests (valid spec, compatible mission input, parent-child, budget, memory scope, tool set).
4. Structured return contracts — define and enforce return payload schema per agent type (status, summary, evidence, confidence, suggested_actions, artifacts, errors).

# Acceptance Criteria

- Agent specs load from config and validate.
- Runs transition through lifecycle states with TTL and waiting metadata.
- Spawn requests are validated before creating a run.
- Every agent type returns a structured contract; no free-form-only output.

# Spec reference

[docs/specs/vel-agent-runtime-spec.md](../../specs/vel-agent-runtime-spec.md) — Agent Lifecycle, Agent Spec, Spawn Contract, Return Contract.

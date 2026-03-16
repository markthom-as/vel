---
title: Build clarification policy engine
status: todo
priority: P0
owner: orchestration
labels: [uncertainty, policy, routing]
---

# Goal

Implement deterministic routing from `AgentAssessment` to next action.

# Deliverables

- `packages/core/clarification-policy/engine.ts`
- `packages/core/clarification-policy/rules.ts`
- `packages/core/clarification-policy/thresholds.ts`
- policy result type with selected action, rationale, and resolver target

# Requirements

- Support actions: `proceed`, `proceed_with_annotation`, `ask_user`, `ask_agent`, `retrieve_more`, `simulate`, `block`
- Incorporate user preference mode into thresholds.
- Hard-block destructive or high-blast-radius actions when confidence is too low.
- Prefer cheaper resolvers before user interruption unless uncertainty is fundamentally normative or approval-bound.

# Acceptance criteria

- Integration tests cover at least one case per action.
- Policy decision includes human-readable rationale for logs/UI.
- Threshold configuration can be changed without modifying engine logic.

# Notes

Start rule-based. Resist the temptation to make a tiny inscrutable judge model because it feels fancy.

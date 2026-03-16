---
title: Update agent step contract to emit assessment metadata
status: todo
priority: P0
owner: agents
labels: [uncertainty, agents, contract]
---

# Goal

Require actionable agent steps to emit uncertainty-aware assessment metadata before side-effectful execution.

# Deliverables

- contract/interface update for agent step outputs
- migration plan for existing agents
- shim for agents not yet upgraded

# Requirements

- Contract should include `confidenceVector`, `uncertainties`, `recommendedAction`, and rationale.
- Legacy agents should degrade gracefully rather than crash the runtime.
- Side-effectful steps should not bypass the policy engine unless explicitly whitelisted.

# Acceptance criteria

- At least one existing agent is migrated end-to-end.
- Legacy shim test passes.
- Runtime logs indicate when old agents are operating in degraded mode.

# Notes

This ticket is the bridge between the nice theory and the messy reality of an evolving agent stack.

---
title: Create inter-agent consultation resolver
status: todo
priority: P1
owner: orchestration
labels: [uncertainty, resolver, multi-agent]
---

# Goal

Allow Vel to route uncertainty to another agent when that agent has better domain authority than the current actor.

# Deliverables

- `packages/core/resolvers/agent-resolver.ts`
- resolver registry integration
- target agent selection logic using authority and expected information gain
- returned advice normalization into assessment/decision updates

# Requirements

- Support named specialist agents such as architecture, security, design-system, product-spec.
- Preserve provenance of agent advice.
- Prevent infinite consultation loops.
- Cap consultation depth or count per task step.

# Acceptance criteria

- Integration test shows architectural uncertainty routed to architecture agent.
- Returned advice is stored as agent-sourced assumption or resolved uncertainty.
- Loop guard test passes.

# Notes

Another agent should be a resolver, not an absolution machine.

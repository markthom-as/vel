# Agent Orchestrator

Vel should not be one giant demiurge process.
It should start with one explicit orchestrator and only coordinate specialized agents when the split is justified.

## Default Posture

- one orchestrator by default
- add specialists only when they reduce complexity or isolate risk
- every added agent must have a narrower scope than the orchestrator, not another copy of it
- no agent gets ambient authority to all tools, secrets, or write surfaces

## Recommended Agent Roles

- planner
- context synthesizer
- risk evaluator
- suggestion composer
- notification broker
- reflection analyst
- integration worker

## Orchestration Principles

- single orchestrator first
- narrow responsibilities
- explicit handoff contracts
- bounded tool access
- explicit capability scopes
- execution leases and termination rules
- observable execution traces
- reviewable outputs

## Workflow Pattern

```text
trigger
→ context synthesis
→ risk evaluation
→ suggestion generation
→ policy check
→ surface selection
→ user delivery
→ feedback capture
→ learning capture
```

## Anti-Pattern

Avoid "one omnipotent agent with every tool and vague vibes".
That way lies latency, hallucinated authority, and maintenance hell.

Avoid spawning subagents just because the problem feels sophisticated.
Extra agents are a cost center unless they give clearer ownership, safer capabilities, or better review boundaries.

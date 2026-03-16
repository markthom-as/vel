# Agent Orchestrator

Vel should not be one giant demiurge process.
It should coordinate specialized agents.

## Recommended Agent Roles

- planner
- context synthesizer
- risk evaluator
- suggestion composer
- notification broker
- reflection analyst
- integration worker

## Orchestration Principles

- narrow responsibilities
- explicit handoff contracts
- bounded tool access
- observable execution traces

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
```

## Anti-Pattern

Avoid "one omnipotent agent with every tool and vague vibes".
That way lies latency, hallucinated authority, and maintenance hell.

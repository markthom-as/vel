# Vel Skills + Workflows Pack

This pack extends the original skill-system architecture with a first-class workflow runtime.

A workflow in Vel is a governed, triggerable, auditable orchestration package that runs one or more skills inside a bounded execution context such as:

- project
- task
- nudge
- thread
- session
- schedule
- event-triggered automation

The recommended model is:

- **tools** = primitive capabilities
- **skills** = reusable applied behaviors
- **workflows** = multi-step orchestrations over skills and tools
- **agents** = planners/operators that may invoke tools, skills, and workflows under policy

## Contents

- `docs/` — architecture, schemas, roadmap, policy, context model, workflow runtime design
- `schemas/` — starter JSON Schemas for skills and workflows
- `examples/` — example skill and workflow packages
- `tickets/` — Codex-ready implementation tickets for MVP and later phases

## Workflow Positioning

Workflows should be first-class citizens, not just a skill subtype hacked into submission.

A workflow package should support:

- trigger bindings (time, event, manual, state change)
- execution context binding (project/task/thread/nudge/session)
- step graph / sequential plan / conditional branch
- step-level policy and confirmation rules
- resumability, retries, idempotency, and audit logs
- human checkpoints and approval gates
- structured outputs and artifact emission
- deep CLI and runtime integration

## Suggested filesystem layout

```text
vel/
  crates/
    skill-runtime/
    workflow-runtime/
    trigger-runtime/
    policy-engine/
    registry/
    cli/
  skills/
    core/
    integrations/
    local/
  workflows/
    core/
    automation/
    projects/
    user/
  docs/
```

## Suggested command surface

```bash
vel skill list
vel workflow list
vel workflow inspect core/morning-orientation
vel workflow run core/morning-orientation --context-file context.json
vel workflow test core/morning-orientation
vel workflow trigger list
vel workflow logs core/morning-orientation --last 50
```

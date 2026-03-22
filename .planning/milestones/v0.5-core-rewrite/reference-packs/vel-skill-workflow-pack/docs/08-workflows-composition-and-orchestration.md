# Workflows, Composition, and Orchestration

## Why composition matters

A skill system becomes dramatically more valuable when skills can call other skills in a governed way.

Without composition, you just have a cabinet full of clever trinkets.

With composition, you start to get reusable behavior pipelines.

## Recommended composition model

Start simple.

Support:

- sequential execution
- structured output passing
- conditional branching
- retries
- dry-run mode

Do not start with BPMN cathedral syndrome or a giant visual no-code editor. Those things can wait their turn.

## Example workflow skill

```yaml
apiVersion: vel/v1
kind: Skill

metadata:
  name: morning-orchestrator
  namespace: core
  version: 0.1.0

spec:
  type: workflow
  workflow:
    steps:
      - id: context
        skill: core/context-pack
      - id: calendar
        skill: core/calendar-brief
        input:
          source: steps.context.output.calendar
      - id: tasks
        skill: core/task-triage
        input:
          source: steps.context.output.tasks
      - id: standup
        skill: core/generate-standup
        input:
          calendar: steps.calendar.output
          tasks: steps.tasks.output
```

## Skill-to-skill calling contract

When one skill calls another, the runtime should:

- evaluate dependency availability
- re-check permissions in nested context
- validate inputs and outputs
- preserve traceability between parent and child runs

## Parent-child policy model

A parent skill should not be able to smuggle forbidden capabilities into a child run.

Effective grant for child run should be:

- child requested capabilities
- intersected with parent grants
- intersected with runtime/user/workspace policy

## Routing and orchestration

Later, Vel can support richer agent routing:

- select candidate skills by intent
- score confidence
- choose one or more skills
- chain them automatically
- degrade gracefully when unavailable

But that layer should sit above the stable skill runtime contract.

## Degradation model

If a dependency is unavailable, a workflow should be able to:

- fail hard
- skip step
- fall back to alternate step
- ask for confirmation/input

This should be explicit, not magical.

## Recommendation

Support a modest workflow mode in phase 3 after basic single-skill runtime is stable. That is the moment when the architecture starts compounding instead of merely existing.

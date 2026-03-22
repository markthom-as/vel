# Workflow System Overview

## Why workflows need to be first-class

A skill system alone gives Vel pluggable, permissioned, reusable units of behavior. That is necessary, but not sufficient. Vel also needs a way to coordinate those skills across time, events, user actions, and state transitions.

That is the role of the workflow system.

A workflow is a governed orchestration package that:

- binds one or more triggers to one or more execution contexts
- evaluates preconditions and policy
- runs a sequence or graph of steps
- calls skills, tools, prompts, and deterministic hooks
- can pause for human approval or resume later
- emits structured outputs, audits, and artifacts

In practice, workflows are what make Vel feel like a real assistant rather than a bag of disconnected competencies.

## Core distinction

- **Tool**: primitive capability, e.g. read calendar, update Todoist task, query notes, run shell command
- **Skill**: reusable applied behavior, e.g. morning brief, task enrichment, standup generation
- **Workflow**: orchestrated chain of behavior over time and context, e.g. when day starts, run morning orientation in project/task/thread context and create nudges and follow-ups

## The guiding idea

Treat workflows as the *governed temporal layer* above skills.

Skills answer “how to do a thing.”
Workflows answer “when, why, and in what context should several things happen together.”

## Workflow package anatomy

A workflow package should contain:

- manifest
- trigger definitions
- context binding rules
- state machine / step graph / ordered steps
- optional templates/prompts
- optional hooks
- optional tests and fixtures
- explicit permissions and confirmation rules

Example:

```text
workflows/
  core/
    morning-orientation/
      workflow.yaml
      prompt.md
      input.schema.json
      output.schema.json
      triggers.yaml
      steps/
        01-load-context.yaml
        02-generate-brief.yaml
        03-task-triage.yaml
        04-create-nudges.yaml
      hooks/
        on_start.ts
        on_error.ts
        on_complete.ts
      tests/
        happy-path.yaml
        missing-calendar.yaml
```

## Workflow execution contexts

A workflow run should always be bound to one or more contextual anchors.

Primary anchors:

- user
- workspace
- project
- task
- thread
- nudge
- event
- schedule window
- artifact
- run/session

A workflow may read broadly, but it should execute from a clear locus. This keeps reasoning, logging, and UI presentation legible.

## Example workflow use cases

- Morning orientation workflow
- Post-meeting capture workflow
- Project drift detection workflow
- Deadline proximity workflow
- Calendar-task reconciliation workflow
- Thread follow-up workflow
- Nudge aging/escalation workflow
- Intake-to-project-classification workflow
- Agent review and confirmation workflow

## Workflow design rules

1. A workflow should be auditable end-to-end.
2. A workflow should be resumable after interruption.
3. A workflow should have explicit boundaries of authority.
4. A workflow should be idempotent where feasible.
5. A workflow should be able to degrade gracefully when dependencies are unavailable.
6. A workflow should not hide writes behind fuzzy prompt magic.
7. Human approval points should be first-class, not ad hoc chat interruptions.
8. State transitions should be explicit and inspectable.

# Core Concepts

## Tool vs Skill vs Agent vs Workflow

This distinction is foundational.

## Tool
A tool is a primitive capability exposed by the runtime.

Examples:

- `calendar.read`
- `calendar.write`
- `tasks.read`
- `tasks.write`
- `memory.read`
- `shell.exec`
- `files.read`
- `files.write`
- `repo.search`
- `web.fetch`

A tool is the low-level affordance.

## Skill
A skill is a reusable applied behavior that uses context, tools, and possibly model reasoning to perform a task.

Examples:

- `core/daily-brief`
- `core/task-triage`
- `integrations/google-calendar-reconcile`
- `integrations/todoist-enrich`
- `local/repo-standup`

A skill should have:

- identity
- manifest
- schemas
- execution mode
- capability declarations
- policy requirements
- tests
- metadata for routing and UI

## Agent
An agent is an orchestrator, planner, or executive layer that decides which tools and/or skills to invoke.

An agent may:

- inspect user intent
- select candidate skills
- choose execution order
- decide what context budget to allocate
- handle fallback behavior

A skill system does not require a giant autonomous agent to be useful. But eventually agents should sit on top of the skill runtime, not replace it.

## Workflow
A workflow is an explicit composition of skills and/or tools into a multi-step process.

Examples:

- morning startup pipeline
- project kickoff ritual
- standup generation pipeline
- weekly review pipeline
- metadata enrichment pass

The workflow layer should be able to:

- run sequential steps
- pass structured output between steps
- branch conditionally
- retry steps
- emit audit logs

## Why this separation matters

If a “skill” is just any code or prompt fragment that can do something, you lose:

- policy clarity
- testability
- composability
- inspectability
- reusable routing
- sane UX in CLI and UI

Vel should use these distinctions to build a layered system instead of one giant behavior blob.

## Skill execution modes

### Prompt-only
A skill that primarily renders instructions with context and produces structured output.

Good for:

- classification
- transformation
- summarization
- drafting
- interpretation

### Scripted
A skill implemented entirely by code, often deterministic.

Good for:

- file transforms
- connector orchestration
- batch normalization
- automation glue

### Hybrid
A skill with deterministic hooks plus model-driven execution.

Good for:

- planner-supported workflows
- rich context gathering + semantic reasoning
- artifact generation with validation

Hybrid will likely be Vel’s most common long-term mode.

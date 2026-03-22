# 07. Skills and Workflows

## 7.1 Positioning in the architecture

Skills and workflows should be built **on top of** the canonical object model and Core Tools API.

They are not the foundation. They are governed executable citizens inside the foundation.

## 7.2 Definitions

### Skill
A reusable unit of executable behavior.
Often encapsulates:

- prompt logic
- code hooks
- tool calls
- object-context binding
- output validation

### Workflow
A triggered or manually-invoked orchestrated sequence of actions, skills, and tool calls over typed context.

## 7.3 Why object-first matters here

A workflow such as “morning overview” should run against:

- current `Task`s
- relevant `Event`s
- active `Project`s
- open `Thread`s
- generated `Nudge`s
- user/workspace `Config`

That is much stronger than “call some integrations and improvise.”

## 7.4 Skill manifest expectations

A skill should define:

- metadata
- entrypoints
- input/output schemas
- required capabilities
- context bindings
- limits
- examples
- warning/facts labels

## 7.5 Workflow manifest expectations

A workflow should define:

- trigger model
- scope
- bound object contexts
- steps
- conditions
- retries
- outputs
- audit mode
- policy hints

## 7.6 Context binding model

Skills/workflows should bind explicitly to typed contexts.

Examples:

- project context
- task context
- nudge context
- thread context
- time context
- source context
- user context

## 7.7 Trigger types for workflows

Recommended trigger support:

- manual
- scheduled
- app lifecycle event
- object mutation event
- source sync event
- condition-based trigger
- user action trigger

## 7.8 Example workflow types

- Todoist sync reconciliation
- morning standup generation
- overdue task triage
- thread-to-task extraction
- event preparation nudges
- project weekly review

## 7.9 Execution discipline

Workflows and skills should only act through core tools/API.
They should not directly mutate storage.

## 7.10 Workflow steps

A step may be:

- tool call
- skill invocation
- conditional branch
- fan-out map step
- wait/schedule continuation
- artifact generation
- audit note append

## 7.11 Outputs

A workflow may produce:

- updated objects
- linked objects
- artifacts
- audit records
- summaries
- nudges
- follow-up scheduled runs

## 7.12 Summary recommendation

Skills and workflows should be first-class governed executable objects that operate over typed Vel context via the Core Tools API and are constrained by the same policy system as everything else.

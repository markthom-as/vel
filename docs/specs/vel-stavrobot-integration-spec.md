# Vel Architecture Integration: Stavrobot Patterns

## Purpose
Adopt strong architectural patterns from Stavrobot without copying code (AGPL).

Patterns incorporated:
1. Capability Isolation
2. Tiered Memory
3. Subagent Contracts
4. Plugins vs Skills
5. Self‑Improvement Safety Pipeline

## Vel Runtime Components

vel/
 ├ core/            # orchestration + planning
 ├ executor/        # sandboxed tool runtime
 ├ coder/           # isolated code-editing agent
 ├ memory/          # tiered memory system
 ├ bridges/         # external integrations
 ├ skills/          # procedural playbooks
 └ agents/          # subagent templates

Runtime flow:

core → executor → tool

Core never executes tools directly.

---

## Capability Isolation

Components

vel-core
- planning
- state machine
- policy engine
- suggestion engine
- reflection engine

vel-executor
- tool execution
- plugin sandbox
- permission enforcement
- capability tokens

vel-coder
- repo modification
- code generation
- patch proposals
- self‑improvement

Tools must declare manifests:

tool.yaml

Example:

name: calendar_lookup
permissions:
  - calendar.read
effects:
  - read_only
timeout: 10s

---

## Tiered Memory

### Constitutional Memory

Always loaded.

Examples

- user_preferences
- medications
- chronic constraints
- tone preferences
- safety overrides

Stored in:

memory/constitution.json

---

### Topic Scratchpads

Contextual memory.

Examples

- project_vel
- project_mimesis
- travel_context
- health_tracking

Schema

topic_pad
---------
id
topic
summary
key_entities
recent_events
updated_at

---

### Event Store

Persistent structured logs.

Examples

- reminders
- meetings
- sensor events
- tool calls
- agent actions

Tables

events
facts
actions
suggestions

---

## Subagent Contracts

Subagents must declare:

mission
tool whitelist
TTL
return contract

Example schema

subagent_spec
-------------
id
mission
allowed_tools
max_runtime
memory_scope
return_format

Example

research_agent

mission: gather information
tools: web_search, webpage_reader
ttl: 3 minutes

Return payload

{
 summary
 evidence
 confidence
 suggested_actions
}

Subagents cannot modify state directly.

They propose actions.

---

## Plugins vs Skills

### Plugins

Executable capabilities.

Examples

calendar_lookup
send_signal_message
weather_lookup
filesystem_search

Directory

vel/plugins/

Structure

plugin/
  tool.yaml
  handler.rs

---

### Skills

Procedural playbooks.

Examples

resolve_missed_medication
plan_dinner
daily_reflection
schedule_trip

Directory

vel/skills/

Example

skills/missed_medication.md

Goal:
Determine whether medication was missed and recommend action.

Steps
1 check last intake
2 compute delay
3 classify severity
4 suggest response

Skills guide reasoning.

Plugins perform execution.

---

## Self‑Improvement Pipeline

Vel never edits itself directly.

Pipeline

detect problem
generate proposal
evaluate
replay logs
approve
stage patch
merge

vel-coder responsibilities

- patch proposals
- evaluation suite
- git worktree staging
- approval gate

---

## Executor Sandbox

Executor runs tools in isolation.

Possible implementations

- firecracker
- docker
- wasmtime
- seccomp sandbox

Minimal approach

- separate process
- unix socket API
- capability tokens
- filesystem jail

Executor API

POST /execute

{
 tool
 arguments
 capability_token
}

---

## Observability

Log everything.

Tables

agent_runs
tool_calls
subagent_runs
suggestions
decisions

Enables debugging, replay, reflection.

---

## Reflection Engine

Scheduled introspection.

Tasks

- daily review
- suggestion quality audit
- missed opportunity analysis
- risk prediction accuracy

Outputs improvement suggestions.

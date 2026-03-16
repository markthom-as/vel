# Vel Agent Runtime Spec

## Purpose

Define the runtime contract for Vel agents, subagents, tool execution, memory access, and introspection.

This spec turns Vel from "a smart app with agent-shaped vibes" into an actual agent runtime.

---

## Goals

Vel runtime must support:

- persistent orchestration
- bounded subagents
- capability-scoped tool execution
- structured memory access
- observability and replay
- safe self-improvement
- multi-device coordination across desktop, phone, and watch

---

## Runtime Topology

```text
+-------------------+
|     vel-core      |
| planner/policy    |
+---------+---------+
          |
          v
+-------------------+
|   agent runtime   |
| lifecycle/router  |
+---+-----------+---+
    |           |
    v           v
+------+    +--------+
|memory|    |executor|
+------+    +--------+
               |
               v
           +--------+
           | tools  |
           +--------+
```

Optional peers:

- `vel-coder`
- `vel-bridges`
- remote device runtimes
- reflection scheduler

---

## Core Concepts

### Agent

A stateful worker with:

- identity
- mission
- bounded context
- permitted tools
- memory scope
- lifecycle status

### Subagent

A short-lived agent spawned for a specific task.

Subagents do not own global state.

They return proposals, summaries, or artifacts.

### Run

A single execution instance of an agent.

### Capability Token

A signed, scoped permission allowing a tool call or class of tool calls.

### Artifact

A durable output from an agent run, such as a patch, summary, plan, or message draft.

---

## Agent Lifecycle

States:

```text
created
queued
running
waiting
completed
failed
expired
cancelled
```

Transitions:

- `created -> queued`
- `queued -> running`
- `running -> waiting`
- `waiting -> running`
- `running -> completed`
- `running -> failed`
- `running -> expired`
- `queued -> cancelled`

Rules:

- Every run must have a TTL.
- Every waiting state must record what it is waiting on.
- Expired agents must emit a partial result if possible.
- Failed agents must emit structured failure metadata.

---

## Agent Spec

Each agent must declare a runtime spec.

```yaml
id: research_agent
kind: subagent
mission: gather relevant information about a topic and return structured findings
ttl_seconds: 180
allowed_tools:
  - web.search
  - web.fetch
memory_scope:
  constitution: true
  topic_pads:
    - project_vel
  event_query: limited
return_contract: research_summary_v1
max_tool_calls: 12
max_tokens: 24000
side_effect_policy: propose_only
```

---

## Spawn Contract

Agents are spawned by `vel-core` or an approved supervisor agent.

Spawn request:

```json
{
  "agent_id": "research_agent",
  "mission_input": {
    "topic": "calendar conflict resolution patterns"
  },
  "parent_run_id": "run_123",
  "deadline": "2026-03-15T23:59:00Z",
  "priority": "normal"
}
```

Spawn validation checks:

- valid agent spec
- compatible mission input
- allowed parent-child relationship
- available budget
- safe memory scope
- safe tool set

---

## Return Contract

Every agent type must return a structured payload.

Example:

```json
{
  "status": "completed",
  "summary": "Found three relevant patterns for conflict resolution.",
  "evidence": [
    {"kind": "source", "value": "calendar event store"},
    {"kind": "pattern", "value": "prefer travel-safe buffer windows"}
  ],
  "confidence": 0.81,
  "suggested_actions": [
    {
      "type": "propose_schedule_shift",
      "reason": "reduces conflict risk"
    }
  ],
  "artifacts": [],
  "errors": []
}
```

No free-form sludge as the sole output.

Free text may exist, but it cannot replace the contract.

---

## Runtime Budgets

Each run must enforce budgets.

Dimensions:

- wall clock time
- token budget
- tool call count
- memory query count
- side-effect count
- network access class

Why this matters:

Without budgets, "agentic" quickly becomes a fancy word for resource leak.

---

## Tool Permission Model

Agents do not call tools directly.

They request execution through the executor.

```text
agent -> runtime -> executor -> tool
```

Each request requires:

- run id
- tool name
- arguments
- capability token

Capability token fields:

```json
{
  "subject": "run_123",
  "tool": "calendar.read",
  "scope": "primary_calendar",
  "expires_at": "2026-03-15T22:10:00Z",
  "side_effects": "none"
}
```

Policies:

- tokens are short-lived
- tokens are least-privilege
- write tools require stricter issuance rules
- high-risk actions may require user approval or policy approval
- tokens are logged

---

## Executor RPC Interface

Minimal executor API:

### Execute Tool

```http
POST /execute
```

Request:

```json
{
  "run_id": "run_123",
  "tool": "calendar.read",
  "arguments": {
    "start": "2026-03-16T00:00:00Z",
    "end": "2026-03-17T00:00:00Z"
  },
  "capability_token": "signed-token-here"
}
```

Response:

```json
{
  "status": "ok",
  "result": {
    "events": []
  },
  "usage": {
    "duration_ms": 182
  }
}
```

### Tool Failure

```json
{
  "status": "error",
  "error": {
    "code": "PERMISSION_DENIED",
    "message": "Token does not allow calendar.read"
  }
}
```

Executor guarantees:

- timeout enforcement
- filesystem/network isolation where applicable
- structured errors
- logging for replay
- no direct mutation outside declared tool side effects

---

## Memory Query Contract

Agents access memory through typed queries, not raw database spelunking.

Memory surfaces:

- constitutional memory
- topic pads
- event store
- fact store
- recent agent decisions

Example query:

```json
{
  "run_id": "run_123",
  "surface": "topic_pad",
  "selector": {
    "topic": "project_vel"
  }
}
```

Example response:

```json
{
  "status": "ok",
  "result": {
    "topic": "project_vel",
    "summary": "Vel is evolving toward a chief-of-staff architecture.",
    "key_entities": ["executor", "skills", "subagents"]
  }
}
```

Rules:

- agents get only scoped access
- broad retrieval requires explicit justification
- write paths are separate from read paths
- memory writes require provenance metadata

---

## Memory Write Contract

Writes must include:

- source run id
- write type
- target surface
- provenance
- confidence
- retention hint

Example:

```json
{
  "run_id": "run_123",
  "write_type": "event_append",
  "target": "events",
  "payload": {
    "event_type": "suggestion_shown",
    "suggestion_id": "sug_456"
  },
  "provenance": {
    "source": "suggestion_agent"
  },
  "confidence": 0.97,
  "retention_hint": "normal"
}
```

Suggested rule:

No subagent may write constitutional memory directly.

That would be how the furniture starts rearranging itself at night.

---

## Introspection Hooks

Every run should expose introspection data.

Hooks:

- current objective
- current plan step
- confidence estimate
- blockers
- recent tool calls
- pending decisions
- uncertainty flags

Example runtime introspection snapshot:

```json
{
  "run_id": "run_123",
  "objective": "assess schedule risk for tomorrow morning",
  "plan_step": "checking commute and calendar density",
  "confidence": 0.72,
  "blockers": [],
  "uncertainty_flags": [
    "travel time source stale by 35 minutes"
  ]
}
```

This powers:

- debug UI
- HUD/task view
- trust building
- agent supervision
- human interruption

---

## Uncertainty Escalation

Vel should have first-class uncertainty handling.

When confidence drops below threshold or ambiguity is materially important:

- ask user
- ask supervising agent
- spawn specialized verifier subagent
- downgrade from action to suggestion
- pause execution

Uncertainty classes:

- missing data
- conflicting evidence
- stale context
- policy ambiguity
- tool failure
- low model confidence

Each uncertainty event should be logged.

---

## Multi-Device Runtime Model

Vel may run across:

- desktop
- phone
- watch
- optional server/home node

Recommended model:

### Canonical Runtime

One primary runtime holds durable state and coordination authority.

### Satellite Runtimes

Device-local runtimes handle:

- UI rendering
- short interactions
- local sensors
- notification routing
- cached context

### Sync Model

Use append-only event synchronization with conflict-aware merges.

Rules:

- canonical runtime resolves authoritative agent state
- satellites may cache and propose local events
- watch runtime should be thin and latency-oriented
- expensive inference should usually happen on phone or desktop, not watch

This matches your earlier instincts: don't make the watch pretend it is a datacenter wearing jewelry.

---

## Distributed Agent Graph

Future-facing model:

- local observer agents on devices
- central planner/supervisor
- specialized worker subagents
- coder agent isolated from user-facing runtime

Graph edges must be explicit.

Each edge defines:

- who may spawn whom
- what artifacts may pass
- what memory may be shared
- which tools are legal

Example:

```text
supervisor -> research_agent
supervisor -> commute_risk_agent
reflection_engine -> coder_agent
watch_observer -> notification_agent
```

Disallow arbitrary lateral spawning unless explicitly configured.

---

## Observability Schema

Minimum tables:

### agent_runs

- run_id
- agent_id
- parent_run_id
- status
- started_at
- ended_at
- ttl_seconds
- summary
- confidence

### tool_calls

- call_id
- run_id
- tool_name
- arguments_hash
- started_at
- ended_at
- status
- side_effect_class

### memory_reads

- read_id
- run_id
- surface
- selector
- result_size

### memory_writes

- write_id
- run_id
- target
- provenance
- confidence

### uncertainty_events

- uncertainty_id
- run_id
- class
- description
- resolution

### artifacts

- artifact_id
- run_id
- type
- location
- hash

This is what makes replay, audits, and self-improvement possible.

---

## Replay and Simulation

Vel should support replaying prior runs against frozen context.

Replay uses:

- recorded inputs
- recorded tool outputs or mocks
- frozen memory snapshots
- expected output contracts

Use cases:

- regression testing
- policy evaluation
- self-improvement validation
- postmortems

This is mandatory if Vel is going to learn without becoming haunted.

---

## Failure Semantics

Every failure must be typed.

Failure classes:

- tool_error
- timeout
- permission_denied
- malformed_return
- memory_unavailable
- policy_blocked
- ambiguity_unresolved
- dependency_failure

Failures should include:

- machine-readable code
- human-readable explanation
- retryability flag
- suggested fallback

Example:

```json
{
  "code": "TOOL_TIMEOUT",
  "message": "weather.read timed out after 5s",
  "retryable": true,
  "fallback": "proceed without weather-dependent suggestions"
}
```

---

## Safety Model

Safety hierarchy:

1. hard policy
2. user standing preferences
3. runtime constraints
4. mission goals

Principles:

- suggestions before actions
- least privilege always
- side effects require provenance
- high-risk actions require gating
- user override must be simple
- audit logs must be durable

---

## Self-Improvement Hooks

Runtime must expose structured traces to `vel-coder`.

Allowed outputs to coder:

- repeated failure patterns
- common user corrections
- slow paths
- policy friction
- replayable run traces

Coder must not patch the live runtime directly.

Required pipeline:

1. propose
2. test
3. replay
4. approve
5. stage
6. merge

---

## Recommended File Layout

```text
vel/
  core/
  executor/
  memory/
  coder/
  bridges/
  agents/
    specs/
  skills/
  runtime/
    lifecycle/
    router/
    budgets/
    contracts/
    introspection/
    replay/
  schemas/
    agent_run.schema.json
    return_contracts/
```

---

## Initial Implementation Milestones

See [docs/tickets/agent-runtime/](../tickets/agent-runtime/README.md) for the ticket pack derived from these milestones.

### Milestone 1 — Runtime Skeleton

- agent spec loader
- run lifecycle manager
- spawn validation
- structured return contracts

### Milestone 2 — Executor Integration

- capability token issuance
- executor RPC
- tool call logging

### Milestone 3 — Memory Contracts

- typed read/write APIs
- provenance metadata
- topic pad access controls

### Milestone 4 — Introspection + HUD

- run snapshot endpoint
- uncertainty events
- active task tree
- confidence display

### Milestone 5 — Replay + Reflection

- run replay harness
- reflection jobs
- improvement suggestion feed

---

## Non-Goals

Do not:

- let agents mutate arbitrary global state
- let subagents write constitutional memory
- let tools bypass executor
- let free-form text stand in for runtime contracts
- let self-improvement bypass replay/evals

That is how agent systems become spiritually baroque and operationally stupid.

---

## End State

When this spec is implemented, Vel becomes:

- a real agent runtime
- inspectable
- bounded
- safer to extend
- ready for distributed device surfaces
- capable of reflection without collapsing into mush

# Vel Multi-Client Swarm Spec

Status: Planned orchestration specification  
Audience: coding agent, orchestration implementer, client implementer  
Purpose: define how Vel can run multiple client and agent operations in parallel without forking truth or decision authority

---

## Purpose

Vel already has two important architectural directions:

- one preferred brain, many temporary limbs
- bounded agents with explicit runtime contracts

This spec defines the next layer above those ideas:

> a supervised multi-client swarm that can execute different operations in parallel while preserving one shared task model and one canonical state authority

This is not a generic "many AIs chatting to each other" design.

The goal is practical parallelism for Vel across:

- CLI
- web
- iPhone
- watch
- desktop voice
- bounded subagents
- optional remote compute workers

---

## Goals

The swarm layer must support:

- parallel execution of independent operations
- shared task decomposition
- capability-scoped execution
- deterministic integration of results
- client-specific strengths without client-specific truth
- graceful offline participation by edge clients
- observability, replay, and cancellation

---

## Non-Goals

Do not use the swarm model to introduce:

- multiple competing policy engines
- multiple sources of truth for context, risk, or nudges
- unconstrained peer-to-peer agent chatter
- autonomous self-spawning loops without budgets
- client-owned business logic forks

If multiple workers disagree, they do not vote on truth. They return structured outputs to a supervisor that integrates them against canonical state.

---

## Core Principle

Vel swarm should be designed as:

> **one canonical planner/integrator, many bounded workers**

The planner/integrator lives on a canonical or temporary authority node.

Workers may run on:

- canonical VELD
- desktop compute node
- phone edge client
- watch companion through phone
- remote executor

But workers do not own final truth.

---

## Terminology

### Swarm Task

A top-level user or system objective decomposed into smaller units of work.

Examples:

- prepare morning briefing
- reconcile inbox-like message debt
- produce project synthesis
- investigate travel risk and propose action
- run a code task across multiple specialized workers

### Work Unit

A bounded executable unit assigned to one worker.

### Worker

A concrete execution participant.

Types:

- client worker
- subagent worker
- tool executor worker
- synthesis worker
- retrieval worker

### Supervisor

The orchestrator responsible for:

- decomposition
- assignment
- budget enforcement
- integration
- cancellation
- final result emission

### Integration

The act of combining work-unit outputs into:

- final response
- proposed actions
- durable artifacts
- follow-on tasks

---

## Swarm Topology

```text
user/system trigger
        |
        v
  +-------------+
  | supervisor  |
  | planner     |
  +------+------+ 
         |
         v
  task graph / work units
    |      |       |
    v      v       v
 worker  worker  worker
    |      |       |
    +--- structured outputs ---+
                |
                v
         integration layer
                |
                v
      canonical state / artifacts
```

Optional workers may live on separate devices or runtimes, but the task graph and final integration remain centrally supervised.

---

## Relationship To Existing Specs

This spec extends, and must not contradict:

- `docs/specs/vel-agent-runtime-spec.md`
- `docs/specs/vel-distributed-and-ambient-architecture-spec.md`
- `docs/specs/vel-cluster-sync-spec.md`
- `docs/specs/vel-rust-swift-boundary-spec.md`
- `docs/specs/vel-cognitive-loop-spec.md`

Interpretation rules:

- distributed spec defines where authority and edge behavior live
- cluster sync spec defines how nodes, workers, and authority epochs synchronize
- agent runtime spec defines lifecycle and bounded worker contracts
- this swarm spec defines coordinated parallel execution across those workers

Operational default:

- cross-machine swarm traffic should assume Tailscale-backed connectivity unless the task is same-device or explicitly configured otherwise

---

## Authority Model

The swarm layer must preserve current Vel authority boundaries.

### Canonical planner owns

- top-level task creation
- decomposition
- assignment policy
- dependency resolution
- integration
- writes to canonical state
- final action approval policy

### Workers may own

- retrieval
- summarization
- ranking
- proposal generation
- tool execution
- UI-side rendering tasks
- local capture acquisition

### Workers must not own

- canonical context inference truth
- risk truth
- nudge truth
- durable policy truth
- direct mutation of canonical state outside approved action contracts

---

## Worker Classes

### 1. Retrieval Worker

Purpose:

- gather relevant data
- fetch files/signals/captures/artifacts
- produce evidence bundles

Typical tools:

- search
- storage queries
- API reads

Return contract:

- evidence set
- confidence
- missing-data notes

### 2. Analysis Worker

Purpose:

- evaluate evidence
- propose classifications, rankings, summaries, or risks

Typical tools:

- model inference
- rule evaluation
- bounded memory queries

Return contract:

- analysis payload
- rationale
- uncertainty markers

### 3. Action Worker

Purpose:

- execute approved side effects

Typical tools:

- external integration bridges
- notifications
- file changes
- message sending

Return contract:

- receipt
- status
- errors

### 4. Client Worker

Purpose:

- perform device-specific local work
- collect user input
- execute low-risk queued acknowledgements
- supply local cache context

Examples:

- phone capture worker
- watch acknowledgement worker
- desktop voice capture worker

### 5. Integration Worker

Purpose:

- merge structured worker outputs into one coherent result package

This is usually implemented inside the supervisor, not as a peer worker.

---

## Swarm Task Model

Every swarm run must operate on explicit tasks, not free-form conversational drift.

Required fields for a top-level swarm task:

- `task_id`
- `intent_class`
- `requested_by`
- `priority`
- `deadline`
- `output_spec`
- `side_effect_policy`
- `parent_task_id` optional

Required fields for each work unit:

- `work_unit_id`
- `task_id`
- `worker_class`
- `mission`
- `inputs`
- `dependencies`
- `allowed_tools`
- `memory_scope`
- `ttl_seconds`
- `budget`
- `result_contract`

---

## Dependency Graph

Work units may run in parallel only when dependency-safe.

Allowed parallelism:

- many retrieval units over disjoint sources
- retrieval plus client cache hydration
- independent analysis units over separate evidence bundles
- execution units over disjoint side effects after approval

Must remain serialized:

- integration before all required dependencies are satisfied
- writes to the same canonical object unless idempotent and commutative
- actions whose preconditions depend on unresolved prior writes

The swarm scheduler must treat work as a DAG, not a bag of prompts.

---

## Parallel Execution Rules

### Rule 1: Parallelize evidence gathering first

The easiest safe win is parallel retrieval across:

- captures
- commitments
- signals
- artifacts
- external integrations

### Rule 2: Keep integration centralized

Parallel workers may produce evidence and proposals.

Only the supervisor integrates.

### Rule 3: Side effects require explicit policy

A worker cannot decide to send messages, mutate commitments, or resolve nudges unless the task policy already authorizes that class of effect.

### Rule 4: Client-local work is advisory unless covered by action contracts

Phone, watch, and desktop clients may contribute:

- local captures
- cached state
- queued acknowledgements
- user clarifications

Those contributions become normal structured inputs to the supervisor.

---

## Load Balancing

Parallel execution is not enough.

The swarm scheduler must also place work on the most appropriate available worker without overloading a single node or pushing unsuitable work onto constrained clients.

### Objectives

Load balancing should optimize for:

- deadline success
- responsiveness for interactive tasks
- preservation of battery and thermal budget on edge devices
- use of higher-capacity compute for heavier work
- graceful degradation when preferred workers are unavailable

### Scheduling Inputs

The scheduler should consider:

- worker class
- current load
- queue depth
- recent failure rate
- estimated runtime
- estimated token/tool budget
- network reachability
- device power state
- battery level for phone/watch workers
- thermal pressure for phone/watch/mac workers
- locality of required data
- user interaction urgency
- Tailscale reachability and tailnet path quality

### Worker Capacity Model

Each worker should expose a lightweight capacity descriptor:

- `max_concurrent_units`
- `current_active_units`
- `preferred_work_classes`
- `disallowed_work_classes`
- `latency_class`
- `compute_class`
- `power_class`
- `reachability`

Example classes:

- `compute_class`: edge_low, desktop_medium, server_high
- `power_class`: battery_constrained, plugged_in, server
- `latency_class`: interactive, normal, batch

### Placement Rules

#### Rule 1: Interactive work gets latency priority

User-facing actions such as:

- quick capture
- done/snooze acknowledgement
- short clarification
- current-context refresh

should prefer low-latency nearby workers even if they are not the most powerful.

#### Rule 2: Heavy synthesis prefers compute-rich workers

Expensive tasks such as:

- project synthesis
- multi-source summarization
- broad ranking over many artifacts
- code generation/verification bundles

should prefer desktop or server-class workers.

#### Rule 3: Edge devices should not be saturated

Phone and watch workers must reserve headroom for:

- UI responsiveness
- capture
- notifications
- local action queue handling

Do not schedule background heavy analysis onto a constrained edge device just because it is available.

---

## Scheduler State & Retry/Reclaim Signals

The swarm scheduler needs an explicit queue view and receipt lane to do anything beyond naive dispatch. The new cluster sync surfaces provide that bridge:

- `GET /v1/sync/work-queue` lets the scheduler peek at the backlog for each node/worker-class pair after receipts with terminal states have been removed.
- `POST /v1/sync/work-queue/claim-next` gives workers a bounded scheduler primitive for "claim the next eligible unit" without needing every edge client to reimplement receipt filtering.
- `POST /v1/sync/work-assignments` claims a `work_request_id` and writes a `claimed` receipt; the scheduler must include its `worker_id` and any `queue_depth` it observed so other routers can respect capacity.
- `PATCH /v1/sync/work-assignments` transitions each receipt through `started`, `completed`, `failed`, or `cancelled`. The scheduler observes these transitions to manage retries and to prevent duplicate side effects.
- `GET /v1/sync/work-assignments` offers historical insight for diagnostics and for replaying what timed-out or failed work units previously did.

The scheduler must treat receipts older than the configured stale window (currently 5 minutes) as eligible for reclamation unless `completed` or `cancelled`. Reclaimed work is rerouted, ideally to a worker class that can resolve the failure. Duplicate queue attempts should consult the latest receipt and either reuse it (if it is still `claimed`/`started`) or only enqueue a new unit when the latest receipt is terminal or explicitly failed with a retriable reason. Queue inspection should expose `attempt_count`, `claimable_now`, `claim_reason`, and `next_retry_at` so both the scheduler and operators can explain why a unit is runnable, blocked by backoff, or reclaimable. Per-work-type retry policy should come from runtime configuration, not ad hoc worker code; the current slice reads validation and branch-sync retry bounds from `config/policies.yaml`.

Receipt metadata also powers the supervisor’s retry policy: per-worker failure rates surface from the worker registry, queue depth shows saturation, and the scheduler can decide whether to escalate to clarification or to retry on a higher-capacity worker class. This keeps the DAG scheduler deterministic while still surviving node restarts and operator reconnections.

The runtime wires this into the existing loops subsystem so a background scheduler loop can self-poll `POST /v1/sync/work-queue/claim-next`, mark receipts via `POST /v1/sync/work-assignments` / `PATCH /v1/sync/work-assignments`, and surface `loop` events whenever the queue is empty or retry backoff blocks new claims. Worker self-polling uses the same contract, ensuring local batches obey global retry/backoff state even if the operator interface isn’t open.

#### Rule 4: Data locality matters

If the needed data is already cached locally on a worker, that worker may be preferred for bounded retrieval even if another node has more raw compute.

#### Rule 5: Spillover must preserve policy boundaries

When a preferred worker is overloaded, work may spill to another worker only if:

- the worker has required capabilities
- memory scope is safe
- side-effect policy is compatible
- deadlines remain plausible

Load balancing must not bypass authority or safety checks.

### Load-Shedding Rules

Under pressure, the scheduler should degrade in this order:

1. defer batch/reflective work
2. reduce parallel fan-out
3. switch to cached or lightweight analysis paths
4. skip optional worker branches
5. ask for clarification or return partial results

It should not degrade by silently running high-cost work on a phone/watch that cannot support it.

### Rebalancing

If a worker becomes slow, unreachable, or overloaded mid-run, the supervisor may:

- stop assigning new work there
- retry pending units elsewhere
- reissue idempotent retrieval units
- cancel optional low-priority units

In-flight side-effect units must not be blindly duplicated. They require receipt-aware retry handling.

### Work Unit Receipts & Claims

Each work unit must emit receipts for every lifecycle transition (`claimed`, `started`, `completed`, `failed`, `retriable`). Receipts include the `work_request_id`, worker identity, authority epoch, and timestamp so replay or retry logic can prove what happened. The receipt stream is authoritative for whether a given side effect ran, so planners can avoid duplicate side effects and integrate structured results deterministically.

Workers must claim work before executing it; claims are reflected in the `cluster_workers` heartbeats along with the receipt state. A worker that fails or exits unexpectedly must leave a `failed` receipt plus failure metadata so the supervisor can triage or reassign. Before re-queueing a request, the supervisor examines the last receipt for that `work_request_id` and only retries if the last state is `failed`, `retriable`, or `expired`.

Operator-inspection surfaces (API, CLI) surface the latest receipt states per `work_request_id`, including the last worker that handled it and whether it is still marked `in-flight`. This visibility helps detect stuck assignments, enact manual cancels, or reroute work to other tailnet workers without guessing at what already ran.

### Preferred Placement Order

Default preference should be:

1. canonical or temporary authority for integration and policy-sensitive work
2. local low-latency client worker for immediate interactive device actions
3. desktop/NAS/server worker for heavy analysis or synthesis
4. remote fallback worker when allowed by policy

Exact order may vary by task class, but the scheduler must make that choice explicit.

When multiple remote workers are eligible, workers reachable over the tailnet should be treated as first-class candidates rather than generic remote hosts.

---

## Multi-Client Participation

The swarm model should treat clients as specialized limbs.

### CLI

Best for:

- explicit operator requests
- inspection
- debugging
- high-bandwidth control

### Web

Best for:

- overview
- human review
- structured approval
- artifact inspection

### iPhone

Best for:

- quick capture
- current-context glance
- acknowledgement
- low-friction clarifications

### Watch

Best for:

- interrupt acknowledgment
- ultra-brief state
- one-tap done/snooze

### Desktop Voice

Best for:

- fast command intake
- clarification
- layered explanation

No client becomes its own planner. Each client is a specialized ingress/egress or worker surface.

---

## Offline Edge Participation

Edge clients may participate in the swarm while disconnected, but only in constrained ways.

Allowed offline roles:

- capture creation
- action queueing
- cache-backed evidence contribution
- local acknowledgement

Not allowed offline:

- canonical integration
- final policy decisions
- cross-client coordination authority

When connectivity resumes, offline contributions replay into the canonical swarm event/action log.

---

## Shared Memory Model

Workers should not all read the full universe by default.

Memory access must be scoped into tiers:

- constitutional memory
- task-scoped evidence bundle
- topic pads / thread summaries
- recent events
- canonical object snapshots

The supervisor is responsible for constructing bounded evidence packs.

Workers should consume those packs rather than repeatedly re-querying the world.

This reduces cost, improves determinism, and makes replay tractable.

---

## Result Contracts

Every work unit must return a structured result.

Required envelope:

```json
{
  "work_unit_id": "wu_123",
  "status": "completed",
  "summary": "Found three relevant commitments.",
  "output": {},
  "evidence": [],
  "uncertainty": [],
  "artifacts": [],
  "errors": [],
  "started_at": "2026-03-17T12:00:00Z",
  "finished_at": "2026-03-17T12:00:04Z"
}
```

Free text alone is not acceptable.

If a worker fails, it must return structured failure metadata.

---

## Integration Rules

The supervisor integrates in this order:

1. validate result contracts
2. discard expired or unauthorized outputs
3. reconcile duplicate evidence
4. merge analyses
5. detect conflicts
6. choose final result or clarification path
7. emit artifacts, actions, or user-visible output

Conflict resolution rules:

- canonical state beats stale worker assumptions
- later valid result beats earlier stale result when same unit was retried
- lower-confidence proposal does not overwrite higher-confidence evidence-backed output without explanation
- side-effect receipts are authoritative for whether an action happened

---

## Cancellation And Preemption

The supervisor must support:

- cancelling the entire swarm task
- cancelling a single work unit
- preempting low-priority units when deadline pressure rises
- stopping downstream units when a critical dependency fails

Example:

- if retrieval shows the requested artifact does not exist, dependent synthesis units should not continue blindly

---

## Budget Model

Every swarm task must have explicit budgets for:

- total wall clock time
- total tool calls
- total token usage
- max concurrent workers
- max side effects
- remote compute allowance

Each work unit inherits a sub-budget.

The scheduler must refuse decomposition plans that exceed budget before execution begins.

---

## Observability

The swarm runtime must emit durable events for:

- task created
- work unit spawned
- work unit started
- work unit waiting
- work unit completed
- work unit failed
- work unit cancelled
- integration started
- integration completed
- action executed

Minimum inspection surface:

- top-level swarm task status
- work-unit tree/DAG
- per-unit runtime and outcome
- evidence/artifact links
- final integration summary

If the swarm cannot be inspected, it will become an un-debuggable source of ghost behavior.

---

## Safety Rules

### Rule 1: No peer authority drift

Workers do not negotiate truth with each other directly.

### Rule 2: No hidden side effects

All side effects must emit receipts and durable events.

### Rule 3: No unlimited self-spawn

Workers may request sub-work, but supervisor approval is required.

### Rule 4: No business-logic forks in clients

Phone and watch clients may participate, but they must not become independent planning engines.

### Rule 5: Clarify before high-consequence execution

If integrated results still leave uncertainty around a high-risk action, the supervisor must request clarification instead of guessing.

---

## Example Swarm Patterns

### 1. Morning briefing swarm

Parallel units:

- fetch current context
- fetch open commitments
- fetch active nudges
- fetch top recent captures
- optional lightweight summarizer

Integration result:

- one coherent morning briefing artifact and response

### 2. Message debt swarm

Parallel units:

- retrieve pending message threads
- retrieve related commitments
- classify urgency
- draft candidate replies

Integration result:

- ranked message-debt review with proposed actions

### 3. Code task swarm

Parallel units:

- repo search worker
- test-impact worker
- bounded code-edit worker
- verification worker

Integration result:

- patch proposal plus verification report

### 4. Travel-risk swarm

Parallel units:

- fetch calendar event
- fetch travel estimates
- fetch prep commitments
- compute risk proposal

Integration result:

- one recommendation, optionally with clarification request

---

## Recommended Initial Implementation Order

1. Add explicit top-level `SwarmTask` and `WorkUnit` domain types.
2. Reuse existing agent runtime lifecycle and return-contract ideas for work units.
3. Implement a DAG scheduler with bounded parallel retrieval first.
4. Add supervisor-side integration and conflict handling.
5. Add inspection surfaces for swarm status and work-unit detail.
6. Only then let edge clients and remote workers participate in the same swarm protocol.

Do not start with autonomous many-agent chatter.

---

## Acceptance Criteria

The first acceptable swarm implementation satisfies all of these:

1. One top-level task can spawn multiple bounded work units with explicit dependencies.
2. Independent work units can run in parallel.
3. Every work unit returns a structured contract.
4. Only the supervisor integrates and writes canonical results.
5. Edge clients can contribute queued actions or local evidence without becoming planners.
6. The full swarm execution is inspectable and replayable.
7. Side effects remain capability-scoped and receipt-backed.

That is the correct Vel shape for a multi-client swarm.

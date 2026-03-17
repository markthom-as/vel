# Vel Future Architecture Map

Status: Planned architecture synthesis  
Audience: maintainers, architecture work, coding agents  
Created: 2026-03-17  
Updated: 2026-03-17  
Purpose: consolidate the aspirational architecture across specs and ticket packs into one bounded future system map and a practical service-subdivision plan.

## 1. How To Use This Document

This document is the future-facing companion to [docs/architecture-inventory.md](architecture-inventory.md).

Use them together:

- [docs/architecture-inventory.md](architecture-inventory.md) describes current truth, drift, and immediate decomposition pressure
- this document describes the intended future system shape so service and crate subdivision can move toward one coherent runtime instead of many partial architectures

This document does not override [docs/status.md](status.md) for shipped behavior.

It also does not rename the current codebase.

When this document uses service names such as `context_service`, `agent_runtime_service`, or `project_workspace_projection_service`, those are target architectural boundaries, not claims that those exact modules or crates already exist today.

### 1.1 Current-code anchor points

This future map must stay compatible with the current runtime shape:

- `vel-core` remains the domain-semantics boundary
- `vel-storage` remains the persistence boundary
- `vel-api-types` remains the transport DTO boundary
- `veld` remains the canonical runtime and orchestration process
- route handlers are still thin HTTP boundaries over `veld` services
- current context remains the shipped present-tense authority
- run lifecycle and work-receipt lifecycle are both real shipped control-plane primitives

If a proposed future boundary contradicts those current anchors, that proposal should be treated as wrong or premature until `docs/status.md` says otherwise.

### 1.2 Active-work alignment

This document should be read together with the active convergence and expansion lanes that are already shaping the repo:

- [docs/tickets/repo-audit-hardening/README.md](tickets/repo-audit-hardening/README.md) for architecture cleanup, module extraction planning, and doc-truth work
- [docs/tickets/repo-feedback/README.md](tickets/repo-feedback/README.md) for current convergence and boundary correction work
- [docs/tickets/multi-client-swarm/README.md](tickets/multi-client-swarm/README.md) for planned distributed execution and cluster orchestration
- [docs/specs/vel-cluster-sync-spec.md](specs/vel-cluster-sync-spec.md) and [docs/specs/vel-multi-client-swarm-spec.md](specs/vel-multi-client-swarm-spec.md) for the still-planned distributed/runtime direction

The map is only useful if it helps those active workstreams converge on one architecture rather than fork into parallel partial designs.

## 2. Core Future Principle

Vel should converge on:

- one canonical Rust runtime
- many specialized surfaces
- provider-aware integration edges
- one bounded execution model under one supervisor
- one authoritative present-tense context model

The planned system is coherent only if the repo avoids creating parallel authorities for:

- context
- uncertainty
- orchestration
- client behavior
- integration semantics
- self-improvement

## 3. Target System Shape

### 3.1 Top-level future planes

The planned future system reduces cleanly into seven planes:

1. present-tense cognition runtime
2. supporting epistemic and uncertainty layer
3. agent runtime and execution control plane
4. integration substrate
5. project and session control plane
6. surface projection layer
7. reflection, self-knowledge, and governed self-modification

### 3.2 Canonical top-level flow

```text
Sources / Clients
    ->
Integration substrate
    ->
Canonical signals / events / source refs
    ->
Current context runtime
    ->
Risk / policy / nudges / suggestions
    ->
Project + session control plane
    ->
Surface projections
    ->
Clients / operators / notifications / voice

In parallel:
execution control plane
    ->
bounded workers / agents / tools
    ->
artifacts / receipts / decision traces

And above that:
self-knowledge / replay / reflection / governed self-modification
```

## 4. Future Subsystems

### 4.1 Present-tense cognition runtime

This remains the current authoritative brain.

It should own:

- canonical signals and normalized facts
- commitments and dependencies
- current context
- risk
- policy and nudge lifecycle
- suggestions
- threads
- synthesis

Guardrail:

- nothing else should become a second present-tense truth engine

### 4.2 Supporting epistemic and uncertainty layer

This supports the runtime without replacing it.

It should own:

- confidence metadata
- decision traces
- assumptions
- uncertainty records and ledgers
- clarification and resolver policy
- retrieval and validation routing

Guardrails:

- beliefs must remain subordinate to `current_context`
- uncertainty and context reasoning must share one confidence vocabulary
- decision traces must be shared primitives, not per-subsystem one-offs

### 4.3 Agent runtime and execution control plane

This is the bounded execution substrate for agents, subagents, workers, and tools.

It should own:

- runtime specs
- run lifecycle
- spawn validation
- capability-scoped tool execution
- budgets and TTLs
- waiting states and return contracts
- bounded worker supervision
- task and work-unit orchestration

Unification rule:

- agents, Navs, workers, and subagents should collapse into one supervised execution model
- they are different roles in one runtime, not separate planner species

Current-code alignment:

- this future plane must extend the shipped run lifecycle, retry lifecycle, worker loop machinery, and queued-work receipt model instead of inventing a second execution ledger
- execution control should converge around shared receipts, capabilities, decision traces, and durable artifacts rather than one-off route-specific control paths

### 4.4 Integration substrate

This is the provider-aware edge of the system.

It should own:

- integration families
- providers
- connections
- capability registry
- canonical source object references
- people and external identity graph
- sync state and sync history
- provider-specific normalization and writeback modules
- host capability contracts for client-specific affordances

Canonical model:

- family
- provider
- connection
- capability contract
- host adapter

Guardrail:

- treat “plugin” as packaging, not ontology
- the family/provider/connection model is the real substrate
- do not bind integration semantics to a single client host
- clients should advertise capabilities such as auth handoff, notifications, local file access, background sync, and device-native capture
- the same integration family should be runnable across desktop, mobile, web, CLI, or server-hosted surfaces when the required capabilities exist

Current-code alignment:

- the future substrate must remain compatible with the shipped family/provider/connection direction already visible in `vel-core::integration` and the partially shipped cluster/sync surfaces described in [docs/status.md](status.md)
- provider-specific work should continue to normalize into canonical signals, commitments, refs, and context inputs rather than bypassing the runtime with ad hoc client-owned state

### 4.5 Project and session control plane

This is the future operator-facing work substrate.

It should own:

- project registry
- commitment-backed task workspace
- external task mappings
- agent session registry
- outbox, steering, and feedback controls
- shared web/CLI workspace projections

Guardrails:

- commitments remain the canonical task truth
- transcripts remain evidence, not the operator abstraction
- sessions become the operator abstraction for active work across Vel and external agents

### 4.6 Surface projection layer

These are read and mutation surfaces built over already-computed state.

They should own:

- web projections
- CLI projections
- iPhone/watch/mobile projections
- notifications and widget projections
- voice projections

Guardrails:

- projections shape state, they do not re-derive policy
- clients may cache and queue mutations, but may not own context/risk/policy logic

### 4.7 Reflection, self-knowledge, and governed self-modification

This is the future meta-runtime.

It should own:

- repo/doc/code/test/schema awareness
- knowledge graph and evidence model
- drift detection
- freshness/change intelligence
- replay harness
- reflection engine
- protected surface registry
- patch proposal lifecycle
- validation orchestration
- sandbox execution
- rollback and approval

Separation rule:

- self-knowledge observes, indexes, explains, and detects drift
- self-modification proposes, validates, requests approval, applies, and rolls back

## 5. Planned Service Subdivision

These are target seams for future decomposition.

They are not evidence that the current repo already has one-file-per-service or one-crate-per-service boundaries today.

### 5.1 Services that should remain in `veld` for a long time

These are canonical-brain responsibilities and should not fork early:

- `context_service`
- `risk_service`
- `policy_service`
- `suggestion_service`
- `thread_service`
- `project_workspace_projection_service`
- `agent_runtime_service`
- `spawn_validation_service`
- `sync_authority_service`
- `state_publication_service`
- `result_integration_service`

### 5.2 Services that should become internal modules first

These are clear boundaries, but should stabilize inside `veld` before crate or process extraction:

- `chat_service`
- `explain_service`
- `decision_trace_service`
- `uncertainty_service`
- `resolver_service`
- `integration_registry_service`
- `person_resolution_service`
- `metadata_enrichment_service`
- `agent_session_service`
- `session_control_service`
- `notification_projection_service`
- `voice_projection_service`

### 5.3 Translation to today’s code

Today, many of these future service seams still live inside broader modules under `crates/veld/src/services`, `crates/veld/src/routes`, `crates/veld/src/worker.rs`, and the large DTO/storage files noted in [docs/architecture-inventory.md](architecture-inventory.md).

That means the practical use of this map is:

- clarify target ownership before extraction
- prevent new code from deepening current file concentration
- keep active cleanup tickets pointed toward one future shape
- avoid claiming that planned subsystem names are already shipped module boundaries

### 5.4 Services that are good later crate candidates

After contracts are exercised and stable:

- `executor`
- `memory_contracts`
- `self_knowledge`
- `replay_reflection`
- `worker_registry`
- `placement_engine`
- `metadata_enrichment`

### 5.5 Services that are likely separate processes later

Not early, but eventually defensible:

- sandboxed executor
- heavy compute worker runtime
- self-improvement execution lane
- remote bridge runners for specialized integrations

## 6. Suggested Future Module / Crate Families

These are future families, not immediate repo moves.

### 6.1 Canonical runtime family

- `vel-core`
- `vel-storage`
- `vel-api-types`
- `veld`

This remains the center of gravity until internal contracts are genuinely stable.

### 6.2 Candidate future internal families

- `vel-context-runtime`
- `vel-uncertainty`
- `vel-agent-runtime`
- `vel-executor`
- `vel-self-knowledge`
- `vel-self-modification`
- `vel-integrations`
- `vel-projects`
- `vel-memory-contracts`
- `vel-replay`

Guardrail:

- treat these as future subdivision hints, not current truths
- do not rewrite the repo to match these names before the contracts demand it

## 7. Integration and Client Boundary Rules

### 7.1 Keep in core runtime

- context reduction
- risk and policy
- suggestion logic
- thread and project/session semantics
- provider and connection registry
- integration capability contracts
- person identity resolution
- metadata enrichment decisions
- provenance and explainability

### 7.2 Keep in clients

- UI
- notifications
- haptics and action affordances
- App Intents and Shortcuts
- device-native capture
- HealthKit/EventKit/watch connectivity
- local cache and optimistic mutation queue
- widgets and complications
- client host adapters for capabilities exposed to the integration substrate

### 7.3 Shared client guardrails

- clients may cache and project
- clients must not own business logic for context, risk, or policy
- clients must not become the only place an integration can run unless the capability is inherently device-bound
- integration APIs and domain types must stay host-agnostic and capability-based
- watch/widget/mobile surfaces must consume backend-shaped state instead of deriving domain rules locally
- HTTP/API boundary remains the default before embedded Rust

## 8. Major Overlap Hotspots That Need Unification

### 8.1 Agents vs Navs vs workers

Planned docs currently describe:

- agents and subagents
- Navs
- workers and supervisors

Target unification:

- one task/work-unit model
- one supervisor
- one bounded execution lifecycle
- different worker kinds within one runtime

### 8.2 Beliefs vs uncertainty

Planned docs currently describe:

- belief stores
- decision traces
- uncertainty ledgers
- assessments and assumptions

Target unification:

- one shared confidence model
- one decision record vocabulary
- belief and uncertainty artifacts layered around the same authoritative context runtime

### 8.3 Self-knowledge vs self-modification

Target unification:

- self-knowledge observes and explains
- self-modification consumes that evidence to decide whether to propose and validate changes

### 8.4 Task truth vs external providers

Target unification:

- external task providers feed commitments
- projects and sessions organize work
- no second durable task authority emerges

### 8.5 Metadata enrichment vs suggestions/policy

Target unification:

- enrichment becomes a specialized proposal and writeback subsystem
- it uses the same approval, audit, and explainability principles as other suggestions
- it does not become a parallel decision constitution

## 9. Sequencing For Service Subdivision

### 9.1 Near-term

Use the current runtime and extract boundaries inside `veld`:

1. route -> service cleanup
2. storage modularization
3. projection services
4. integration registry and provider modules
5. project/session services

### 9.2 Mid-term

Once current contracts stabilize:

1. uncertainty and decision-trace primitives
2. self-knowledge read path
3. executor boundary
4. replay and reflection
5. metadata enrichment subsystem

### 9.3 Later

Only after replay, evidence, and guardrails are real:

1. governed self-modification
2. sandboxed apply path
3. heavier worker placement and swarm scheduling
4. optional process extraction for compute-heavy lanes

## 10. Non-Negotiable Guardrails

- `current_context` remains the authoritative present-tense state object
- one confidence model across context, uncertainty, and self-modification
- one supervisor model across agents, Navs, and workers
- one task truth centered on commitments
- one provider ontology centered on family/provider/connection
- one runtime decision authority in Rust
- clients remain thin in business-logic authority
- replay and evidence must exist before autonomous self-improvement

## 11. Short Conclusion

The aspirational architecture is coherent if the repo keeps compressing toward one bounded runtime with layered adjunct systems.

The future system should be:

- one canonical runtime
- one execution model
- one integration substrate
- one project/session control plane
- many surface projections
- one evidence-first self-knowledge layer
- one governed self-modification lane

The main architectural risk is not lack of ideas. It is allowing planned subsystem names and partially overlapping specs to harden into multiple parallel authorities before the shared contracts exist.

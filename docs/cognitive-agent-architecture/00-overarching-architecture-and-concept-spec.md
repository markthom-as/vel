---
title: Overarching Architecture And Agentic Concept Spec
doc_type: spec
status: proposed
owner: staff-eng
created: 2026-03-17
updated: 2026-03-17
keywords:
  - architecture
  - concept
  - agentic
  - orchestration
  - security
index_terms:
  - overarching architecture
  - concept spec
  - agentic engineering principles
  - capability broker
  - execution traces
related_files:
  - docs/MASTER_PLAN.md
  - AGENTS.md
  - docs/templates/agent-implementation-protocol.md
  - docs/cognitive-agent-architecture/README.md
  - docs/cognitive-agent-architecture/01-cross-cutting-system-traits.md
  - docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md
  - docs/cognitive-agent-architecture/integrations/canonical-data-sources-and-connectors.md
  - docs/cognitive-agent-architecture/cognition/self-awareness-and-supervised-self-modification.md
summary: Consolidated architectural and operational principles for how Vel should orchestrate agents, mediate capabilities, verify behavior, and evolve without sacrificing trust.
---

# Purpose

Define the durable concept for Vel as a local-first cognition runtime with supervised agent execution.

This document is the architectural bridge between the product idea, the cognitive architecture pack, and the implementation ticket queue.

# Problem

Vel has strong local-first and trust-oriented instincts, but the repo has accumulated planning material from multiple directions:

- cognition and orchestration notes,
- implementation tickets,
- agent workflow guidance,
- partial distributed-runtime work,
- security ideas that are implied rather than made explicit.

Without one explicit concept spec, the ticket queue can drift into disconnected workstreams.

# Goals

- keep Vel local-first, inspectable, and reversible
- keep the main authority process small, explicit, and trustworthy
- use agents as supervised specialists, not ambient omnipotent actors
- mediate external capabilities and secrets through narrow boundaries
- require execution-backed verification instead of prompt-only confidence
- define schemas, manifests, and templates before broad implementation drift can form around implicit contracts
- make repo-aware introspection and bounded self-modification explicit, reviewable, and narrow
- make architectural learning compound over time through docs, traces, and reusable patterns

# Non-Goals

- maximizing agent autonomy at the expense of user control
- handing broad raw credentials to models by default
- building a sprawling distributed system before the local authority runtime is solid
- assuming speculative multi-agent complexity is inherently better than a strong single orchestrator
- treating LLM judgment alone as sufficient verification
- allowing open-ended self-rewriting without explicit writable scope, review, and verification

# Current State

Current implementation truth lives in [docs/MASTER_PLAN.md](../MASTER_PLAN.md).

Today the repo is in a mixed state:

- strong local-first and inspectability principles are visible,
- services, routes, and storage boundaries are still mid-decomposition,
- distributed agent surfaces exist in partial form,
- the doc and ticket packs are stronger than the runtime in some areas.

This spec defines the target direction that active tickets should converge toward.

# Proposed Design

## Core Concept

Vel is a **local authority runtime** that owns durable cognition state, policy evaluation, and execution supervision.

Other actors are subordinate:

- clients are operator or capture surfaces,
- integrations are mediated capability providers,
- subagents are bounded workers,
- external runtimes are leased execution environments,
- docs and evals are part of the control plane, not decoration.

## Primary Architectural Principles

### 1. Local-First Authority

- The authority node owns canonical local state and policy decisions.
- Core capture, recall, review, and context loops must work without remote infrastructure.
- Remote services are optional capability providers, not architectural prerequisites.

### 2. Trust Through Inspectability

- Every meaningful system decision should be explainable from persisted inputs, policy rules, and run events.
- User-visible failures must leave inspectable traces.
- High-impact behavior must be reversible or explicitly review-gated.

### 3. Single Orchestrator By Default

- Start with one orchestrating authority flow.
- Introduce specialized subagents only when they have:
  - a clear responsibility,
  - a bounded write surface,
  - explicit tool and capability scopes,
  - logged handoffs,
  - verifiable outputs.
- “One giant model with every tool” is the default anti-pattern.

### 4. Capability Mediation Over Raw Access

- Agents should not hold raw third-party credentials when a brokered capability can perform the action.
- Prefer scoped tokens, host/path-scoped permissions, and boundary-time injection over prompt-visible secrets.
- Unknown or unmatched external requests should fail closed.

### 5. Explicit Trust Domains

Vel has multiple trust domains:

- authority runtime,
- storage boundary,
- client surfaces,
- tool and integration boundaries,
- delegated agent sandboxes,
- future plugin or WASM sandboxes.

Crossing trust domains requires an explicit contract, audit trail, and narrow capability scope.

### 6. Boundary-Owned Error Handling

- Let errors propagate to the boundary that can map them correctly.
- Services should not silently swallow failures unless a documented degraded-mode path exists.
- The system should prefer honest degraded behavior over fake success.

### 7. Execution-Backed Confidence

- Agent output is draft output until it has been executed, tested, or manually checked.
- The workflow should prefer:
  - baseline tests first,
  - red/green TDD for logic changes,
  - direct execution for API and UX flows,
  - small reviewable diffs,
  - evidence in summaries.

### 8. Compounding Engineering

- Reuse known-good patterns from the repo before inventing new ones.
- Capture successful prompts, walkthroughs, fixtures, and verification patterns so future work starts from a higher baseline.
- Reduce cognitive debt in hard areas with concise walkthroughs and linear explanations.

### 9. Cross-Cutting Traits Are Mandatory

- Modularity, accessibility, configurability, data logging, rewind/replay, and composability are repo-wide architecture traits, not per-team preferences.
- These traits should be accounted for explicitly in specs, tickets, and subsystem changes.
- See [01-cross-cutting-system-traits.md](01-cross-cutting-system-traits.md) for the formal trait definitions and subsystem application matrix.

### 10. Canonical Contracts Before Breadth

- Core data shapes, manifests, config schemas, and policy templates should be explicitly defined before implementation fans out across clients, connectors, and workers.
- Object definitions should name the owning crate, serialization boundary, versioning rule, and canonical example or template.
- The queue should prioritize documentation, schema, contract, and architecture work before broadening implementation scope.

### 11. Bounded Self-Awareness

- Vel should be able to inspect its state, docs, config, tickets, and repository layout as part of supervised introspection.
- Repo visibility is useful only when paired with explicit writable scopes, diff visibility, and verification gates.
- Self-modification should be a supervised capability, not an ambient right of every agent or runtime.

## System Shape

### Authority Runtime

The authority runtime should own:

- canonical state,
- context reduction,
- policy checks,
- suggestion generation,
- execution supervision,
- run events, traces, and artifacts.

### Client Surfaces

Clients should remain thin:

- capture input,
- present current state,
- surface explanations and controls,
- submit reviewed actions back to the authority.

### Capability Broker Layer

External integrations and future tools should be mediated through a capability layer that:

- authenticates the caller,
- evaluates scoped permissions,
- resolves the narrow resource or host/path allowance,
- injects credentials only at point of use,
- logs the action with stable run or trace identifiers.

### Contract And Schema Layer

The authority runtime should keep a canonical contract layer for:

- typed domain objects,
- transport DTOs,
- config schemas,
- policy schemas,
- handoff envelopes,
- connector manifests,
- repo-visible self-models.

Those contracts should have named owners, versioning rules, and templates or examples.

### Self-Model And Introspection Layer

Vel should maintain a bounded self-model that can answer:

- what code and docs exist,
- what contracts and configs govern runtime behavior,
- what writable scopes are currently allowed,
- what tickets or plans constrain the active task,
- what changes are safe to propose versus safe to apply.

### Delegated Agent Runtime

Delegated workers should run only with:

- explicit leases,
- explicit tool allowlists,
- no self-escalation path,
- isolated execution environments,
- observable lifecycle events,
- termination rules on timeout, heartbeat failure, or policy violation.

## Cognitive Workflow Pattern

Preferred high-level loop:

```text
signal or operator trigger
-> context synthesis
-> policy/risk evaluation
-> candidate action or suggestion
-> policy gate
-> surface or execution selection
-> delivery or supervised execution
-> feedback capture
-> trace, artifact, and learning capture
```

## Data And Contract Rules

- Domain types live in `vel-core`.
- Transport DTOs stay at the API and client boundary.
- `current_context` should be typed and versioned, not an unbounded JSON blob in business logic.
- Run-backed operations must emit lifecycle events and persist terminal state.
- Handoffs must carry objective, constraints, expected output shape, and trace linkage.
- Configs, manifests, and policy files should have canonical schema docs and checked-in templates.
- Integration families, providers, and source modes should come from one canonical connector model rather than ad hoc per-surface lists.

## Security And Secret Rules

- New routes and high-impact tool paths should require auth or capability gating by default.
- Public surfaces should be rare, explicit, and documented.
- Raw secrets must not appear in prompts, logs, traces, fixtures, or snapshots.
- Secret decryption should happen at the narrowest point of use.
- Unsupported or unknown routes, actions, and request patterns should reject safely by default.
- Repo-aware agents must not infer write permission from read permission; writable scope must be explicit.
- Self-modification paths should require diff visibility, tests or execution evidence, and a review gate.

## Verification And Evaluation Rules

- Deterministic replay and simulation should verify long-running cognition behavior.
- LLM evals may grade reasoning quality, but they do not replace deterministic checks or execution-backed verification.
- New agentic flows should produce observable traces, run IDs, or equivalent event linkage.
- Manual execution remains necessary for high-value web, API, and integration flows.

# Boundaries

## Layer Ownership

- `vel-core`: domain semantics and invariants
- `vel-storage`: repositories, transactions, and durable persistence
- `veld` routes: request parsing, auth, service invocation, DTO mapping
- `veld` services: application logic and orchestration
- clients: presentation and operator workflow
- delegated runtimes: bounded execution only through explicit contracts

## Responsibility Split

- Authority decides.
- Workers propose or execute within bounds.
- Clients present and collect operator intent.
- Integrations provide capability, never system truth by themselves.

# Operational Considerations

- observability must cover external calls, tool invocations, handoffs, run transitions, and degraded modes
- the queue should prefer foundational safety and boundary tickets before speculative autonomy work
- docs are part of the control surface; stale authority pointers create real implementation risk
- each new agentic surface should document its trust domain, failure modes, and capability boundary

# Acceptance Criteria

1. The active architecture and ticket queue both reference this concept directly or align with its principles.
2. Missing queue coverage for capability mediation, trust boundaries, and execution observability is repaired.
3. New agent-related work uses explicit scopes, traces, and review gates instead of ambient authority.

# Related Terms

- canonical name: local authority runtime
- aliases: authority node, brain, orchestrator
- related packs or subsystems: Connect, sync, evals, WASM sandboxing, user trust, integrations

# Search Terms

- overarching architecture
- concept spec
- capability mediation
- fail closed
- execution-backed verification
- bounded subagents

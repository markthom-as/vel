---
title: Cross-Cutting System Traits
doc_type: spec
status: proposed
owner: staff-eng
created: 2026-03-17
updated: 2026-03-17
keywords:
  - modularity
  - accessibility
  - configurability
  - logging
  - replay
  - composability
index_terms:
  - cross-cutting traits
  - quality attributes
  - system traits
  - rewind and replay
  - observability baseline
related_files:
  - docs/cognitive-agent-architecture/00-overarching-architecture-and-concept-spec.md
  - docs/cognitive-agent-architecture/architecture/cross-cutting-trait-audit.md
  - docs/MASTER_PLAN.md
  - docs/templates/spec-template.md
  - docs/templates/ticket-template.md
summary: Repository-wide quality traits and abstraction rules that every Vel subsystem should account for: modularity, accessibility, configurability, logging, rewind/replay, and composability.
---

# Purpose

Define the cross-cutting system traits that all Vel subsystems must consider, regardless of language, package, or runtime boundary.

These traits are not optional polish. They are structural quality attributes that guide how systems should be designed, abstracted, and reviewed.

# Problem

Vel already values trust, local-first behavior, and inspectability, but those values are not enough on their own to drive consistent engineering decisions across:

- Rust crates,
- client surfaces,
- integrations,
- visual packages,
- docs and planning artifacts,
- future agent runtimes.

Without explicit trait definitions, different parts of the repo can optimize for convenience in incompatible ways.

# Goals

- make these traits explicit and reusable across the whole repo
- provide stable abstraction guidance for each trait
- define what “good enough” looks like per subsystem
- force new specs and tickets to account for trait impact

# Traits

## 1. Modularity

### Definition

Systems should be decomposed into narrow units with clear ownership, replaceable seams, and limited reasons to change.

### Required Abstractions

- stable module or package boundaries
- narrow interfaces between layers
- minimal cross-layer knowledge
- single-responsibility services and components

### Required Behaviors

- domain logic does not leak transport or storage concerns
- subsystems can be tested in isolation
- new features extend seams instead of expanding god modules

## 2. Accessibility

### Definition

Vel should be accessible to both humans and machines:

- human accessibility for web, Apple, CLI, and docs surfaces
- machine accessibility for APIs, configs, logs, and typed outputs

### Required Abstractions

- semantic UI structure
- keyboard and assistive-tech friendly interaction patterns
- plain-text and structured operator surfaces
- predictable machine-readable data formats

### Required Behaviors

- web surfaces should support keyboard navigation, semantic labels, and reduced reliance on color alone
- Apple surfaces should respect platform accessibility affordances
- CLI and logs should remain parseable and readable without hidden context
- docs should describe real behavior with clear reading order and direct language

## 3. Configurability

### Definition

Behavior that legitimately varies by environment, operator choice, or integration state should be explicitly configurable and inspectable.

### Required Abstractions

- typed config structures
- explicit defaults
- effective-config inspection surfaces
- scoped feature toggles or policy parameters where appropriate

### Required Behaviors

- avoid magic behavior controlled only by hidden environment assumptions
- default values should be safe and documented
- configuration should be inspectable from an operator surface
- config should not sprawl into ad hoc per-module parsing logic

## 4. Data Logging and Observability

### Definition

Meaningful system activity should leave structured, safe, inspectable records.

### Required Abstractions

- structured logs
- run events or audit events
- stable identifiers such as request IDs, run IDs, and trace IDs
- explicit degraded-mode and denial records

### Required Behaviors

- high-value boundaries should emit logs or events
- raw secrets must never be logged
- logs should support operator inspection and regression debugging
- important denials and failures should be recorded, not silently dropped

## 5. Rewind / Replay

### Definition

Where behavior depends on event sequences, time, or derived state, the system should support deterministic inspection, replay, or idempotent recovery.

### Required Abstractions

- append-only event history where practical
- snapshots or durable checkpoints
- replayable queues or event feeds
- deterministic test harnesses and fixtures

### Required Behaviors

- important workflows should be reproducible from durable records
- offline or queued actions should support safe replay
- derived state should be regenerable where architecture permits
- replay paths should be used for verification, not treated as theoretical only

## 6. Composability

### Definition

Systems should combine through explicit contracts so capabilities can be reused, rearranged, or replaced without rewriting the whole stack.

### Required Abstractions

- narrow contracts and manifests
- typed inputs and outputs
- capability descriptors
- reusable service, component, or package boundaries

### Required Behaviors

- new behavior should be built by composing existing seams when possible
- clients, integrations, and runtimes should interact through contracts instead of ad hoc shared assumptions
- composability should not come from “everything imports everything”

# Subsystem Application Matrix

## `vel-core`

- modularity: domain-only types and invariants
- configurability: typed policy and config-related domain types when needed
- replay: deterministic types and ordering semantics
- composability: reusable domain contracts with no transport leakage

## `vel-storage`

- modularity: repository and transaction seams
- logging: durable run and event records
- replay: append-only or replayable records where workflows depend on sequence
- configurability: backend and retention policies through explicit config, not hidden branches

## `veld` Runtime and API

- modularity: thin routes and service-owned application logic
- accessibility: predictable DTOs, useful errors, operator-readable inspect surfaces
- configurability: typed config, explicit feature gates, inspectable effective config
- logging: request IDs, run IDs, traces, denials, degraded modes
- replay: idempotent actions, replay-friendly workflows, deterministic test seams
- composability: services and capability brokers behind explicit contracts

## Clients: Web, Apple, CLI

- accessibility: semantic and assistive-friendly UI, readable CLI output, direct docs links
- configurability: inspectable endpoint and surface configuration
- replay: durable offline queues and safe retry/replay rules
- composability: thin surfaces over shared contracts, not policy forks

## Integrations and Tooling

- modularity: provider-specific adapters behind stable contracts
- configurability: explicit auth/source config and freshness policy
- logging: sync logs, denial records, degraded-state reporting
- replay: safe resync and dedupe semantics
- composability: capability declarations, manifests, and scoped execution

## Agent Runtimes and Future Sandboxes

- modularity: role-specific bounded workers
- configurability: declared capabilities and policy gates
- logging: trace-linked handoffs and execution events
- replay: inspectable run histories and deterministic eval fixtures
- composability: explicit manifests, host ABIs, and handoff envelopes

## Docs, Specs, and Templates

- accessibility: clear reading order and direct language
- configurability: templates that expose important choices instead of hiding them
- logging: documented authority chain and decision records
- replay: reproducible walkthroughs and command-backed examples
- composability: shared templates and consistent sections across docs

# Review Rules

When adding or reviewing a subsystem change, explicitly ask:

1. What module seam or contract owns this behavior?
2. How does a human or machine access and understand it?
3. What is configurable, and how is the effective value inspected?
4. What records exist when it runs, fails, or is denied?
5. Can it be replayed, retried, or reconstructed?
6. How does it compose with existing services, clients, and runtimes?

# Acceptance Criteria

1. New architecture docs and tickets reference or apply these traits explicitly.
2. Templates require authors to account for trait impact rather than assuming it.
3. The ticket queue contains explicit coverage for currently missing trait work.

# Current Baseline Audit

Use [architecture/cross-cutting-trait-audit.md](architecture/cross-cutting-trait-audit.md) as the current subsystem-level baseline coverage and gap classification artifact for ticket `018`.

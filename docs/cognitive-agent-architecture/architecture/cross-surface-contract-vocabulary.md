---
title: Cross-Surface Contract Vocabulary
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-19
updated: 2026-03-19
keywords:
  - commands
  - queries
  - events
  - read models
  - dto
  - transport
  - adapter
index_terms:
  - command query read model
  - transport dto ownership
  - shell embodiment
  - cross-surface vocabulary
related_files:
  - docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md
  - docs/cognitive-agent-architecture/architecture/canonical-schemas-and-contracts.md
  - docs/api/runtime.md
  - clients/apple/README.md
  - crates/vel-core/src/lib.rs
  - crates/vel-api-types/src/lib.rs
summary: Canonical ownership rules for commands, queries, events, read models, and transport DTOs across Vel's Rust core, runtime, and shell adapters.
---

# Purpose

Define the language Vel should use when designing cross-surface product contracts.

This document exists so future Apple, web, CLI, and desktop work uses the same vocabulary for:

- product actions
- product reads
- durable facts about what happened
- shell-facing snapshots
- transport shapes at the boundary

# Problem

Without explicit vocabulary, cross-surface design drifts in predictable ways:

- shells invent screen-shaped APIs
- transport DTOs leak into core semantics
- product logic gets disguised as frontend state assembly
- the same flow is described differently in Apple, web, and runtime code

Vel already has the right direction in several shipped seams. This document turns that direction into a durable rule.

# Canonical Terms

## Commands

Commands are requests that cause product actions or state transitions.

Commands answer:

- what action is being requested
- what inputs are required
- what policy or review gate must be satisfied

Commands belong in:

- domain vocabulary in `vel-core` when the action itself is part of stable product semantics
- application/runtime services in `veld` when orchestration is required
- shell adapters only as invocation wrappers, not as owners of the action semantics

Examples from the current repo:

- start a daily-loop session
- submit a daily-loop turn
- create a capture
- approve or reject an execution handoff

Commands should not be named after one shell’s UI affordance.

Bad pattern:

- `OpenPhoneMorningTab`

Good pattern:

- `StartMorningOverview`
- `SubmitDailyLoopTurn`

## Queries

Queries are requests for product state, inspection, or summary reads.

Queries answer:

- what the operator or shell needs to know now
- what bounded state is being requested
- what auth or capability boundary applies

Queries belong in:

- runtime/application services for composition and policy-aware summarization
- shell adapters as transport invocations only

Examples from the current repo:

- get `Now`
- get current context
- get active daily-loop session
- get agent inspect

## Events

Events are durable records that describe what happened.

Events belong in:

- domain/runtime semantics when they capture meaningful system transitions
- traces, run events, and persisted records where auditability matters

Events should not be invented as frontend convenience labels when they duplicate existing run, capture, review, or persistence semantics.

Examples:

- capture created
- handoff approved
- daily-loop session advanced
- run transitioned to terminal state

## Read Models

Read models are stable shell-facing summaries assembled for operator use.

They are not raw table dumps and they are not screen-specific prop bags.

Read models belong in:

- backend/runtime composition
- typed transport DTOs at the boundary

Examples already present:

- `NowData`
- `DailyLoopSessionData`
- `AgentInspectData`

For the concrete `v0.2` MVP read-model authority, use [mvp-loop-contracts.md](./mvp-loop-contracts.md). That document defines the active loop-specific contracts `OverviewReadModel`, `CommitmentFlow`, `ReflowProposal`, `ThreadEscalation`, and `ReviewSnapshot`.

Read models may be optimized for a shell family or workflow, but they should still be named by product meaning rather than one component tree.

## Transport DTOs

Transport DTOs are the serialized boundary shapes used by HTTP and other adapters.

In the current repo, these belong primarily in `vel-api-types`.

Transport DTOs:

- should be typed and durable
- should stay boring and explicit
- should not become the core domain model

Current examples:

- `ApiResponse<T>`
- `NowData`
- `CommitmentData`
- `AgentInspectData`

# Ownership Rules

## Operator Action Contract Migration Rule

The current migration seam for operator action semantics is:

- domain contract in `vel-core::operator_queue`
- synthesized queue ownership in `veld::services::operator_queue`
- read-model composition in backend services such as `Now`
- DTO mapping in `vel-api-types`

Future action-model work should therefore prefer:

1. extend core action semantics first
2. teach the backend queue how to synthesize or project those semantics
3. map them into transport DTOs
4. let shells consume the result

Avoid skipping directly from a product idea to:

- route-local JSON
- shell-only filter labels
- frontend-owned action categories

## `vel-core`

Owns:

- domain semantics
- stable vocabulary
- invariants
- identifier and value types

Should not own:

- HTTP DTOs
- component-shaped payloads
- shell-only navigation semantics

## `veld` Services And Routes

Own:

- application orchestration
- policy-aware composition
- auth-aware query and command execution
- mapping between domain/state and transport DTOs

Should not own:

- shell-specific interaction design
- Swift or React presentation concerns

## `vel-api-types`

Owns:

- transport DTOs for runtime/API boundaries
- shared serialized shapes used by current HTTP-first shells

Should not own:

- persistence rules
- domain invariants
- storage-only models

## Shell Adapters

Current shells:

- Apple `VelAPI`
- web `types.ts` plus loader layer
- CLI request/formatting layer

These own:

- transport invocation
- local rendering needs
- shell-specific ergonomics

They do not own:

- product truth
- policy
- durable review semantics

# Shell Embodiment Vs Product Contract

Different shells may embody the same contract differently.

That is healthy.

Examples:

- Apple may surface daily loop through voice and glanceable state
- web may surface the same contract through a multi-panel operator view
- CLI may surface the same contract through compact inspection text

Those are embodiment differences.

They do not justify:

- separate product semantics
- separate daily-loop logic
- separate capability rules

# Current Boundary Rules

## Apple

Current Apple mode is HTTP/JSON-first.

That means:

- Swift consumes backend-owned contracts
- `VelAPI` is the transport adapter
- native presentation remains shell-owned

It does **not** mean:

- Apple should reconstruct policy locally
- Swift should become the owner of product workflows

## Web

Current web mode is also HTTP/JSON-first.

That means:

- React consumes typed DTOs and read models
- browser code renders state and interaction flow
- backend remains responsible for policy and summary composition

## CLI

CLI is the fallback and inspection shell.

That means:

- it may format the same data differently
- it should not invent a separate contract vocabulary

# Design Tests

When introducing or reviewing a new cross-surface contract, ask:

1. Is this a command, query, event, read model, or transport DTO?
2. Does it belong in `vel-core`, `veld`, `vel-api-types`, or a shell adapter?
3. Is it named by product meaning rather than one screen?
4. Would Apple, web, and CLI all still understand it even if they render it differently?

If the answer to 4 is no, the shape is probably too shell-specific.

# Anti-Patterns

## Screen-Shaped Contracts

Bad:

- payloads named after one view hierarchy
- contracts that bundle interaction chrome with product meaning

## Shell-Owned Policy

Bad:

- frontend decides whether a mutation is allowed
- Swift decides what a review gate means

## DTO Leakage Into Core

Bad:

- transport-only fields or envelope semantics pushed into `vel-core`

## Generic JSON As The Main Product Contract

Bad:

- raw blobs as the primary surface instead of explicit typed contracts

# Current Proof Examples

The current repo already demonstrates the intended pattern:

- daily loop: backend-owned command/query flow, shared across CLI, web, and Apple
- agent inspect: backend-owned read model, rendered in CLI and web

These should be treated as canonical examples for future work.

# Acceptance Criteria

1. Commands, queries, events, read models, and transport DTOs are defined in one repository-aligned vocabulary.
2. The doc makes clear where each concept belongs today in `vel-core`, `veld`, `vel-api-types`, and shell adapters.
3. Apple and web remain explicitly HTTP/JSON-first consumers of backend-owned contracts unless a later phase changes that architecture intentionally.

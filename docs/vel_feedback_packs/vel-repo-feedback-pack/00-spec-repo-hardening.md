---
title: Vel Repo Hardening Spec
status: proposed
owner: codex
generated_on: 2026-03-16
---

# Summary

Vel already has a credible runtime spine. The next improvement is not "more features first"; it is tighter coherence.

The present repo has four strengths:
- good crate separation
- run-backed work for important flows
- a real schema and migration discipline
- explicit specs for most major ideas

The present repo also has four liabilities:
- doc truth is spread across too many files
- ontology is only partially explicit (`capture`, `signal`, `artifact`, `thread`, `event`)
- `current_context` is effectively the de facto state bus but not yet codified as one
- some APIs and docs still carry historical drift

# Design principles

## 1. One system map, many subordinate docs

Vel needs one canonical map that explains:
- major subsystems
- allowed dependencies
- present-tense evaluation flow
- read-only versus recompute boundaries

## 2. One ontology, enforced in code and docs

Every persisted thing should have one sentence of meaning:
- capture = raw user/system observation
- signal = normalized machine-usable fact
- commitment = intended work / obligation
- artifact = durable generated output
- event = audit record
- context = current derived state
- suggestion = proposed policy/behavior adjustment
- nudge = current actionable prompt

## 3. `current_context` is the runtime boundary

Anything that wants to know "what is happening now" should read `current_context`, not recompute from raw signals ad hoc.

## 4. Read routes must remain read-only

The repo has already moved in this direction for risk and explain. Finish that cleanup consistently.

# Concrete repo-wide code changes

## A. Add a canonical system map doc

Create:
- `docs/specs/vel-canonical-system-map.md`

Contents:
- subsystem list
- crate boundaries
- table of read-only vs recompute routes
- current context ownership
- worker responsibilities
- retry-capable run kinds

## B. Add a terminology doc

Create:
- `docs/specs/vel-terminology.md`

Update:
- `README.md`
- `docs/status.md`
- `AGENTS.md`

## C. Pull route/business-logic rules into a short architecture contract

Create:
- `docs/specs/vel-service-boundary-contract.md`

State explicitly:
- routes validate/map only
- services orchestrate
- storage persists
- explain/read endpoints do not call recompute services

## D. Harden current-context shape

Create a typed `CurrentContext` domain struct in `vel-core` and stop treating the shape as loosely-owned JSON assembled only in service code.

Suggested files:
- `crates/vel-core/src/current_context.rs` (new)
- `crates/vel-core/src/lib.rs`
- `crates/veld/src/services/inference.rs`
- `crates/vel-api-types/src/lib.rs`

## E. Add invariants and smoke tests

Need tests that fail loudly if:
- a GET route causes new persisted risk rows
- current-context JSON omits required keys
- docs claim routes that do not exist
- evaluate order changes

# Acceptance criteria

- docs have a clear canonical hierarchy
- ontology terms are stable across code, routes, and docs
- current context has a typed owner
- read-only endpoints stay read-only
- operator can inspect system state without triggering recomputation

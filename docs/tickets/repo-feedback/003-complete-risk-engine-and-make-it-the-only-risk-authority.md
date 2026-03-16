---
title: "Complete the risk engine and make it the single authority for commitment risk"
status: todo
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on:
  - 001-enforce-evaluate-read-boundary.md
  - 002-refactor-inference-into-deterministic-reducers.md
labels:
  - vel
  - risk
  - backend
  - product
---
The repo has risk scaffolding, risk schema, and risk-aware product language, but risk still feels like a partly-real subsystem rather than the singular place where commitment risk is computed.

That ambiguity is dangerous. If inference, explain routes, nudges, and UI each grow their own little folk theories of urgency, Vel becomes internally inconsistent.

## Goal

Risk must become a first-class, explicit subsystem that owns:

- risk score
- risk level
- risk factors / reasons
- dependency pressure
- consequence / proximity dimensions
- confidence or completeness indicators if data is missing

## Current issue

The codebase still shows traces of fallback logic and ad hoc interpretation around risk. That is survivable during bootstrap, but not once risk begins driving:

- nudge escalation
- current context summaries
- suggested next actions
- UI cards
- weekly synthesis

## Tasks

- Define the canonical risk snapshot shape and keep it stable across storage, API, explainability, and UI.
- Implement missing factors promised by the schema/specs, especially:
  - proximity
  - consequence
  - dependency pressure
  - external anchoring / calendar coupling
  - stale-open age
- Add a concept of partial information so "unknown" is not silently treated as "safe".
- Ensure inference consumes latest persisted risk summaries rather than inventing independent risk heuristics.
- Ensure UI risk cards render fields that actually exist in the canonical risk payload instead of bespoke client-only assumptions.

## Suggested implementation detail

Create a typed internal `RiskSnapshot` or equivalent in `vel-core` so risk is not passed around as half-structured JSON folklore.

## Acceptance Criteria

- There is one canonical risk data shape across compute, storage, API, explain, and UI.
- All risk-producing code lives in the risk layer.
- Inference and nudge logic consume risk; they do not reinvent it.
- Tests cover at least:
  - due-soon commitments
  - blocked-by-dependency commitments
  - externally scheduled commitments
  - long-stale commitments
  - missing-data cases

## Notes for Agent

Vel does not need fake precision. It does need semantic consistency. A rough single authority beats five elegant lies.

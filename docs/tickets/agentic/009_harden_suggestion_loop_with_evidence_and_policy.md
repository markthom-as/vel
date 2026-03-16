# Ticket 009 — Harden the suggestion loop with evidence and policy

## Goal

Take the existing suggestion loop from “good scaffold” to “trustworthy steering primitive”.

## Why now

`crates/veld/src/services/suggestions.rs` already creates suggestions from repeated evidence, but the current implementation is intentionally simple:

- hard-coded 7-day window
- hard-coded threshold
- hard-coded current/suggested minutes
- simple pending-check dedupe

That is enough for scaffolding, not yet enough for a durable steering loop.

## Current starting point

Current file:
- `crates/veld/src/services/suggestions.rs`

Current suggestion types:
- `increase_commute_buffer`
- `increase_prep_window`

Current implementation characteristics:
- repeated resolved nudges trigger suggestions
- evidence threshold is fixed
- payloads use fixed minute values
- no policy-config integration

## Deliverable

Strengthen the loop while keeping it deterministic and inspectable.

## Implementation plan

### 1. Move thresholds into policy/config
Add config-driven values for:
- suggestion evidence window
- minimum count for commute suggestion
- minimum count for prep suggestion
- default increment sizes

### 2. Improve evidence shape
Instead of only “count matching nudges”:
- include last-seen timestamps
- include sample ids or related nudge ids in payload
- optionally include relevant commitment/event context

### 3. Tighten dedupe rules
Prevent suggestion spam by considering:
- pending suggestions
- recently resolved/rejected suggestions of same type
- possibly same context signature

### 4. Improve explainability
Suggestion payload should tell the operator:
- why this suggestion exists
- what evidence was counted
- what config or heuristic was used

## Files likely touched

- `crates/veld/src/services/suggestions.rs`
- `crates/veld/src/policy_config.rs`
- `config/policies.yaml`
- suggestion-related route/DTO docs if payload shape changes
- docs/specs if needed

## Tests

Add tests for:
- threshold behavior
- dedupe behavior
- payload evidence presence
- config overrides

## Acceptance criteria

- suggestion thresholds are not magic constants buried in code
- payloads include evidence strong enough to inspect manually
- repeated suggestion spam is harder
- loop stays deterministic and easy to reason about

## Out of scope

- machine learning
- free-form LLM-generated advice
- giant policy system redesign

## Suggested agent prompt

Implement Ticket 009.

Harden the existing suggestion loop using configuration and better evidence payloads.
Keep the system deterministic and inspectable.
Do not turn suggestions into opaque magic.

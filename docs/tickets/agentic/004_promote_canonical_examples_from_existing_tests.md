# Ticket 004 — Promote canonical examples from existing tests

## Goal

Turn the strongest existing tests and fixtures into **reusable examples** that future agents can compose from.

Vel already has one of the hardest parts: a canonical-day test fixture and end-to-end behavior assertions. Right now that knowledge is trapped inside test code.

## Why now

The repo already contains reusable scenario logic in `crates/veld/src/app.rs`, especially around:

- canonical day setup
- evaluate loop
- context assertions
- risk assertions
- nudge snooze/resolution
- suggestion generation
- synthesis artifact creation

That should become first-class reusable material.

## Current starting point

`crates/veld/src/app.rs` contains:
- `canonical_day_fixture(...)`
- behavioral tests for context, risk, nudges, suggestions, synthesis

This is exactly the kind of “known-good thing” an agent should be able to reuse.

## Deliverable

Extract or mirror the canonical scenario into a more reusable home.

Recommended path:

- keep tests where they are
- add a mirrored example package under `knowledge/examples/`
- optionally extract shared fixture helpers into a test support module if that reduces duplication cleanly

## Implementation plan

### 1. Create a reusable example doc
Document:
- fixture intent
- inserted commitments/signals
- expected system behaviors
- API sequence used

### 2. Consider code extraction
If the current fixture is too embedded inside `app.rs`, move shared setup into:
- `crates/veld/src/test_support.rs`
or
- `crates/veld/tests/support.rs`
only if it improves clarity

### 3. Add a “compose from this example” playbook
Show how an agent can adapt the canonical day fixture for:
- commute policy changes
- suggestion tuning
- new explain assertions
- new synthesis checks

## Files likely touched

- `crates/veld/src/app.rs` or test support extraction
- `knowledge/examples/canonical-day-fixture.md` (new)
- `knowledge/playbooks/adapt-an-existing-fixture.md` (new)

## Tests

- preserve all existing canonical-day coverage
- if extraction occurs, ensure no coverage regression
- avoid giant refactors that change behavior and structure at once

## Acceptance criteria

- the canonical scenario is discoverable outside raw test code
- agents can see how to reuse or adapt it
- fixture logic remains tested
- the example points to concrete code, routes, and assertions

## Out of scope

- replacing all tests
- over-generalizing the fixture
- introducing a heavy fixture framework

## Suggested agent prompt

Implement Ticket 004.

Treat the existing canonical day test scenario as valuable reusable knowledge.
Promote it into docs/examples without weakening the tests.
If you extract code, keep the change small and behavior-preserving.

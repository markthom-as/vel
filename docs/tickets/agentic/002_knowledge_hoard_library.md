# Ticket 002 — Build a Vel knowledge hoard library

## Goal

Create a versioned library of **working examples, recipes, and reference implementations** inside the repo so future agents reuse known-good solutions instead of regenerating them from scratch.

This is the direct Vel implementation of the “hoard things you know how to do” pattern.

## Why now

The repo already contains a lot of reusable knowledge, but it is scattered:

- `docs/specs/`
- `docs/reviews/`
- canonical test fixture logic in `crates/veld/src/app.rs`
- Apple client bootstrap under `clients/apple/`
- route/service patterns repeated across files

Vel is ripe for a proper hoard.

## Current starting point

Existing assets worth harvesting immediately:

- context generation + run-backed artifact pattern
- risk compute + explain pattern
- nudge lifecycle pattern
- canonical day fixture
- Apple client API consumption pattern
- CLI route/command symmetry

## Deliverable

Add a new top-level knowledge area, for example:

- `knowledge/`
or
- `docs/knowledge/`

Recommended structure:

- `recipes/`
- `examples/`
- `playbooks/`
- `prompts/`
- `reference_impls/`

## Implementation plan

### 1. Define the schema
Each knowledge item should include:
- title
- purpose
- when to reuse it
- files involved
- minimal example
- tests or validation steps

### 2. Seed the hoard with real examples from the repo
Minimum initial entries:

1. run-backed endpoint pattern  
2. explain endpoint pattern  
3. route → service → storage vertical slice  
4. canonical day fixture usage  
5. Apple client API consumption pattern  
6. CLI command talking to daemon pattern  

### 3. Add a “reuse before reinvent” section to `AGENTS.md`
Require agents to inspect the hoard before building new code.

### 4. Cross-link from docs
Update `docs/status.md` or the documentation index so the knowledge library becomes part of the official repo map.

## Files likely touched

- `AGENTS.md`
- `docs/vel-documentation-index-and-implementation-status.md`
- `knowledge/README.md` or `docs/knowledge/README.md` (new)
- initial seed files under `recipes/`, `examples/`, `playbooks/`, `prompts/`

## Tests

No code-heavy tests required, but add a basic consistency check if you introduce metadata:
- every item has required headings
- linked files exist

## Acceptance criteria

- the repo contains a dedicated reusable knowledge area
- at least 5 real entries are seeded from the current codebase
- `AGENTS.md` instructs agents to search the hoard first
- examples point to actual files in the repo, not aspirational pseudo-code

## Out of scope

- vector databases
- embeddings
- external knowledge systems
- auto-indexing with a server

## Suggested agent prompt

Implement Ticket 002.

Use the existing Vel repo as source material.
Do not invent hypothetical examples.
Seed the library with working patterns already present in the code.
Prefer concrete, minimal, reusable examples over long essays.

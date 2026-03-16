# Ticket 003 — Linear walkthroughs and architecture snapshot

## Goal

Add a repo-native way for agents and humans to get a **linear walkthrough** of Vel’s real execution flow.

The current repo has rich specs, but it still benefits from a “follow the bytes” map that starts at entrypoints and walks through the runtime in execution order.

## Why now

The codebase is large enough that agents can easily confuse:
- route logic
- service logic
- storage logic
- read-only explain paths
- state mutation paths

A linear walkthrough reduces accidental architectural drift.

## Current starting point

Key execution paths already exist:

- daemon: `crates/veld/src/main.rs`, `crates/veld/src/app.rs`
- routes: `crates/veld/src/routes/*`
- services: `crates/veld/src/services/*`
- storage: `crates/vel-storage/src/db.rs`
- CLI: `crates/vel-cli/src/main.rs`, `crates/vel-cli/src/commands/*`

The docs are broad, but there is no single “execution-order” walkthrough file.

## Deliverable

Create a generated-or-maintained architecture snapshot such as:

- `docs/linear-walkthrough.md`
- `docs/architecture-snapshot.md`

Minimum sections:

1. operator CLI flow  
2. daemon boot flow  
3. evaluate loop flow  
4. context generation flow  
5. explain flow  
6. synthesis flow  

## Implementation plan

### 1. Write the first manual snapshot
Do not over-automate first.
Use the current code to create a trustworthy walkthrough.

### 2. Add a maintenance rule
For every new subsystem or major behavior change:
- update the snapshot if the execution flow changed

### 3. Optional follow-on command
If simple enough, add a documented agent workflow like:
- `vel explain architecture` (later)
For now, a maintained markdown doc is enough.

## Files likely touched

- `docs/linear-walkthrough.md` (new)
- `AGENTS.md`
- `README.md` or `docs/architecture.md`
- maybe `docs/vel-documentation-index-and-implementation-status.md`

## Tests

No runtime tests required.
Validation should be review-driven:
- no factual contradictions with current routes/services
- references point to existing files

## Acceptance criteria

- there is a single document that walks through Vel in execution order
- it clearly separates read-only and mutating flows
- it names the concrete source files for each step
- agents can use it as a first-pass orientation map

## Out of scope

- generating diagrams
- full docs automation
- speculative future architecture

## Suggested agent prompt

Implement Ticket 003.

Write a linear walkthrough of the current Vel repo as it actually exists.
Do not describe planned systems as if they are implemented.
Prefer execution order over conceptual grouping.

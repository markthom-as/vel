# Ticket 001 — Agentic bootstrap: first run the tests

## Goal

Add an explicit, repo-native bootstrap path so any coding agent starts in the same way:

1. read the right docs
2. validate the environment
3. run the test suite
4. only then mutate code

This operationalizes the “first run the tests” pattern against the current Vel repo.

## Why now

The repo already has meaningful tests, but there is no obvious one-command entrypoint or script that standardizes agent startup. Right now that behavior lives mostly in prose in `AGENTS.md`.

## Current starting point

- `AGENTS.md` tells agents what to read
- tests exist in:
  - `crates/veld/src/app.rs`
  - `crates/vel-storage/src/db.rs`
  - `crates/vel-config/src/lib.rs`
  - `crates/vel-cli/src/main.rs`
  - command/service modules
- no `scripts/`, `justfile`, `Makefile`, or CI workflow is visible

## Deliverable

Create a small bootstrap/tooling layer that makes the safe entrypoint obvious and machine-usable.

## Implementation plan

### 1. Add a single bootstrap script
Create one of:

- `scripts/agent-bootstrap.sh`
- `justfile`
- `Makefile`

Preferred shape:

- `fmt-check`
- `lint`
- `test`
- `agent-bootstrap`

`agent-bootstrap` should:
- print required reading order
- run formatting/lint/test commands
- fail fast

### 2. Update `AGENTS.md`
Add a short “mandatory startup sequence” section that points to the new script/command instead of leaving startup behavior informal.

### 3. Add `docs/agentic-dev-process.md`
Document:
- exact bootstrap commands
- what an agent should do on green vs red
- rule: no feature work until baseline is understood

### 4. Add a lightweight status command
Optional but worthwhile:
- `vel doctor` already exists
- add a documented local dev recipe that chains `doctor` + tests

## Files likely touched

- `AGENTS.md`
- `README.md`
- `docs/agentic-dev-process.md` (new)
- `scripts/agent-bootstrap.sh` or `justfile` or `Makefile` (new)

## Tests

Add at least one validation for the bootstrap path:

- if using a script, shellcheck-compatible behavior where possible
- if using `justfile`/`Makefile`, smoke-test documented commands in CI later

## Acceptance criteria

- there is one obvious command for “agent startup”
- `AGENTS.md` references it directly
- the repo documents what to do when tests fail before any code changes
- bootstrap is small, deterministic, and does not hide side effects

## Out of scope

- inventing a giant local toolchain
- adding Docker/Nix unless absolutely required
- changing product behavior

## Suggested agent prompt

Implement Ticket 001 in the Vel repo.

Constraints:
- prefer the smallest possible bootstrap surface
- do not add heavy tooling
- update AGENTS.md and README so the startup path is obvious
- make the command fail fast on test failures

# Ticket 010 — Record-example and skill extraction CLI

## Goal

Add a small repo-native mechanism for turning solved work into reusable examples.

This is the operational bridge between:
- knowledge hoard
- prompts/runbooks
- current repo workflows

## Why now

Once Tickets 002, 004, and 005 exist, Vel should have enough structure to support a lightweight “record this solution” workflow.

Do not overbuild it. The point is to make reuse habitual.

## Current starting point

The repo already has:
- rich docs
- canonical tests
- explainability
- CLI infrastructure in `crates/vel-cli`
- daemon APIs for core runtime behavior

What it does not have is a first-class path for saying:
“this solved pattern should become reusable material.”

## Deliverable

Add a minimal command or script to scaffold a new knowledge/example entry.

Preferred options:

### Option A — CLI subcommand
- `vel knowledge new ...`
- `vel example record ...`

### Option B — local script
- `scripts/record-example.sh`

Given current scope, either is acceptable. A script is cheaper; a CLI command is more native.

## Implementation plan

### 1. Scaffold a new entry
Inputs:
- title
- category (`recipe`, `example`, `playbook`, `prompt`)
- related files
- summary

Output:
- a new markdown file in the knowledge area with standard headings

### 2. Add a small template
Headings:
- purpose
- when to reuse
- files involved
- minimal example
- validation
- caveats

### 3. Document the workflow
Update `AGENTS.md` and the knowledge README to say:
- after a useful solved task, record it

## Files likely touched

- `crates/vel-cli/src/main.rs` and a new command module, or `scripts/record-example.sh`
- knowledge README
- `AGENTS.md`

## Tests

If implemented as CLI:
- add parser tests
- add a filesystem smoke test if practical

If implemented as script:
- keep it very small and document usage

## Acceptance criteria

- there is a low-friction way to scaffold a new knowledge item
- output uses a standard template
- the workflow is documented
- the mechanism is simple enough that people will actually use it

## Out of scope

- databases
- embeddings
- auto-tagging
- autonomous ingestion of arbitrary commits

## Suggested agent prompt

Implement Ticket 010.

Add the smallest useful mechanism for recording solved work as reusable knowledge.
Favor low friction over sophistication.
The output should land directly in the repo’s knowledge library.

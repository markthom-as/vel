# Ticket 005 — Prompt library and agent runbooks

## Goal

Version the prompts and runbooks that should govern agent work in this repo.

Vel already has `AGENTS.md`, but it does not yet have a structured library of:
- code-mod prompts
- review prompts
- walkthrough prompts
- reuse/combine prompts
- TDD prompts

## Why now

The repo is mature enough that agent quality will increasingly depend on **prompt consistency** rather than raw model cleverness.

## Current starting point

- `AGENTS.md` provides general repo instructions
- docs/specs/reviews provide content, but not explicit reusable prompts
- there is no dedicated prompt library

## Deliverable

Add a prompt/runbook area, for example:

- `knowledge/prompts/`
- `knowledge/playbooks/`

Minimum prompt/runbook set:

1. first-run-the-tests  
2. write failing test first  
3. create linear walkthrough  
4. reuse before inventing  
5. small reviewable patch  
6. explain-only read path validation  

## Implementation plan

### 1. Add prompt templates
Each prompt should specify:
- when to use it
- what inputs it expects
- what output shape is desired
- repo-specific constraints

### 2. Add runbooks
Minimum runbooks:
- fixing a failing test
- adding a new endpoint
- extending a service without breaking boundaries
- reviewing a risky subsystem (`inference`, `risk`, `nudge_engine`)

### 3. Wire into `AGENTS.md`
Add a section telling agents which prompt/runbook to choose by task type.

## Files likely touched

- `AGENTS.md`
- `knowledge/prompts/*.md` (new)
- `knowledge/playbooks/*.md` (new)

## Tests

No runtime tests required.
Review for:
- clarity
- specificity
- alignment with actual repo constraints

## Acceptance criteria

- prompt templates are under version control
- prompts reference real repo concepts and file locations
- runbooks make agent behavior more deterministic
- `AGENTS.md` points to them

## Out of scope

- storing chat transcripts
- external prompt registry
- prompt automation infrastructure

## Suggested agent prompt

Implement Ticket 005.

Create a versioned prompt and runbook library for the Vel repo.
Use the current architecture and AGENTS.md as the source of truth.
Prompts should be short, operational, and specific to this codebase.

---
title: "Rationalize docs, status, and roadmap so the repo has one narrative"
status: todo
owner: agent
type: documentation
priority: medium
created: 2026-03-15
depends_on: []
labels:
  - vel
  - docs
  - roadmap
  - product
---
The repo has a lot of useful documentation. It also has enough documentation now to become its own weather system.

That is a compliment with teeth.

## Current issue

There are many spec/review/roadmap files, and they have clearly helped shape the code. But the doc surface is now broad enough that a new agent can easily confuse:

- historical review
- current truth
- planned architecture
- superseded plan
- active implementation sequence

## Goal

Preserve the rich design record while giving the repo a **single operational narrative**.

## Tasks

- Define three explicit doc classes:
  - current truth
  - active plan
  - historical review
- Create a top-level doc index that points agents to the minimum viable reading order.
- Mark superseded or historical specs clearly so they do not masquerade as active requirements.
- Tighten `docs/status.md` so it is the canonical implementation ledger, not an everything-bagel.
- Move stale or archival review material under a clearly named archive/history path if needed.
- Ensure AGENTS.md points to the smallest useful set of authoritative docs.

## Acceptance Criteria

- A new coding agent can answer "what is real, what is next, and what is old?" in under five minutes.
- Historical review docs remain available but are clearly non-authoritative.
- `docs/status.md` and `AGENTS.md` point to the same reality.
- The next implementation sequence is obvious without spelunking half the repo.

## Notes for Agent

Documentation should function like memory, not like dream residue.

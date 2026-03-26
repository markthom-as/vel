# Notes

This directory contains working notes, interview logs, parked design fragments, and exploratory internal writeups.

These files are useful, but they are not implementation-truth authority by themselves.

## What Belongs Here

- interview transcripts and structured Q&A logs
- exploratory implementation notes
- parked design ideas that are not yet accepted into an active milestone packet
- intermediate synthesis documents that may later be promoted into tickets, `.planning`, or canonical docs

## What Does Not Belong Here

- shipped-behavior authority
- the active milestone plan
- the canonical ticket queue
- durable contract docs once the contract boundary is accepted

## How To Read These Notes

When a note conflicts with a higher-authority document, defer in this order:

1. `docs/MASTER_PLAN.md`
2. `.planning/ROADMAP.md` and the active milestone packet
3. `docs/tickets/`
4. canonical docs under `docs/user/`, `docs/api/`, and accepted architecture/contract docs
5. `docs/notes/`

## Promotion Rules

- Promote to `.planning/` when the material becomes active milestone work.
- Promote to `docs/tickets/` when the work becomes a bounded implementation target.
- Promote to `docs/cognitive-agent-architecture/` or `docs/user/` when the contract or user behavior becomes accepted and durable.
- Move explicitly future-only product or architecture concepts to `docs/future/`.

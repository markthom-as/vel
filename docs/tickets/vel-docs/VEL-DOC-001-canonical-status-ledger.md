---
id: VEL-DOC-001
title: Establish canonical status ledger and doc ownership boundaries
status: proposed
priority: P0
owner: docs / platform
---

# Goal

Make `docs/status.md` the single canonical repo-wide implementation ledger and demote the documentation index to navigation/coverage duty.

# Why

Right now multiple docs are auditioning for the role of official truth. The result is not pluralism; it is paperwork entropy.

# Scope

- update `docs/status.md`
- update `docs/vel-documentation-index-and-implementation-status.md`
- add ownership notes to both docs
- add cross-links between canonical status and subsystem docs

# Required changes

## 1. Add a canonical ownership preamble to `docs/status.md`

Add a short top section stating:
- this document is the canonical repo-wide implementation ledger,
- all repo-wide implementation status questions resolve here,
- subsystem docs may contain detail but must not contradict this file.

## 2. Refactor `docs/vel-documentation-index-and-implementation-status.md`

Change the document so it functions as:
- documentation map,
- coverage tracker,
- pointer index.

Remove or rewrite sections that independently claim detailed rollout truth.

## 3. Normalize status labels

Apply a shared taxonomy in `docs/status.md`:
- implemented
- partial
- bootstrap implemented
- deferred
- planned
- experimental

## 4. Add canonical links from subsystem docs

For docs that currently contain status sections, add a note near the top:
- repo-wide implementation status is tracked in `docs/status.md`
- this document provides subsystem-specific detail only.

# Acceptance criteria

- `docs/status.md` explicitly declares itself canonical.
- the documentation index no longer duplicates detailed implementation truth.
- at least chat-related subsystem docs link back to canonical status.
- status labels are consistent.

# Suggested implementation steps

1. edit `docs/status.md` header and section labels.
2. strip detailed status duplication from the doc index.
3. add “scope and authority” note to subsystem docs.
4. do a final grep for stale phrases like “canonical status” to ensure only the right doc makes that claim.


---
title: Vel repo audit hardening spec
status: proposed
owner: platform
created: 2026-03-17
updated: 2026-03-17
---

# Purpose

Turn the current docs/tickets/readme/spec/code audit into one convergence program with three outputs:

1. stricter repo-truth surfaces,
2. hermetic and trustworthy engineering workflows,
3. a deliberate modularization process instead of opportunistic file-splitting.

This spec is about repo quality and execution structure, not new product breadth.

# Audit summary

The repo is usable and materially coherent, but it currently has four friction points:

1. canonical docs still drift from live code in a few high-visibility entrypoints,
2. some tests depend on ambient local files or cwd-sensitive defaults,
3. route and surface modules have grown large enough that boundaries are now uneven,
4. ticket packs vary in maturity, status language, and authority framing.

# Decisions

## 1. Operational docs become strict

The following surfaces should stay terse, current, and non-aspirational:

- `README.md`
- `docs/README.md`
- `docs/status.md`
- `docs/api/README.md`
- `docs/api/runtime.md`
- `docs/api/chat.md`
- active ticket-pack READMEs

These documents describe what is true now, what is actively being worked, and where to go next.

Product ideas, richer future architecture, and exploratory concepts should remain documented in:

- `docs/specs/`
- `docs/product-spec-pack/`
- historical review packs

The rule is not "flatten the ideas". The rule is "do not blur current truth and design intent in the same voice".

## 2. Root README becomes strict status + operator entrypoint

`README.md` should answer:

- what Vel is,
- what is implemented now,
- how to run and verify it,
- where the active work queue lives.

It should link out to the active hardening pack instead of mixing roadmap prose into the summary.

## 3. Missing optional local inputs degrade cleanly

Default local snapshot paths and optional adapter files should not make background loops, bootstrap sync, or tests fail just because the file is absent.

Expected behavior for optional local inputs:

- missing path: zero ingest, no panic
- malformed existing file: explicit failure
- inaccessible existing file: explicit failure

This keeps runtime loops usable while preserving real error reporting for actual bad inputs.

## 4. Modularization must follow a repo map first

Large files should not be split ad hoc.

Before any major decomposition pass, create and maintain:

- a top-level responsibility map,
- route/service/storage ownership boundaries,
- candidate extraction seams,
- acceptance tests that preserve behavior during extraction.

Use [docs/specs/vel-architecture-audit-method.md](vel-architecture-audit-method.md) as the required process for this work.

The goal is architectural compression, not file-count inflation.

## 5. Ticket packs need a shared schema

Every active or near-term ticket pack should declare:

- `class`: convergence, expansion, speculative
- `authority`: execution, design, historical
- `status_model`: allowed local status words
- `source_of_truth`: usually `docs/status.md` for shipped behavior
- `entry_criteria`: when to use the pack
- `exit_criteria`: what completion means

This reduces ambiguity across packs without erasing local detail.

# Workstreams

## A. Repo-truth repair

- fix canonical doc/code mismatches
- align chat ticket accounting
- make README point to active work instead of stale "planned next" text
- add targeted drift checks for high-signal canonical docs

## B. Test hermeticity

- remove dependency on ambient local integration files
- isolate config defaults versus runtime overrides in tests
- keep `cargo test` green from a clean checkout

## C. Architecture map before decomposition

- inventory oversized files and their responsibilities
- identify extraction seams by boundary, not by line count alone
- sequence service/module splits after the map exists

## D. Ticket-pack normalization

- define pack schema
- classify current packs by maturity and authority
- reduce ambiguity between active convergence and speculative architecture
- make pack READMEs converge on a shared metadata and section model

# Big-picture modularization process

Any major decomposition effort should use this sequence:

1. Build a responsibility inventory.
   For each large file, list owned concerns, dependencies, and outward contracts.

2. Mark true boundaries.
   Separate route parsing, application logic, persistence, transport DTO mapping, and UI orchestration.

3. Identify stable seams.
   Extract only when a seam has clear inputs/outputs and test coverage can lock behavior.

4. Extract one axis at a time.
   Examples: chat services, settings/integrations orchestration, storage submodules, typed frontend domains.

5. Re-audit after each slice.
   Confirm the extraction reduced coupling instead of merely moving code around.

# Acceptance criteria

- top-level operational docs no longer contradict live code on high-traffic surfaces,
- `cargo test` passes from a clean repo state without relying on missing local snapshot files,
- the repo has one explicit audit-hardening ticket pack with sequenced ownership,
- the pack includes a dedicated architecture-map process before broad decomposition,
- product/speculative ideas remain documented, but are no longer presented as current runtime truth.

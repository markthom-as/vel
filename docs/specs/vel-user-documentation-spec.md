---
title: Vel Full-Fat User Documentation Spec
status: proposed
owner: docs / product
created: 2026-03-17
updated: 2026-03-17
---

# Purpose

This spec defines the end-user documentation set Vel should ship and maintain as a first-class product surface.

The goal is not a thin developer README. The goal is a full-fat user documentation system that makes Vel usable, explainable, and trustworthy for an actual operator running it day to day.

# Problem

Vel currently has strong internal and implementation documentation, but user-facing documentation is fragmented.

What exists today is enough for contributors and repo operators to infer behavior, but not enough for a normal user to reliably answer:

- what Vel is for,
- how to install and run it,
- how to connect sources,
- what the main surfaces mean,
- what to do each day,
- how to recover from stale or degraded state,
- what data stays local,
- what is implemented now versus planned.

Without a dedicated user-documentation contract, end-user docs risk becoming:

- scattered across README fragments,
- stale relative to shipped behavior,
- too implementation-shaped,
- too thin for onboarding and daily use.

# Principles

## Principle 1: user docs are a product surface

User documentation is part of the shipped product, not cleanup work after implementation.

## Principle 2: current truth first

User docs must describe current behavior first and clearly separate:

- implemented now,
- partial,
- planned,
- unsupported.

`docs/status.md` remains canonical for implementation truth. User docs translate that truth into operator-facing language.

## Principle 3: task-oriented before subsystem-oriented

User docs should primarily answer user tasks such as:

- install Vel,
- start the daemon,
- connect Apple and local sources,
- interpret current context,
- recover from stale integrations,
- use Vel daily.

They should not force users to reconstruct a workflow from architecture docs.

## Principle 4: trust and local-first clarity

Vel is local-first. User docs must make data location, sync behavior, permissions, and trust boundaries explicit.

## Principle 5: one obvious path

There must be one obvious entrypoint for user documentation, with clear next steps from basic setup to daily operation to troubleshooting.

# Required Documentation Set

Vel should maintain the following user-facing documentation set.

## 1. User entrypoint

A canonical entrypoint such as `docs/user/README.md` or equivalent that answers:

- what Vel is,
- who it is for,
- what works today,
- the shortest path to first successful use,
- where to go next.

## 2. Quickstart

A short path from zero to first useful outcome:

- prerequisites,
- install/build,
- start `veld`,
- verify health,
- create first capture or commitment,
- confirm current context is live.

## 3. Setup and configuration guide

A fuller setup guide covering:

- config file and env vars,
- database/artifact locations,
- Apple/macOS local source setup,
- network/base URL choices,
- optional integrations,
- permission requirements.

## 4. Daily use guide

A practical workflow for repeated use:

- morning orientation,
- capture during the day,
- review nudges and commitments,
- evaluate and sync loops,
- end-of-day and weekly review.

## 5. Surface guide

A guide to the main user-visible surfaces and terms:

- current context,
- now,
- nudges,
- commitments,
- captures,
- sync/integration status,
- suggestions,
- explain views.

## 6. Integration guides

Per-source guides for sources that matter to user setup, including at minimum:

- calendar,
- Todoist,
- activity,
- health,
- messaging,
- notes,
- transcripts,
- git.

Each guide should cover:

- what data is used,
- where it comes from,
- how to enable it,
- common failure modes,
- what “working” looks like.

## 7. Troubleshooting and recovery

A user-facing troubleshooting guide for:

- daemon unreachable,
- stale context,
- no sources syncing,
- missing Apple data,
- permission issues,
- configuration mistakes,
- degraded or partial source state.

## 8. Privacy and data ownership guide

A plain-language explanation of:

- local storage,
- exported snapshots,
- secret handling,
- what leaves the machine,
- what does not,
- how to inspect or remove local data.

## 9. Reality and maturity guide

A concise document that explains:

- what Vel is good at today,
- what is still bootstrap or partial,
- what is intentionally deferred.

This should prevent users from mistaking specs for shipped capability.

# Structure

Recommended structure:

- `docs/user/README.md`
- `docs/user/quickstart.md`
- `docs/user/setup.md`
- `docs/user/daily-use.md`
- `docs/user/surfaces.md`
- `docs/user/troubleshooting.md`
- `docs/user/privacy.md`
- `docs/user/integrations/`

This spec does not require all of these files to exist immediately. It defines the target documentation architecture.

# Source of Truth Rules

- `docs/status.md` is canonical for implementation truth.
- user docs must link to status when implementation maturity matters.
- user docs must not present planned behavior as available behavior.
- implementation docs and architecture docs are not substitutes for user docs.

# Implementation Guidance

The first viable slice should be:

1. add a canonical user-docs entrypoint,
2. add quickstart,
3. add setup/configuration,
4. add daily-use guide,
5. add troubleshooting for the most common operator failures,
6. add Apple/macOS local-source guidance because that path now exists and materially affects day-to-day usability.

After that, expand surface guides and per-integration guides.

# Acceptance Criteria

This spec is meaningfully satisfied when:

1. a new user can find one clear docs entrypoint from the repo root,
2. a new user can get Vel running and verify success without reading architecture docs,
3. a user can understand the core surfaces and daily loop from dedicated user docs,
4. a macOS user can set up local Apple-linked sources from explicit instructions,
5. troubleshooting guidance covers the most likely real failures,
6. docs distinguish shipped behavior from planned behavior.

---
title: Ticket 505 - Implement daily review loops on top of project-aware context
status: proposed
owner: codex
priority: high
---

# Goal

Turn Vel's day-centered philosophy into durable loops that generate useful artifacts and interventions.

# Files

## Changed
- `crates/veld/src/services/synthesis.rs`
- `crates/veld/src/services/inference.rs`
- `crates/veld/src/services/loop_registry.rs`

## New
- `crates/veld/src/services/review_loops.rs`

# Loops

## Morning review
Inputs:
- due today tasks
- next event
- overdue commitments
- drifting projects
- unresolved uncertainties

Outputs:
- morning context artifact
- maybe one top suggestion
- maybe one question if blocking ambiguity exists

## Midday review
Inputs:
- current time vs schedule
- completed vs planned state
- interruptions / conflicts
- remaining free windows

Outputs:
- recalibration artifact
- reordering proposal if justified

## Evening review
Inputs:
- completed work
- deferred work
- drifted projects
- suggestion outcomes

Outputs:
- recap artifact
- tomorrow prep suggestion set

# Acceptance criteria

- daily loops use the same project-aware context structure
- each loop produces inspectable artifacts and optional proposals

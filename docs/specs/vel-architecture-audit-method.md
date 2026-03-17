---
title: Vel architecture audit method
status: active
owner: platform
created: 2026-03-17
updated: 2026-03-17
---

# Purpose

Define the required process for auditing Vel's big-picture architecture before decomposing large files, services, or modules.

This is a method spec, not a decomposition ticket.

Use it to decide:

- what a large file currently owns,
- which seams are real,
- which extractions reduce coupling,
- which proposed splits are only line-count theater.

# When to use this method

Use this method before:

- splitting oversized route, service, storage, or client files,
- creating new crates or top-level subsystems,
- moving code across ownership boundaries,
- opening decomposition tickets that affect more than one current module.

Do not skip this process just because a file is large.

# Required inputs

Read these first:

- [docs/status.md](../status.md)
- [docs/architecture-inventory.md](../architecture-inventory.md)
- [docs/future-architecture-map.md](../future-architecture-map.md)
- [docs/specs/vel-repo-audit-hardening-spec.md](vel-repo-audit-hardening-spec.md)

Then inspect the live code for the target area.

# Audit outputs

Every architecture audit should produce four concrete outputs.

## 1. Responsibility inventory

For each target file or module, record:

- owned concerns
- inbound dependencies
- outbound dependencies
- runtime side effects
- persisted state touched
- transport/API contracts touched
- tests covering current behavior

## 2. Boundary classification

Classify code by function:

- route parsing
- application/service logic
- persistence
- DTO mapping
- policy evaluation
- projection/view-model shaping
- background orchestration

If one file owns too many classes, say which ones and why.

## 3. Extraction seams

List candidate seams with:

- seam name
- why it is stable
- inputs
- outputs
- behavioral risks
- tests required before extraction

Do not propose a seam unless its input/output contract can be described clearly.

## 4. Decomposition order

Sequence any follow-on extraction work by risk:

1. lowest-risk internal module extraction
2. medium-risk service boundary cleanup
3. high-risk persistence or transport contract changes

# Audit procedure

## Step 1. Confirm current authority

Before auditing a subsystem, confirm:

- what is shipped now,
- what is in progress,
- which docs are current truth,
- which related packs are only planning or historical input.

## Step 2. Measure the hotspot

Capture:

- file size
- number of direct imports/dependencies when useful
- whether tests exist at route/service/unit levels
- whether the file mixes multiple boundary classes

Large size alone is not enough to justify extraction.

## Step 3. Map responsibilities, not functions

Group code into coherent responsibilities such as:

- chat orchestration
- integration settings persistence
- calendar sync execution
- worker queue routing
- DTO decoder definitions

Avoid trivial groupings like "helper methods at top, handlers at bottom."

## Step 4. Identify authority violations

Look for:

- routes doing application logic
- services doing raw transport shaping
- storage leaking transport DTO assumptions
- clients re-deriving policy instead of projecting server state
- multiple modules acting as truth authorities for the same concept

## Step 5. Propose seams

Good seams usually have:

- one dominant responsibility
- explicit input/output shape
- clear tests
- minimal cross-module mutation

Bad seams usually:

- merely move helpers out of a large file,
- create a new module that still depends on everything,
- duplicate existing domain or transport logic.

## Step 6. Lock behavior first

Before extraction, add or identify:

- unit tests for pure logic
- service tests for orchestration behavior
- route tests for external contract preservation
- fixture notes for persistence-sensitive code

## Step 7. Extract one axis at a time

Allowed first moves:

- split route parsing from orchestration
- split settings persistence from integration execution
- split queue evaluation from sync transport wiring
- split typed frontend domain contracts from page components

Avoid mixing multiple extraction axes in one pass.

## Step 8. Re-audit after extraction

After each extraction, ask:

- did dependency count drop,
- did ownership become clearer,
- did tests become easier to place,
- did any duplicate authority appear,
- did the change reduce real coupling or only redistribute code.

# Default audit targets

Current high-pressure targets include:

- [crates/veld/src/app.rs](/home/jove/code/vel/crates/veld/src/app.rs)
- [crates/veld/src/routes/chat.rs](/home/jove/code/vel/crates/veld/src/routes/chat.rs)
- [crates/vel-storage/src/db.rs](/home/jove/code/vel/crates/vel-storage/src/db.rs)
- [crates/veld/src/services/integrations.rs](/home/jove/code/vel/crates/veld/src/services/integrations.rs)
- [crates/veld/src/services/client_sync.rs](/home/jove/code/vel/crates/veld/src/services/client_sync.rs)
- [clients/web/src/components/SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx)
- [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts)

# Success criteria

The method has been followed when:

- oversized files have explicit responsibility inventories,
- follow-on tickets cite stable seams rather than vague cleanup goals,
- decomposition order is tied to risk and authority boundaries,
- module extraction work can be reviewed against a clear before/after ownership model.

# Roadmap: Vel

## Archived Milestones

- `v0.1` archived phase packet: [v0.1-phases](/home/jove/code/vel/.planning/milestones/v0.1-phases)
- `v0.2` shipped true-MVP archive: [v0.2-ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.2-ROADMAP.md)
- `v0.3` shipped canonical `Now` + client mesh archive: [v0.3-ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.3-ROADMAP.md)

## Latest Closed Milestone

`0.4.x` is now closed. It delivered the `Now/UI` MVP conformance correction line over the shipped `0.3.0` baseline:

- compact `Now` as the daily operating surface
- corrected shell/navigation/documentation access
- restored `Inbox` / `Now` data truth
- cleaned `Threads` and `Settings` MVP information architecture
- closed the line with a strict-clean web build and focused regression evidence

## Active Milestone

The active milestone is now the backend rewrite packet:

- [v0.5-core-rewrite/ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/ROADMAP.md)

Milestone `0.5` turns Vel into a canonical object-centered authority runtime with:

- first-class canonical objects as the system of record
- a typed action membrane for reads and writes
- policy, grants, ownership, and audit as mandatory infrastructure
- module, skill, and workflow primitives in core
- Todoist and Google Calendar as proving adapters
- hard cutover to the new backend authority

## Scope Guardrails

`0.5` is only about the new backend substrate and proving adapters:

- canonical object kernel and relation/storage rewrite
- action membrane, policy, grants, and audit
- module registry/loader and core-module bootstrap
- workflow and skill runtime primitives
- Todoist and Google Calendar adapter proving flows
- hard backend cutover and verification

Do not widen this milestone into:

- web UI or Apple/client embodiment work
- broad connector expansion beyond Todoist and Google Calendar
- speculative workflow-builder or trigger product work
- marketplace/registry ecosystem work

## Versioning Policy

Vel now uses semver language for product lines and shipped releases:

- `0.3.0` is the latest shipped release baseline in the repo
- `0.4.x` is the latest closed release line
- `0.5.0-beta` is the active implementation line toward the first beta backend rewrite

For roadmap execution inside a release line, use a four-part lineage identifier:

- format: `<major>.<minor>.<phase>.<plan>`
- example: `0.5.57.1` means milestone `0.5`, Phase `57`, Plan `01`
- phase numbers remain in place for historical continuity with the existing planning system
- shipped artifacts and tags should continue to use normal semver, not the four-part planning identifier

## Active Phase Packet

Use the `v0.5` packet directly for execution:

- [57-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/57-CONTEXT.md)
- [57-01-PLAN.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/57-01-PLAN.md)
- [ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/ROADMAP.md)

---
*Last updated: 2026-03-22 after closing `0.4.x` and activating the `0.5` backend rewrite packet*

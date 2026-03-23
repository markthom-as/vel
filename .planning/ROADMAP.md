# Roadmap: Vel

## Archived Milestones

- `v0.1` archived phase packet: [v0.1-phases](/home/jove/code/vel/.planning/milestones/v0.1-phases)
- `v0.2` shipped true-MVP archive: [v0.2-ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.2-ROADMAP.md)
- `v0.3` shipped canonical `Now` + client mesh archive: [v0.3-ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.3-ROADMAP.md)
- `v0.5` shipped backend core rewrite archive: [v0.5-ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.5-ROADMAP.md)
- `v0.5.1` shipped canonical client reconnection archive: [v0.5.1-ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.5.1-ROADMAP.md)

## Previously Closed Milestones

`0.5` is now closed. It delivered the backend core rewrite packet:

- canonical object-centered backend authority
- typed action membrane with policy, grants, ownership, and audit
- governed module bootstrap and activation
- manual workflow runtime over canonical objects
- native calendar core plus availability semantics
- Todoist and Google Calendar proving adapters
- hard cutover to canonical write authority with execution-backed milestone proof

`0.4.x` remains the latest closed UI/conformance line over the shipped `0.3.0` baseline:

- compact `Now` as the daily operating surface
- corrected shell/navigation/documentation access
- restored `Inbox` / `Now` data truth
- cleaned `Threads` and `Settings` MVP information architecture
- closed the line with a strict-clean web build and focused regression evidence

## Latest Closed Milestone

`0.5.1` is now complete as the canonical client reconnection line:

- archive: [v0.5.1-ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.5.1-ROADMAP.md)
- requirements: [v0.5.1-REQUIREMENTS.md](/home/jove/code/vel/.planning/milestones/v0.5.1-REQUIREMENTS.md)
- audit: [v0.5.1-MILESTONE-AUDIT.md](/home/jove/code/vel/.planning/v0.5.1-MILESTONE-AUDIT.md)

Milestone `0.5.1` reconnected the web operator surfaces to the frozen `0.5` backend:

- truthful-surface doctrine first
- one canonical transport layer
- three surfaces only: `Now`, `Threads`, `System`
- `WriteIntent`-only mutations
- deletion or explicit quarantine of stale client/backend seams
- Apple handoff/spec only
- explicit accepted debt for live browser workflow dispatch proof, because no shipped canonical invocation route exists in the frozen boundary

## Active Milestone

`0.5.2` is now active as the operator-surface embodiment line:

- packet: [v0.5.2-operator-surface-embodiment/ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.5.2-operator-surface-embodiment/ROADMAP.md)
- doctrine: [0.5.2-operator-surface-doctrine.md](/home/jove/code/vel/docs/cognitive-agent-architecture/architecture/0.5.2-operator-surface-doctrine.md)

Milestone `0.5.2` exists to turn the truthful `0.5.1` line into the intended web operator UI state:

- no backend renegotiation
- no new top-level surfaces beyond `Now`, `Threads`, and `System`
- web-first embodiment
- visible performance repair in the active operator path
- Apple parity/handoff updates only

## Scope Guardrails

The completed `0.5` line was only about the new backend substrate and proving adapters:

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

The completed `0.5.1` line was only about truthful client reconnection:

- no backend schema negotiation
- no framework migration
- no broad UI redesign
- no new providers
- no workflow-builder or trigger product widening
- no Apple implementation work

The active `0.5.2` line is only about operator-surface embodiment:

- no backend schema negotiation beyond provable bugs
- no framework migration
- no new providers
- no workflow-builder or trigger widening
- no Apple implementation

## Versioning Policy

Vel now uses semver language for product lines and shipped releases:

- `0.3.0` is the latest shipped release baseline in the repo
- `0.4.x` is the latest closed release line
- `0.5.0-beta` is the latest completed backend rewrite line
- `0.5.1-beta` is the latest completed client reconnection line
- `0.5.2-beta` is the active operator-surface embodiment line

For roadmap execution inside a release line, use a four-part lineage identifier:

- format: `<major>.<minor>.<phase>.<plan>`
- example: `0.5.57.1` means milestone `0.5`, Phase `57`, Plan `01`
- phase numbers remain in place for historical continuity with the existing planning system
- shipped artifacts and tags should continue to use normal semver, not the four-part planning identifier

## Most Recent Phase Packet

The latest completed execution packet is:

- [v0.5-core-rewrite/ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/ROADMAP.md)
- [65-MILESTONE-EVIDENCE.md](/home/jove/code/vel/.planning/milestones/v0.5-core-rewrite/65-MILESTONE-EVIDENCE.md)

## Active Packet

- [v0.5.2-operator-surface-embodiment/ROADMAP.md](/home/jove/code/vel/.planning/milestones/v0.5.2-operator-surface-embodiment/ROADMAP.md)
- [72-01-PLAN.md](/home/jove/code/vel/.planning/milestones/v0.5.2-operator-surface-embodiment/72-01-PLAN.md)

---
*Last updated: 2026-03-22 after activating milestone `0.5.2`*

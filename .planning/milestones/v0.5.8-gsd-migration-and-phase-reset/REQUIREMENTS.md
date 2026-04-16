# Milestone v0.5.8 Requirements

**Status:** COMPLETE
**Milestone:** v0.5.8
**Theme:** GSD migration and milestone-local phase reset

## Milestone Goal

Make the repo’s GSD workflow trustworthy again before larger feature work resumes.

This milestone is allowed to finish in one of three honest states:

- successful `GSD 2` migration
- compatibility bridge that preserves current workflows while adopting only the safe parts of `GSD 2`
- explicit defer decision backed by a completed readiness audit and documented rationale

## Must-Pass Flows

- [x] the repo’s current dependency on the local `get-shit-done` install is documented concretely
- [x] the chosen `GSD 2` migration or defer path is reflected in repo docs and workflow expectations
- [x] active milestone phase discovery stays limited to the current `01`-based phase packet under `.planning/phases/`
- [x] common workflows such as progress, cleanup, roadmap analysis, and new-milestone handling are checked after the change

## Requirement Buckets

- [x] **AUDIT-58-01**: current `get-shit-done` v1 dependencies, assumptions, and risks are inventoried before cutover
- [x] **MIGRATE-58-01**: the chosen migration, compatibility, or explicit defer path is implemented honestly
- [x] **STATE-58-01**: active planning state remains milestone-local and no archived packet is treated as live work
- [x] **VERIFY-58-01**: direct workflow checks prove the repo is not left in a speculative planning-tool state

## Non-Negotiable Constraints

- [x] do not claim `GSD 2` is adopted if current repo workflows still depend on v1-only behavior or if `gsd-pi` runtime/dependency gaps remain unverified
- [x] do not renumber archived milestone packets just to satisfy tooling heuristics
- [x] do not reactivate deferred duplex voice work inside this milestone
- [x] keep the active phase packet under `.planning/phases/` limited to this milestone’s `01`-based phases

## Completion Rule

This milestone is only ready to close when the repo’s actual GSD workflow state is truthful, reproducible, and directly checked.

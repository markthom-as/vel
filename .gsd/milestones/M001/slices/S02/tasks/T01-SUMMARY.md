---
id: T01
parent: S02
milestone: M001
provides:
  - compatibility bridge rules between legacy .planning workflows and .gsd migration state
requires:
  - M001/S01/T01
affects:
  - M001/S03
key_files:
  - .planning/phases/02-gsd2-migration-cutover-and-codex-integration/02-V1-COMPATIBILITY-BRIDGE.md
key_decisions:
  - compatibility bridge selected over blind GSD 2 migration
patterns_established:
  - preserve verified v1 command execution until GSD 2 equivalents are runtime-wired, dependency-complete, and checked
observability_surfaces: []
drill_down_paths: []
duration: 6min
verification_result: passed
completed_at: 2026-04-15
blocker_discovered: false
---
# T01: Migration Cutover and Compatibility Bridge

Created the compatibility bridge document and kept the migration claim conservative: v1 remains operational authority for Codex commands, while `.gsd` remains migration-state evidence until `gsd-pi` commands are verified under the required Node runtime and proven equivalent for the Codex workflows being replaced.

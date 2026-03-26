# S01: GSD 2 Readiness and Compatibility Audit

**Goal:** prove what the repo currently depends on before any toolchain cutover.
**Demo:** the repo’s real v1 dependency surface and migration blockers are documented concretely enough to drive the next step.

## Must-Haves

- inventory current local install paths, command entrypoints, and repo-specific assumptions
- map those assumptions against the available `GSD 2` surface
- document migration blockers, fallback needs, and rollback shape

## Tasks

- [x] **T01: Readiness audit** `est:small`

## Files Likely Touched

- `.planning/phases/01-gsd2-readiness-and-compatibility-audit/01-GSD-MIGRATION-AUDIT.md`
- `.planning/milestones/v0.5.8-gsd-migration-and-phase-reset/ROADMAP.md`
- `.planning/milestones/v0.5.8-gsd-migration-and-phase-reset/REQUIREMENTS.md`
- `.planning/milestones/v0.5.8-gsd-migration-and-phase-reset/13-NEXT-STEPS.md`

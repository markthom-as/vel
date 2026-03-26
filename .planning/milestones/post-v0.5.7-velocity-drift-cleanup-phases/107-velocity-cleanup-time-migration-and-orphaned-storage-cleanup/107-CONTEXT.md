# Phase 107: Velocity Cleanup - Context

**Gathered:** 2026-03-25
**Status:** Ready for planning
**Source:** `docs/VELOCITY-DRIFT-CLEANUP.md`, backup foundation artifacts, and Master Plan review

<domain>
## Phase Boundary

This phase closes the temporal and schema-cleanup slice from the audit:

- `VD-05` complete the migration from `chrono` / `chrono-tz` to the `time` stack
- `VD-06` re-verify flagged schema objects against the live codebase and Master Plan, then remove only the truly unowned tables

This phase is not a broad storage rewrite. It should preserve schema that still supports backup-first trust or self-awareness directions and only delete what can be shown to have no active or planned owner.

</domain>

<decisions>
## Implementation Decisions

- **D-01:** Cleanup should be filtered through `docs/MASTER_PLAN.md`, not treated as blind deletion.
- **D-02:** `storage_targets` is preserved. It is in active runtime use through `backup_runs_repo` and aligns with backup-first trust direction.
- **D-03:** `verification_records` should be preserved unless this phase can prove it does not support the backup/verification trust line.
- **D-04:** `vel_self_metrics` should be treated as a candidate self-awareness substrate, not as disposable residue, unless this phase can prove it is off-plan.
- **D-05:** Adding a `time`-compatible timezone dependency is acceptable if it is the cleanest path to finishing the migration.

## Agent Discretion

- the exact `time` timezone support crate may vary if the chosen option fits the existing codebase better
- the phase may conclude that no tables should be dropped if the evidence shows they still support planned work
- if one flagged table is preserved, the phase should still document the owner and intended future integration so it does not stay ambiguous

</decisions>

<canonical_refs>
## Canonical References

- `docs/VELOCITY-DRIFT-CLEANUP.md`
- `docs/MASTER_PLAN.md`
- `.planning/PROJECT.md`
- `.planning/ROADMAP.md`
- `docs/future/hosted-v1-signalling-backup-dashboard-spec.md`
- `.planning/milestones/v0.1-phases/09-backup-first-trust-surfaces-and-simple-operator-control/09-02-SUMMARY.md`
- `migrations/0020_vel_self_metrics.sql`
- `migrations/0033_storage_backup_foundation.sql`
- `crates/vel-storage/src/repositories/backup_runs_repo.rs`
- `Cargo.toml`
- `crates/veld/src/services/planning_profile.rs`
- `crates/veld/src/services/timezone.rs`
- `crates/veld/src/services/now.rs`
- `crates/veld/src/services/availability_projection.rs`
- `crates/veld/src/services/recurrence_materialization.rs`

</canonical_refs>

<deferred>
## Deferred Ideas

- `VD-07` through `VD-09` large file splits
- `VD-10` full error-boundary normalization
- `VD-11` CLI integration work for `vel-sim` and `vel-agent-sdk`

</deferred>

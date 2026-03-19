---
phase: 09-backup-first-trust-surfaces-and-simple-operator-control
plan: 03
subsystem: api/cli/settings
tags: [backup, doctor, settings, cli, trust]

# Dependency graph
requires:
  - phase: 09-backup-first-trust-surfaces-and-simple-operator-control
    provides: typed backup manifest/status DTOs from 09-01 and backend backup service/routes from 09-02
provides:
  - backend-owned backup trust classification for doctor and settings surfaces
  - CLI doctor/review rendering over the shared backup trust payload
  - settings payload exposure of backup trust plus the default backup root
affects:
  - Phase 09 plan 04 web trust card and CLI backup workflow
  - operator-facing trust checks across CLI and web surfaces

# Tech tracking
tech-stack:
  added: []
  patterns: [shared backup trust classifier, typed settings payload extension, CLI rendering from backend truth]

key-files:
  modified:
    - crates/vel-api-types/src/lib.rs
    - crates/veld/src/services/doctor.rs
    - crates/veld/src/routes/doctor.rs
    - crates/veld/src/services/chat/settings.rs
    - crates/veld/src/services/backup.rs
    - crates/vel-cli/src/commands/doctor.rs
    - crates/vel-cli/src/commands/review.rs
    - crates/veld/src/app.rs
    - .planning/ROADMAP.md
    - .planning/STATE.md

key-decisions:
  - "Doctor owns backup trust classification (`ok`/`warn`/`fail`) while backup service remains the source of persisted status data."
  - "Settings surfaces expose backup trust as one typed nested object plus the default backup root instead of ad hoc scalar fields."
  - "CLI doctor and review render the backend trust payload directly rather than reimplementing backup freshness heuristics."

patterns-established:
  - "Pattern 1: typed nested operator settings payloads can extend `/api/settings` without inventing a separate transport endpoint."
  - "Pattern 2: stale backup detection is modeled as transport state (`stale`) plus trust level (`warn`) instead of string-only messages."
  - "Pattern 3: CLI trust output can share one formatter across doctor and review surfaces."

requirements-completed: [BACKUP-02, CTRL-01, CTRL-02]

# Metrics
duration: 19m
completed: 2026-03-19
---

# Phase 09: Backup-First Trust Surfaces and Simple Operator Control Summary

Backup trust now flows from one backend-owned classifier into doctor, settings, and CLI review surfaces, so the operator can see freshness, omissions, destination, and degraded-state guidance without learning a second control plane.

## Accomplishments

- Added typed backup trust/freshness/settings DTOs and made `DoctorData` carry the shared backup trust payload.
- Implemented backup trust classification in the doctor service, including `ok`/`warn`/`fail` levels, stale backup detection, and guidance derived from the persisted backup status.
- Exposed the same typed trust object through `/api/settings` and rendered it in `vel doctor` and `vel review`.
- Hardened existing settings tests so they assert the backup payload while no longer depending on environment-specific tailscale discovery values.

## Verification

- `cargo test -p veld chat_settings_get_and_patch -- --nocapture`
- `cargo test -p veld doctor -- --nocapture`
- `cargo test -p vel-cli review -- --nocapture`

All three commands passed.

## Deviations from Plan

- The web runtime projection named in the plan remains for 09-04; this slice intentionally stopped at backend/settings/CLI trust surfaces.
- Existing settings tests were adjusted to assert effective values without hard-coding a specific tailscale discovery result, because the current environment can auto-discover a different valid base URL.

## Next Phase Readiness

Plan 09-04 can now wire the same backend-owned backup trust object into the web settings card and the final CLI backup create/inspect/verify workflow without inventing new status heuristics.

---
phase: 10-daily-loop-morning-overview-and-standup-commitment-engine
plan: 03
subsystem: backend/cli
tags: [daily-loop, standup, cli, commitments]

# Dependency graph
requires:
  - phase: 10-daily-loop-morning-overview-and-standup-commitment-engine
    provides: typed session/storage seam from 10-01 and backend morning routes from 10-02
provides:
  - backend standup start/resume/turn/finalization on the shared session protocol
  - one-to-three commitment cap with typed standup outcome persistence
  - CLI text shell over the daily-loop API for `vel morning`, `vel standup`, and follow-up turn commands
affects:
  - web daily-loop shell in 10-04
  - Apple voice/text integration in 10-05

# Tech tracking
tech-stack:
  added: []
  patterns: [shared-session-protocol, backend-owned-commitment-finalization, cli-thin-shell]

key-files:
  modified:
    - crates/veld/src/services/daily_loop.rs
    - crates/vel-storage/src/repositories/daily_sessions_repo.rs
    - crates/vel-storage/src/db.rs
    - crates/veld/tests/daily_loop_standup.rs
    - crates/vel-cli/src/client.rs
    - crates/vel-cli/src/commands/daily_loop.rs
    - crates/vel-cli/src/commands/morning.rs
    - crates/vel-cli/src/commands/mod.rs
    - crates/vel-cli/src/main.rs

requirements-completed: [STANDUP-01, STANDUP-02, STANDUP-03, SESSION-01]

# Metrics
completed: 2026-03-19
---

# Phase 10 Plan 03 Summary

Phase 10 now includes the bounded standup step and a text-capable CLI shell over the same backend authority. Morning no longer has to be a dead-end passive view; it can hand off into a capped standup that writes up to three durable commitments through the existing commitment store.

## Accomplishments

- Extended the backend daily-loop service to start standup directly or from the latest morning session for the day, carrying forward morning intent signals.
- Added standup turn handling with one reprompt when no commitments are defined, a hard three-commitment cap, typed deferral/calendar/focus outputs, and final commitment writes through the existing storage seam.
- Added a narrow `get_latest_daily_session_for_date` storage helper so standup can reuse completed morning session state without falling back to `current_context`.
- Added CLI daily-loop commands and switched `vel morning` to the shared daily-loop API; added `vel standup` plus turn-oriented follow-up commands for reply/skip.

## Verification

- `cargo test -p veld --test daily_loop_standup -- --nocapture`
- `cargo test -p vel-cli daily_loop -- --nocapture`

Both commands passed. Existing warning noise remains in unrelated areas of `veld` and `vel-cli`.

## Notes

- This slice intentionally keeps the CLI thin: start/resume/skip/reply are transport over the backend session API rather than client-owned loop logic.
- With `10-03` complete, the remaining Phase 10 work can split into the web shell (`10-04`) and Apple/docs closure (`10-05`).

# 17-04 Summary

## Outcome

Completed the Apple and CLI shell-alignment slice so all currently shipped operator shells reflect the same `Now` / `Inbox` / `Threads` / secondary `Projects` / advanced `Settings` hierarchy without widening backend semantics.

## What Changed

- Updated [ContentView.swift](/home/jove/code/vel/clients/apple/Apps/VeliOS/ContentView.swift) so Apple labels and section copy better match the approved shell taxonomy:
  - iPad `Capture` is now taught as `Quick entry`
  - `Projects` reads as `Project context`
  - `Inbox`, `Threads`, and `Settings` copy now explicitly reinforces triage, continuity, and advanced-setup roles
- Updated [ContentView.swift](/home/jove/code/vel/clients/apple/Apps/VelMac/ContentView.swift) so the macOS sidebar reads as summary-first `Now`, explicit `Inbox`, secondary `Project context`, `Quick entry`, and `Settings and docs`.
- Updated [ContentView.swift](/home/jove/code/vel/clients/apple/Apps/VelWatch/ContentView.swift) so watchOS stays summary-first, with `Advisories`, `Inbox actions`, `Quick entry`, and `Settings and docs` framed as supporting lanes rather than raw implementation buckets.
- Refreshed [README.md](/home/jove/code/vel/clients/apple/README.md) with the shared product-surface hierarchy and summary-first Apple posture.
- Reframed CLI shell wording in [main.rs](/home/jove/code/vel/crates/vel-cli/src/main.rs), [today.rs](/home/jove/code/vel/crates/vel-cli/src/commands/today.rs), [doctor.rs](/home/jove/code/vel/crates/vel-cli/src/commands/doctor.rs), [docs.rs](/home/jove/code/vel/crates/vel-cli/src/commands/docs.rs), and [threads.rs](/home/jove/code/vel/crates/vel-cli/src/commands/threads.rs):
  - `vel` help now teaches `Now`, setup/trust, and `Threads`
  - `today` renders as the CLI `Now` lane
  - `doctor` reads as advanced trust/runtime checks
  - `docs` reads as daily-use/setup/deeper-detail guidance
  - `thread` reads as the continuity/archive lane
- Updated CLI tests to lock the new shell framing and repaired stale test fixtures that had drifted behind the current API DTO shape.

## Verification

- `cargo fmt --all`
- `make check-apple-swift`
- `cargo test -p vel-cli`

All passed. `vel-cli` still emits two pre-existing dead-code warnings in `client.rs` during test builds.

## Notes

- This slice intentionally did not add new routes, Apple-local policy, or desktop/Tauri work.
- The Apple verification remains package-level on Linux; full Xcode target builds still require macOS/Xcode.

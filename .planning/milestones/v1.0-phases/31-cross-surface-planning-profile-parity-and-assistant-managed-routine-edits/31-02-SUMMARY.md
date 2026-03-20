# 31-02 Summary

Shipped CLI and Apple planning-profile inspection parity over the canonical backend-owned planning-profile seam.

Main changes:

- added the new CLI inspection lane in [crates/vel-cli/src/main.rs](/home/jove/code/vel/crates/vel-cli/src/main.rs), [crates/vel-cli/src/client.rs](/home/jove/code/vel/crates/vel-cli/src/client.rs), and [crates/vel-cli/src/commands/planning_profile.rs](/home/jove/code/vel/crates/vel-cli/src/commands/planning_profile.rs) so `vel planning-profile` reads the same `/v1/planning-profile` payload the web shell already depends on, with both JSON and summary-first text output
- widened Apple transport parity in [clients/apple/VelAPI/Sources/VelAPI/Models.swift](/home/jove/code/vel/clients/apple/VelAPI/Sources/VelAPI/Models.swift) and [clients/apple/VelAPI/Sources/VelAPI/VelClient.swift](/home/jove/code/vel/clients/apple/VelAPI/Sources/VelAPI/VelClient.swift) by adding typed planning-profile response models and a shared `planningProfile()` client method over the existing backend route
- threaded the same backend-owned profile into Apple stores in [clients/apple/Apps/VeliOS/VelApp.swift](/home/jove/code/vel/clients/apple/Apps/VeliOS/VelApp.swift) and [clients/apple/Apps/VelMac/VelMacApp.swift](/home/jove/code/vel/clients/apple/Apps/VelMac/VelMacApp.swift) so Apple shells do not invent local routine/planning state
- added small summary-first Apple embodiment in [clients/apple/Apps/VeliOS/ContentView.swift](/home/jove/code/vel/clients/apple/Apps/VeliOS/ContentView.swift) and [clients/apple/Apps/VelMac/ContentView.swift](/home/jove/code/vel/clients/apple/Apps/VelMac/ContentView.swift), exposing routine-block and constraint counts plus the next routine anchor without turning Apple into a second planner
- repaired stale CLI fixtures in [crates/vel-cli/src/commands/agent.rs](/home/jove/code/vel/crates/vel-cli/src/commands/agent.rs) and [crates/vel-cli/src/commands/review.rs](/home/jove/code/vel/crates/vel-cli/src/commands/review.rs) so focused `vel-cli` test runs stay real against the current `NowData` and `CommitmentData` contract

Focused verification:

- `cargo fmt --all`
- `cargo test -p veld --test planning_profile_api -- --nocapture`
- `cargo test -p vel-cli planning_profile -- --nocapture`
- `make check-apple-swift`

Notes:

- this slice is read-only parity only; assistant and voice-driven planning-profile edits remain the next slice
- the Apple surface stays summary-first and does not add shell-local planning semantics or mutation logic
- Rust test runs still emit the same pre-existing unused/dead-code warnings in `veld` and `vel-cli`
- no UAT was performed

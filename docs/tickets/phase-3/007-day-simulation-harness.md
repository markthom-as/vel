---
title: Deterministic Day-Simulation Harness
status: planned
owner: staff-eng
type: verification
priority: high
created: 2026-03-17
labels:
  - veld
  - simulation
  - reliability
---

Develop a `vel-sim` library that allows for the deterministic replay of an entire "Day in the Life" of signals and commitments to verify system behavior and prevent regressions in context reduction.

## Technical Details
- **Mock Time**: Replace `std::time` and `tokio::time` usage in the core engine with a controllable `MockClock`.
- **Event Replay**: Implement a harness that can feed 1,000+ synthetic signals and commitment changes into the system sequentially.
- **Snapshot Verification**: Capture the final `CurrentContext` and `Nudge` state at the end of the simulated day and compare it against a known "Golden State."
- **Deterministic Assertions**: Ensure that the simulation produces bit-for-bit identical results across different runs.

## Acceptance Criteria
- A test suite can simulate 24 hours of Vel activity in < 10 seconds.
- Cumulative bugs (e.g., drift in a context field over time) can be reliably reproduced.
- Deterministic behavior is maintained across multiple simulation runs.

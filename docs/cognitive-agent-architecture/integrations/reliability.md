# Reliability + Failure Modes

Vel needs boring reliability. Glamour is not a backup strategy.

## Required Mechanisms

- append-only event log
- periodic snapshots
- retry queues for transient integration failures
- idempotent task execution where possible
- explicit degraded mode

## Failure Classes

- stale context
- duplicate notification
- missed reminder
- integration timeout
- policy regression
- reflection overfit

## Operational Requirement

Every user-visible failure should leave a trace
that can be inspected later.

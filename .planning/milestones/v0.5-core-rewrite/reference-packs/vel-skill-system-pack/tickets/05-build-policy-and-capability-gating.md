# Ticket: Build policy and capability gating

## Objective

Implement requested-vs-granted capability mediation for skill runs.

## Deliverables

- capability namespace definitions
- per-run grant resolution
- deny/ask/auto modes
- execution metadata for requested/granted/denied capabilities

## Acceptance criteria

- read-only permissions can be granted automatically where policy allows
- disallowed capabilities block execution or trigger degraded mode
- logs clearly show requested vs granted capabilities

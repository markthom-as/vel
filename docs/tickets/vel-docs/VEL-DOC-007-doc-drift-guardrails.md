---
id: VEL-DOC-007
title: Add doc drift guardrails and checks
status: proposed
priority: P1
owner: docs / nav-core
---

# Goal

Reduce the probability that documentation silently drifts away from implementation over time.

# Scope

- integration with the self-knowledge / drift engine
- lightweight checks in CI or local tooling

# Required changes

1. Identify a small set of high-signal drift checks (e.g., missing routes, stale Apple URLs, outdated risk/nudge descriptions).
2. Wire these into a CLI command (or reuse `vel system drift`).
3. Document a recommended cadence for running drift checks (locally or in CI).

# Acceptance criteria

- At least one high-value drift check is runnable as a command.
- The docs for status and API routes mention the existence of drift checks.


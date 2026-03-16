---
id: VEL-META-012
title: End-to-end tests fixtures and staged rollout for metadata enrichment
status: proposed
priority: P1
estimate: 3-4 days
dependencies: [VEL-META-001, VEL-META-011]
---

# Goal

Ship this without turning user systems into an avant-garde database performance piece.

# Scope

- Build fixtures for Todoist tasks, calendar events, emails, and cross-linked examples.
- Add end-to-end tests covering:
  - scan -> detect -> propose -> review -> apply
  - reject/edit/apply flows
  - auto-apply policies
  - rollback and conflict cases
- Add feature flags and staged rollout config.

# Deliverables

- E2E test suite
- fixture corpus
- feature flags
- rollout checklist and migration notes

# Acceptance criteria

- Happy path and failure modes are covered.
- Feature can be enabled per source or environment.
- Rollout docs define safe initial policy thresholds.

# Notes

The whole point is reducing metadata entropy, not becoming it.

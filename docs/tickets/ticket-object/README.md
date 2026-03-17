# Vel Ticket Object Ticket Pack

This pack introduces `ticket` as a first-class Vel object with:

- a native Vel backend
- provider-backed backends for GitHub Issues, Linear, Jira, and Todoist
- explicit linkage to commitments, projects, people, sessions, and runs

Primary spec:

- [docs/specs/vel-ticket-object-spec.md](../../specs/vel-ticket-object-spec.md)

## Why this pack exists

Vel currently has commitment-backed action tracking and Todoist mirroring, but it does not yet have a canonical backlog/work object that can span native and external systems.

This pack fixes that by making `ticket` first-class.

## Execution order

1. core schema and native backend
2. commitment/project linkage
3. provider backends
4. operator surfaces
5. migration, tests, and rollout

## Ticket list

- `VTKT-001-ticket-domain-schema-and-storage.md`
- `VTKT-002-native-vel-ticket-backend.md`
- `VTKT-003-ticket-commitment-project-link-model.md`
- `VTKT-004-github-issues-ticket-backend.md`
- `VTKT-005-todoist-ticket-backend-and-migration.md`
- `VTKT-006-linear-ticket-backend.md`
- `VTKT-007-jira-ticket-backend.md`
- `VTKT-008-ticket-api-and-cli.md`
- `VTKT-009-project-surface-ticket-first-migration.md`
- `VTKT-010-tests-fixtures-and-rollout.md`

---
id: INTG-002
title: People and external identity graph
status: proposed
priority: P0
estimate: 4-6 days
dependencies:
  - INTG-001
---

# Goal

Create a first-class Vel-native person model that can unify human identity across calendars, messages, transcripts, tasks, and documents.

# Scope

- Add canonical person tables and core types.
- Add person aliases, contact methods, and external identities.
- Add merge and resolution records with explainable confidence metadata.
- Define unresolved identity handling for low-confidence matches.
- Add fixture cases where one real person appears across multiple providers.

# Deliverables

- `people` domain model in `vel-core`
- storage schema and repositories
- person lookup helpers
- person external identity model
- merge or resolution audit records

# Acceptance criteria

- One person can hold many provider identities.
- Provider identity records can remain unresolved without data loss.
- Message, transcript, calendar, and workspace adapters all have a shared target for human identity.
- Explain surfaces can later answer why a person match exists.

# Notes

Do not reduce people to display strings. That path leads directly to provenance rot and duplicate-human soup.

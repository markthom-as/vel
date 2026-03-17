---
id: VTKT-004
title: GitHub issues ticket backend
status: proposed
priority: P1
estimate: 3-5 days
dependencies:
  - VTKT-001
  - VTKT-003
---

# Goal

Map GitHub issues into the canonical ticket model so they are both self-knowledge evidence and project/work tickets.

# Scope

- ingest GitHub issue metadata into tickets
- link repository, issue number, labels, assignees, and milestones
- preserve alignment with the GitHub issue awareness spec

# Deliverables

- GitHub issue ticket adapter
- provider mapping layer
- provider-specific metadata contract

# Acceptance criteria

- GitHub issues appear as canonical tickets
- ticket identity and GitHub issue identity are not duplicated into competing models
- project surfaces can show GitHub-backed tickets alongside native ones

# Notes

GitHub issues should not live in a strange half-state between self-knowledge and project work.

---
id: SK-011
title: GitHub issue awareness for self-knowledge and maintenance loops
status: proposed
owner: nav-core
priority: p1
area: self-knowledge/github
depends_on: [SK-001, SK-002, SK-003]
---

# Goal

Make GitHub issues first-class self-knowledge inputs so Vel can reason about planned work, known failures, and issue-driven maintenance without confusing backlog intent with shipped behavior.

# Tasks

1. Add a normalized GitHub issue representation and provenance contract.
2. Ingest issue metadata, body text, labels, links, and timestamps.
3. Link issues to files, docs, tickets, PRs, and validation profiles when referenced.
4. Surface issue-aware capability metadata for branch sync and validation work across clients.
5. Ensure issue-driven requests are queueable through structured sync actions.

# Acceptance Criteria

- Vel can store and inspect GitHub issues as self-knowledge evidence.
- Issue references to docs/files/tickets create durable links.
- Open issues remain clearly distinct from shipped implementation truth.
- Cluster/bootstrap metadata exposes branch sync and validation capabilities for clients that can act on issue-driven work.
- Clients can queue branch sync or validation requests without inventing ad hoc payloads.

# Spec Reference

- [docs/specs/vel-github-issues-spec.md](../../specs/vel-github-issues-spec.md)
- [docs/specs/vel-self-knowledge-system-spec.md](../../specs/vel-self-knowledge-system-spec.md)
- [docs/specs/vel-cluster-sync-spec.md](../../specs/vel-cluster-sync-spec.md)

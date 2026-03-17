---
title: Vel Projects page ticket pack
status: ready
owner: agent
created: 2026-03-16
---

# Vel Projects page ticket pack

This pack contains codex-ready markdown tickets for implementing a **Projects** surface in Vel.

## Scope

This pack covers:

- project registry and canonical view-models,
- Todoist-backed project work view and task mutation,
- active chat/session registry across Vel and external assistants,
- queued messages, steering, feedback, and settings controls,
- web Projects page,
- CLI command parity.

## Grounding

This pack assumes the attached repo state where:

- Todoist already syncs into commitments and signals,
- web already has `Now`, `Inbox`, `Threads`, `Settings`,
- transcript ingestion already lands in `assistant_transcripts`,
- there is no first-class Projects page yet.

## Boundary rules

- Do not introduce a duplicate durable task authority unless commitments prove insufficient.
- Todoist remains the system of record for user tasks whenever connected.
- External chat systems may be read-only or queue-only depending on adapter capability.
- Shared backend contracts must power both web and CLI flows.

## Recommended execution order

1. project registry and view-model contracts
2. migrations and storage methods
3. Todoist write-through task mutation support
4. project projection/workspace APIs
5. agent sessions + outbox + steering/feedback APIs
6. web Projects UI
7. CLI command family
8. tests, docs, rollout hardening

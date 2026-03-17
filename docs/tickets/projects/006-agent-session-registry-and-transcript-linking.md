---
id: VEL-PROJ-006
title: Add agent session registry above assistant transcripts
status: proposed
priority: P0
estimate: 3-4 days
dependencies:
  - VEL-PROJ-002
  - VEL-PROJ-004
labels:
  - transcripts
  - chats
  - projects
---

# Goal

Turn passive transcript ingestion into a first-class session model that the Projects page can actually use.

# Scope

- Build a registry layer that maps transcript streams and Vel-native conversations into `agent_sessions`.
- Add source enums for:
  - vel
  - codex
  - claude
  - opencode
  - chatgpt
  - other
- Map transcript metadata to session title, project hint, and last activity.
- Establish linking rules between `assistant_transcripts` and `agent_sessions`.
- Create session summaries for project workspaces.

# Deliverables

- service for upserting/linking agent sessions from transcript ingestion
- optional backfill job to derive sessions from existing `assistant_transcripts`
- session list/query storage methods
- session summary DTOs
- tests for transcript-source mapping and project linkage

# Acceptance criteria

- External assistant transcripts can be surfaced as project-linked active sessions.
- Vel-native conversations can appear in the same project session list.
- Session summaries expose source, title, last activity, and project confidence.
- Backfill or lazy-link logic does not duplicate sessions on repeated sync.

# Notes for agent

`assistant_transcripts` are raw material, not the UI model. This ticket gives them a spine.

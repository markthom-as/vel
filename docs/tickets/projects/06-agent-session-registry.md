---
title: Add agent session registry for project-linked chats
status: ready
owner: agent
priority: P0
area: projects
---

# Goal
Introduce a first-class project-linked session model for active work across Vel, Codex, Claude, OpenCode, and future sources.

## Scope
- add `agent_sessions` table and storage queries
- define DTOs and states
- seed/read sessions independently of transcript evidence
- optionally infer/update session recency from transcript sync or project-linked conversations

## Requirements
- a session is not just a transcript row
- support at least sources: `vel`, `codex`, `claude`, `opencode`, `chatgpt`, `other`
- support statuses: `active`, `idle`, `blocked`, `done`, `archived`
- support `mode` hints: `chat`, `code`, `research`, `review`
- store latest summary and queue depth
- link each session to a project slug

## Suggested storage fields
- id
- project_slug
- source
- source_ref
- title
- status
- mode
- queue_depth
- last_message_at
- last_operator_action_at
- latest_summary
- settings_json
- metadata_json
- timestamps

## Projection behavior
Project workspace should surface sessions ordered by:
1. active status
2. latest activity
3. explicit pinned/priority metadata if added later

## Tests
- create/list/update session
- project workspace includes sessions
- transcript import can optionally refresh last activity for linked sessions without replacing the session model

## Done when
- session registry exists
- sessions are part of workspace payload
- no code path treats raw transcripts as the only active-session abstraction

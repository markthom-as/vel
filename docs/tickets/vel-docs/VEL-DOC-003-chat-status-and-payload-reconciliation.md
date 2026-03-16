---
id: VEL-DOC-003
title: Reconcile chat status docs and fix payload shape mismatches
status: proposed
priority: P0
owner: chat / docs / web
---

# Goal

Make chat documentation internally consistent and exactly aligned with current backend/frontend behavior.

# Scope

- `docs/status.md`
- `docs/vel-documentation-index-and-implementation-status.md`
- `docs/chat-interface-status-and-outstanding.md`
- canonical API docs for `/api`

# Required changes

## 1. Set repo-wide chat status in `docs/status.md`

Update the chat section to reflect implemented work backed by the codebase and tests, including the completion state for tickets 034 and 035 if that is indeed present.

## 2. Strip conflicting ledger behavior from the doc index

The documentation index should not separately claim chat is only implemented through ticket 033 if that is no longer true.

## 3. Correct message creation response documentation

Update the chat docs to reflect that:
- `POST /api/conversations/:id/messages` returns `CreateMessageResponse`
- response contains `user_message`
- may contain `assistant_message`
- may contain `assistant_error`

Do not describe this response as `MessageData` unless the implementation is changed to match.

## 4. Separate “implemented” from “outstanding” in chat docs

The chat status doc should have clean sections like:
- Implemented
- Known limitations
- Outstanding work
- Planned improvements

# Acceptance criteria

- all repo docs agree on current chat milestone status.
- chat route docs match actual response shape.
- no doc claims message-create returns `MessageData` unless code changes.
- subsystem doc contains detail without contradicting canonical status.

# Suggested implementation steps

1. verify current response struct name and serialized fields in `crates/veld/src/routes/chat.rs`.
2. update canonical `/api` docs.
3. update chat subsystem doc.
4. update repo-wide status and doc index pointers.


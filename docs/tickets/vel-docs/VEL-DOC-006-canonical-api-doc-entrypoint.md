---
id: VEL-DOC-006
title: Create canonical API documentation entrypoint for /v1 and /api
status: proposed
priority: P1
owner: docs / backend / chat
---

# Goal

Provide a single discoverable API documentation entrypoint that covers both `/v1` runtime APIs and `/api` chat APIs.

# Scope

- `docs/api.md`
- new `docs/api/` directory (if not present)

# Required changes

1. Create `docs/api/README.md` as the canonical API entrypoint.
2. Add `docs/api/runtime.md` for `/v1` routes.
3. Add `docs/api/chat.md` for `/api` chat routes.
4. Update or replace `docs/api.md` to point to this structure.

# Acceptance criteria

- A new contributor can find both runtime and chat API docs starting from a single README.
- No second, hidden API universe exists outside this entrypoint.


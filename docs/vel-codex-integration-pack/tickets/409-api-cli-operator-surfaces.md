---
title: Ticket 409 - Add project and sync inspection surfaces
status: proposed
owner: codex
priority: medium
---

# Goal

Give the operator enough visibility to understand external awareness and fix drift.

# Files

## Changed
- `crates/veld/src/routes/integrations.rs`
- `crates/veld/src/routes/sync.rs`
- `crates/veld/src/routes/context.rs`
- `crates/vel-cli/src/main.rs` or relevant command modules
- `clients/web/src/components/SettingsPage.tsx`
- `clients/web/src/components/ContextPanel.tsx`

# Add API endpoints
- `GET /v1/projects`
- `GET /v1/projects/:slug`
- `GET /v1/external-items?source_kind=todoist`
- `GET /v1/sync/operations`
- `POST /v1/sync/operations/:id/approve`
- `POST /v1/sync/operations/:id/reject`
- `POST /v1/integrations/projects/reload`

# UI additions
Settings page:
- show last successful project registry import
- show unresolved project mappings
- show pending sync proposals

Context panel:
- active project cluster
- next event
- due/overdue task counts
- sync degradation warning

# Acceptance criteria

- operator can inspect project mappings and pending external writebacks
- context UI exposes external awareness, not just raw text summaries

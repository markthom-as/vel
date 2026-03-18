# Connect API

## Status: Reserved (Phase 2 SP2)

The `/v1/connect` endpoint family is reserved for the agent connect protocol
(ticket 006 — Connect: Agent Launch Protocol & Supervision).

Current behavior: all `/v1/connect/*` routes return 403 Forbidden until
ticket 006 SP2 implementation is complete.

## Planned Endpoints (SP2)

- `POST /v1/connect/launch` — Launch a supervised agent runtime
- `POST /v1/connect/heartbeat` — Renew execution lease
- `POST /v1/connect/terminate` — Terminate a running agent
- `GET /v1/connect/status` — List active connect instances with lifecycle state

## Current Alternative

Use `GET /v1/sync/cluster` to list registered worker nodes and their capabilities.

The CLI equivalent is `vel sync status`.

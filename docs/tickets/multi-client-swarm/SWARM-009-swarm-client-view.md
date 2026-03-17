---
id: SWARM-009
title: Swarm Client Operator View
status: todo
priority: P0
estimate: 3-5 days
dependencies:
  - SWARM-004
---

# Swarm Client Operator View

Provide a single operator surface that enumerates every client/worker in the swarm, summarizes its identity + capabilities, and reports live sync/heartbeat/active-work health.

## Scope

- Expose client metadata (`client_id`, `node_id`, `display_name`, `platform`, `client_version`, `protocol_version`, `build_id`).
- Surface functionality/capabilities: `worker_classes`, execution capabilities, sync transports, and active services (branch sync, validation, metadata refresh). Include which loops/actions the client currently runs.
- Report sync state: last upstream/downstream sync timestamps, sync cursors, authority epoch, pending actions, last sync error, and ping/heartbeat age.
- Show live queue/work status: claimed `work_request_id` values, receipt status, estimated completion time, and linked run/task ids.
- Include reachability/transport (tailscale/lan/local) and measured latency/ping when available.
- Provide operator guidance when a client is stale, version-mismatched, or missing required capabilities.

## Deliverables

- `/v1/swarm/clients` read model (or an extended `GET /v1/sync/cluster`) implementing the contract above.
- Operator documentation describing how to interpret version/capability/sync columns and debug flags.
- Tests that keep the view in sync with heartbeat and receipt updates.

## Acceptance criteria

- Operators can list clients and sort/filter by capability, version, or sync health.
- Each client record includes build/protocol version plus heartbeat/ping data for mismatch detection.
- Active task/work info links to receipts so operators know what every client is executing.
- The view explicitly notes which fields are currently shipped versus planned, preventing speculative interpretation.

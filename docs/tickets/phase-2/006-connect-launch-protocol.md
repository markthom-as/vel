---
title: Connect: Agent Launch Protocol & Supervision
status: in-progress
owner: staff-eng
type: architecture
priority: medium
created: 2026-03-17
labels:
  - veld
  - agentic
  - connect
---

Formalize the `Connect` RPC protocol to allow the Authority Node to launch, monitor, and supervise external agent runtimes across the cluster.

## Technical Details
- **RPC Protocol**: Define a `ConnectMessage` enum for `LaunchRequest`, `Heartbeat`, `StatusUpdate`, and `KillRequest`.
- **Lease System**: Implement a lease-based token system where launched agents must check-in via heartbeat to maintain their execution lease.
- **Worker Discovery**: Update `client_sync` to match `LaunchRequests` with nodes advertising the relevant `agent_runtime:*` capability.
- **Capability Scopes**: Launch requests must include explicit tool or capability allowlists, not ambient authority.
- **No Self-Escalation**: Launched agents must not be able to widen their own permissions through the protocol.
- **Trace Linkage**: Connect workflows must emit stable run or trace identifiers for launch, handoff, heartbeat, denial, and termination events.
- **Execution Isolation**: External runtimes should execute in dedicated sandboxes or isolated runtime envelopes where possible.
- **Termination**: Implement graceful and forced termination logic for runaway processes.

## Acceptance Criteria
- Authority node can launch an external process (e.g., a local LLM or coding agent) on a worker node.
- Agents are terminated if they fail to provide heartbeats.
- Every launched process is tracked with a `RunRecord` in the database.
- Agents launch with explicit scoped capabilities rather than implicit full access.
- Basic launch-and-kill integration tests pass.

---
title: Swarm Execution SDK & Contract
status: planned
owner: staff-eng
type: architecture
priority: medium
created: 2026-03-17
labels:
  - vel-core
  - distributed
  - agent-sdk
---

Formalize the communication contract and provide a unified SDK (`vel-agent-sdk`) for external agents to securely interact with the Vel "Brain."

## Context & Objectives
As Vel transitions to a multi-agent swarm, we need a standardized way for external "Limbs" to:
1.  Discover the Authority Node.
2.  Perform the `Connect` handshake.
3.  Request `CurrentContext` snapshots.
4.  Submit `ClientAction` batches (idempotently).
5.  Maintain a heartbeat to keep their execution lease.

## Impacted Files & Symbols
- **Crate**: `vel-protocol` (New)
  - **Symbols**: `trait VelLimb`, `enum BrainMessage`, `struct LimbManifest`
- **Crate**: `vel-core`
  - **Symbols**: `ConnectInstance`, `WorkerCapability`

## Technical Requirements
- **Protocol**: JSON-RPC over WebSockets or Local Unix Sockets.
- **Security**: Lease-based tokens for all agent actions.
- **Capability Negotiation**: Agents must declare requested capabilities up front and receive explicit scoped grants.
- **Secret Boundary**: The SDK should prefer brokered capabilities or mediated actions over exposing raw provider credentials to agents.
- **Traceability**: Requests, handoffs, denials, and execution results should carry stable run or trace identifiers.
- **Portability**: Provide bindings for Rust and TypeScript (for web-based agents).

## Implementation Steps
1. **Design**: Define the `LimbManifest` and `BrainMessage` enums.
2. **Infrastructure**: Implement the `vel-protocol` crate as a shared dependency.
3. **SDK**: Build the `Limb` reference implementation that handles the heartbeat loop and capability negotiation.
4. **Integration**: Update `veld::services::connect` to use the new protocol.

## Acceptance Criteria
- [ ] An external process can connect to Vel and retrieve the `CurrentContext`.
- [ ] The agent can successfully submit a `capture_create` action using the SDK.
- [ ] The Authority Node correctly tracks the agent's lease via heartbeats.
- [ ] `vel-protocol` is fully unit-tested for serialization/deserialization.
- [ ] The SDK never requires an agent to hold broad provider credentials for ordinary mediated actions.

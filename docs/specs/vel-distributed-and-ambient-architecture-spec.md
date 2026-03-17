# vel_distributed_and_ambient_architecture_spec.md

Status: Canonical distributed / ambient architecture specification  
Audience: coding agent implementing Vel  
Purpose: define how VELD, CLI, mobile/watch clients, desktop voice, and future edge/browser clients relate to one another across online, offline, and handoff scenarios

Related detailed sync/runtime specs:

- `docs/specs/vel-cluster-sync-spec.md` — concrete cluster-aware sync contract, authority epochs, Tailscale-first routing, swarm worker sync
- `docs/specs/vel-multi-client-swarm-spec.md` — supervised parallel work across clients/workers, load balancing, integration rules

---

# 1. Purpose

Vel is not a single-process toy. It is intended to operate across:

- desktop
- NAS
- VPS
- iPhone
- Apple Watch companion
- future browser / worker surfaces
- future ambient/voice surfaces

This spec defines:

- which node is authoritative
- what VELD owns vs what clients own
- how state is replicated
- what works offline
- how handoff between devices should behave
- how synthesis is distributed
- how Tailscale is used without becoming a hard dependency for local-only operation

This is **not** a generic distributed systems spec.  
It is a personal-assistant deployment model.

---

# 2. Core Principle

Vel should be designed as:

> **one preferred brain, many temporary limbs**

Meaning:

- one preferred canonical node at a time
- multiple edge clients/nodes that can act locally
- event/action replication rather than fragile state cloning
- local degraded operation when disconnected
- later reconciliation with canonical state

Do not design Vel as a multi-master cluster.  
This is not a Raft dissertation in disguise.

---

# 3. Node Types

## 3.1 Canonical Core Node

A preferred always-on VELD node.

Examples:
- NAS
- desktop
- VPS

Responsibilities:
- canonical event/action log
- canonical signal store
- canonical commitments/nudges/threads/artifacts metadata
- current-context computation
- risk computation
- policy engine
- canonical synthesis when appropriate
- replication coordination

## 3.2 Compute-Preferred Node

A node with better compute resources, especially for LLM-backed synthesis.

Examples:
- desktop with strong GPU
- VPS with sufficient inference capability

Responsibilities:
- canonical or preferred synthesis execution
- heavier reflective analysis
- optional local assistant/voice reasoning
- future model-serving role

This node may or may not be the same as the canonical core node.

## 3.3 Edge Node

A client capable of local operation and later sync.

Examples:
- laptop
- iPhone app
- desktop CLI when detached from canonical node
- watch companion via phone
- future browser worker

Responsibilities:
- local cache
- local action queue
- local signal capture
- limited local context readout
- local acknowledgements (done/snooze)
- sync when canonical node becomes reachable

## 3.4 Ephemeral Client

A mostly stateless or lightly cached interface.

Examples:
- browser tab
- smart speaker frontend
- future voice shell that delegates everything

Responsibilities:
- capture input/output
- delegate to nearest edge/core
- minimal local persistence

---

# 4. Canonical Node Ordering

Default preference order should be:

1. **NAS**
2. **Desktop**
3. **VPS**

Reasoning:
- NAS is the preferred central always-on state anchor
- desktop has stronger local compute and can become temporary authority if needed
- VPS is an optional remote fallback / compute node

This ordering should be configurable later, but this is the canonical starting policy.

---

# 5. Temporary Authority

If the preferred canonical node is unavailable, the next available node in the ordering may become **temporary canonical authority**.

## 5.1 Allowed temporary authority
Yes:
- desktop may become temporary canonical authority when NAS is offline
- VPS may become temporary canonical authority when both NAS and desktop are unavailable

## 5.2 Requirements
When a temporary authority is active:
- it must record actions/signals in canonical form
- it must preserve append-only logs
- it must sync back to the preferred canonical node later
- it must not silently fork incompatible state

## 5.3 Important rule
Temporary authority is not “new truth forever.”  
It is a continuity mechanism until preferred authority returns.

---

# 6. VELD vs VEL CLI Responsibilities

This distinction is mandatory.

## 6.1 VELD (daemon/runtime node) owns:
- authoritative stores
- signal ingestion coordination
- current-context computation
- risk engine
- policy engine
- nudge state
- synthesis scheduling
- replication endpoints
- background jobs

## 6.2 VEL CLI owns:
- user/operator interface
- command parsing
- presentation
- direct capture / action creation
- local fallback behavior when VELD unavailable
- submitting actions to local or remote VELD

## 6.3 Important rule
CLI must not slowly grow into a shadow daemon.  
If local fallback execution is needed, it should still use shared core modules and explicit local-node mode.

---

# 7. Replication Model

Vel should replicate **events/actions first, state second**.

This is a critical architectural rule.

## 7.1 Replicate:
- signals/events
- user actions
- acknowledgements
- nudges state transitions
- suggestions state transitions
- captures
- commitments changes
- thread changes
- artifact metadata
- current-context snapshots optionally as cache

## 7.2 Do not treat as primary replication primitive:
- raw SQLite file sync
- opaque state dumps without event provenance

## 7.3 Recommended model
Each node maintains:
- local append-only event/action log
- sync cursor(s)
- local cache of replicated canonical state

Canonical node reconciles from event/action stream and recomputes derived state.

---

# 8. Write-Ahead / Action Log

Vel should maintain a durable append-only log of actions/signals suitable for replication.

This can be thought of as a write-ahead or event log, even if implemented through existing signals/events tables plus replication metadata.

## 8.1 Replication unit examples
- capture created
- signal ingested
- commitment done
- nudge snoozed
- nudge resolved
- suggestion accepted
- thread link created

## 8.2 Important property
Replication should be at the level of:
> “what happened”

not:
> “here is my private guessed state blob”

This keeps reconciliation sane.

---

# 9. Offline Mode

Edge nodes must support degraded offline operation.

## 9.1 Required offline actions
These must work locally when canonical node is unreachable:

- create capture
- mark nudge/commitment done
- snooze nudge
- submit feedback
- read latest cached current context
- read recent cached commitments / nudges

## 9.2 Optional later offline features
- limited local explanation
- limited local search over cached data
- lightweight local synthesis
- local transcript capture

## 9.3 Not required offline initially
- authoritative cross-device coordination
- full global synthesis
- full historical search if not cached
- remote notification routing

---

# 10. Handoff Model

Vel should support seamless handoff across devices.

Examples:
- watch nudge acknowledged, desktop reflects it
- desktop voice capture appears on phone
- phone offline capture syncs to canonical node later
- desktop temporary authority hands state back to NAS later

## 10.1 Handoff principle
Clients should feel like different surfaces over one assistant state, not separate assistants.

## 10.2 Implementation rule
All devices should use shared action semantics:
- Done
- Snooze
- Capture
- Accept suggestion
- Reject suggestion

---

# 11. Conflict Resolution

Conflicts will happen.

Initial conflict policy should be simple and explicit.

## 11.1 General rule
Use timestamped event/action ordering with deterministic domain rules.

## 11.2 Specific domain rules

### Done vs Snooze
- `done` dominates `snooze`

### Resolved nudge vs later stale escalation
- resolved state dominates; stale escalation replay must be ignored

### Accepted suggestion vs stale reject replay
- later timestamp wins unless domain-specific rule requires otherwise

### Duplicate signals
- dedupe by adapter/source_ref where possible

## 11.3 Important rule
Never silently keep impossible states such as:
- nudge both resolved and active
- commitment done but downstream nudge still escalating without explanation

---

# 12. Sync Semantics

## 12.1 Sync direction
Edges sync actions/signals upstream to canonical node.  
Canonical node syncs authoritative derived state and metadata back downstream.

## 12.2 Sync contents
Upstream:
- local signals
- local user actions
- acknowledgements
- feedback
- locally captured transcripts/notes

Downstream:
- current context snapshot
- active nudges
- commitments
- threads
- artifacts metadata
- latest synthesis summaries or references

## 12.3 Sync mode
Eventual consistency is acceptable.  
The system does not need perfect real-time consensus for MVP.

---

# 13. Synthesis Distribution

Synthesis may happen on different nodes depending on cost and availability.

## 13.1 Canonical synthesis
Canonical synthesis should run on a **powerful designated node**.

Preferred candidates:
- desktop
- VPS
- NAS only if capable

Use canonical synthesis for:
- weekly synthesis
- project synthesis
- self-review
- intent-vs-behavior synthesis

## 13.2 Local/edge synthesis
Lightweight synthesis may happen locally later for:
- small summaries
- immediate user-facing explanations
- low-cost convenience features

But authoritative reflective artifacts should be generated on the designated synthesis-capable node when possible.

## 13.3 Important rule
Do not require the NAS to run heavyweight LLM workloads if it is not suited for that.

---

# 14. Node Capabilities Model

Nodes should declare capabilities.

Suggested capability flags:
- `can_store_canonical`
- `can_run_policy_engine`
- `can_run_synthesis`
- `can_run_llm`
- `can_send_notifications`
- `can_capture_local_signals`

Examples:

## NAS
- canonical storage: yes
- synthesis/LLM: maybe limited

## Desktop
- canonical storage: yes (temporary or preferred fallback)
- synthesis/LLM: yes

## VPS
- canonical storage: yes (fallback)
- synthesis/LLM: yes depending on provisioning

## iPhone
- canonical storage: no
- local cache/signals: yes

## Watch companion
- direct canonical storage: no
- ack + signal capture via phone: yes

This capability model should guide scheduling and handoff.

---

# 15. Tailscale Assumptions

## 15.1 Primary network assumption
Yes: Tailscale is the primary expected connectivity layer.

Use it for:
- node discovery
- secure connectivity
- stable hostnames via MagicDNS where available
- private service access across desktop/NAS/VPS/devices

For the concrete sync contract that treats Tailscale as a first-class cluster transport, see:

- `docs/specs/vel-cluster-sync-spec.md`

## 15.2 But not a hard dependency
Vel should still work locally without Tailscale.

Meaning:
- local-only mode must be possible
- direct localhost or same-device operation must function
- clients should degrade gracefully if Tailscale unavailable

## 15.3 Discovery preference
Clients should attempt nodes in configured order:
1. local VELD if present
2. NAS VELD
3. desktop VELD
4. VPS VELD
or similar, depending on resolved canonical preference configuration

Exact probe order may differ by client type, but canonical preference should remain explicit.

---

# 16. Client-Side Local Caching

Edge clients should maintain a local cache of:
- latest current context
- active nudges
- relevant recent commitments
- basic thread summaries later
- latest synthesis summary references later

This enables:
- fast rendering
- useful offline mode
- reduced dependence on round-trip latency

Cache must be treated as:
- best-effort
- stale-aware
- overwriteable by canonical sync

Do not let cache become a forked ontology.

---

# 17. Browser / Worker Stretch Goal

Browser local worker / web worker support is explicitly a stretch goal.

Allowed future role:
- local cache
- command forwarding
- lightweight voice/text interaction
- temporary edge-node behavior in browser

Not required for early implementation.

Do not let browser-worker aspirations complicate the core architecture now.

---

# 18. Ambient Computing Principles

Vel may later become more ambient, but the architecture should grow into that rather than assume it now.

Ambient principles:
- context should persist across surfaces
- notifications should reach the most appropriate available surface
- edges may contribute signals even when not canonical
- user should feel one assistant following state across environments

Examples:
- watch buzzes when away from desktop
- desktop voice takes over when seated at workstation
- phone shows same active nudge/context
- smart speaker later could read morning briefing from the same current context

---

# 19. Security / Auth Assumptions

For this phase, assume:
- personal single-user system
- devices connected via Tailscale where possible
- no multi-user auth model required yet
- device trust is managed operationally, not through full enterprise identity architecture

Still:
- replication/authentication boundaries should be explicit
- future token/device auth should not be blocked
- avoid writing code that assumes zero authentication forever

---

# 20. Recommended Implementation Order

Implement this in stages.

## Stage 1
- define node roles
- make VEL CLI operate as client to local/remote VELD
- add local fallback mode conceptually

## Stage 2
- add append-only action/signal sync primitive
- add sync cursors / replication metadata
- support offline captures and done/snooze

## Stage 3
- sync current context + active nudges back to clients
- build local cache behavior

## Stage 4
- support temporary canonical authority on desktop fallback
- reconcile back to NAS

## Stage 5
- add designated synthesis-capable node routing
- support reflective artifact generation on desktop/VPS

Do not begin with full peer-to-peer complexity.

---

# 21. Testing Requirements

## 21.1 Offline action tests
- create capture offline
- done/snooze offline
- sync later to canonical node

## 21.2 Conflict tests
- done vs snooze race
- stale escalation after resolution
- duplicate signal replay

## 21.3 Handoff tests
- watch/phone ack updates desktop state
- desktop fallback authority syncs back to NAS
- cached current context replaced by canonical state on reconnect

## 21.4 Capability tests
- synthesis routes to compute-capable node
- canonical authority fallback order works

---

# 22. Practical Engineering Rules

1. Canonical authority must be explicit.
2. Events/actions replicate; derived state can be recomputed.
3. CLI is not the daemon.
4. Offline mode must support capture + done/snooze at minimum.
5. Done dominates snooze in conflicts.
6. Treat Tailscale as the preferred network, not the only possible one.
7. Keep multi-master fantasies out of MVP.
8. Compute-heavy synthesis belongs on powerful nodes.
9. Cache is useful, but canonical truth still exists.
10. Design for handoff, not for isolated apps.

---

# 23. Success Criteria

This distributed/ambient architecture is successful when:

- one preferred canonical node exists
- clients can fall back gracefully
- offline capture and acknowledgement work
- action/event replication reconciles correctly
- desktop can temporarily take over when NAS is offline
- current context and nudges hand off cleanly across devices
- synthesis can run on a powerful node without forcing all nodes to be powerful
- users experience Vel as one assistant across surfaces

---

# 24. Final Summary

Vel’s distributed architecture should feel like:

- one persistent assistant state
- one preferred canonical brain
- multiple synchronized limbs
- graceful offline degradation
- clean handoff across desktop, phone, watch, and future ambient surfaces

In short:

> VELD is the brain when reachable, edges are capable limbs when needed, and the whole system should reconcile through events rather than pretending every device is equally omniscient.

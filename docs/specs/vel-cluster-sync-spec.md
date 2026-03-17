# Vel Cluster Sync Spec

Status: Planned distributed synchronization specification  
Audience: coding agent, runtime implementer, client implementer  
Purpose: define the concrete sync mechanism for multi-node Vel deployments, including swarm participation, authority handoff, and cluster-aware state propagation

---

## Purpose

Vel already has high-level distributed rules:

- one preferred canonical brain
- edge clients with local cache and action queues
- events/actions first, state second

This spec turns those principles into a concrete sync contract suitable for:

- canonical node plus fallbacks
- multi-client swarm execution
- temporary authority handoff
- cluster-aware worker placement

This is not a multi-master consensus design.

The goal is:

> **coherent event/action sync across a small trusted personal cluster**

not:

> **general-purpose distributed database heroics**

---

## Goals

The sync layer must support:

- append-only upstream sync of actions and signals
- downstream sync of authoritative derived state
- explicit authority identity
- temporary authority handoff and reconciliation
- sync cursors and replay
- idempotent retries
- small trusted cluster membership
- swarm-aware routing and worker discovery
- first-class Tailscale transport and tailnet-aware node routing

---

## Non-Goals

Do not build:

- general peer-to-peer multi-master sync
- raw SQLite file replication as the primary mechanism
- opaque state snapshots as the source of truth
- quorum consensus for MVP
- unbounded service discovery

Vel is a personal trusted cluster, not a generic database product.

---

## Core Principle

All nodes sync around one explicit authority epoch.

At any moment, one node is:

- preferred canonical authority, or
- temporary authority during failover

All other nodes are:

- edge nodes
- compute workers
- read-heavy clients
- queued-action producers

This authority must be explicit in sync metadata, not inferred from vibes.

---

## Tailscale As First-Class Transport

Tailscale should be treated as a first-class Vel cluster transport for MVP and early production use.

This means:

- tailnet connectivity is the default assumption for cross-device communication
- MagicDNS names or configured Tailscale IPs should be first-class node addresses
- node reachability, authority routing, and worker placement should all understand tailnet presence
- loss of Tailscale connectivity is a first-class degraded-state condition, not an incidental network error

Tailscale is not the sync protocol itself.

Vel still needs:

- its own authority model
- its own append-only sync semantics
- its own cursors and idempotency
- its own cluster metadata

But the operational default should be:

> **Vel cluster traffic rides over the tailnet whenever the cluster spans multiple machines or devices.**

### Required transport assumptions

The sync layer should natively support:

- configured Tailscale base URLs per node
- MagicDNS hostname preference where available
- rebinding to a different authority node on the same tailnet
- tailnet reachability checks separate from generic HTTP health
- degraded fallback to localhost or same-LAN endpoints when Tailscale is unavailable

### Why this matters

Without making Tailscale first-class, the cluster design will drift into vague "some network exists" thinking and the real deployment path across NAS, desktop, VPS, phone, and laptop will remain underspecified.

---

## Node Roles

### Authority Node

Owns:

- canonical append-only event/action log
- canonical object stores
- derived-state recomputation
- swarm supervision/integration
- sync acceptance and downstream publication

### Edge Node

Owns:

- local cache
- queued local actions/signals
- sync cursors
- stale-aware rendering

### Compute Node

Owns:

- heavy bounded execution
- optional local copies of evidence/state needed for work

It does not become authority unless explicitly promoted.

---

## Cluster Model

Vel should support a small trusted cluster with explicit membership.

Expected nodes:

- NAS
- desktop
- VPS
- phone
- watch via phone
- optional laptop

Each node should have:

- `node_id`
- `node_class`
- `display_name`
- `capabilities`
- `priority_order`
- `trust_state`
- `last_seen_at`

Cluster membership should be configured, not discovered by arbitrary broadcast protocols.

Cluster membership should include transport metadata for each node:

- `tailscale_hostname` optional
- `tailscale_ip` optional
- `local_lan_url` optional
- `localhost_url` optional
- `sync_base_url`

Tailscale is the preferred cross-device transport, but it does not replace Vel sync semantics.

---

## Sync Objects

Vel sync should distinguish three categories.

### 1. Replicated facts

Append-only or mutation events that describe what happened.

Examples:

- capture created
- nudge snoozed
- commitment done
- signal ingested
- suggestion accepted
- swarm work unit completed
- artifact metadata written

### 2. Derived state

Authoritative state materialized by the authority node.

Examples:

- current context snapshot
- active nudges
- current commitments state
- latest thread summaries
- latest synthesis references

### 3. Cluster control metadata

Examples:

- current authority node
- authority epoch
- per-node sync cursor
- worker availability
- node health

---

## Upstream / Downstream Split

### Upstream sync

Flows from edge or compute nodes to the current authority node.

Contains:

- queued user actions
- local captures
- local signals
- worker receipts
- work-unit completion records

### Downstream sync

Flows from authority node to other nodes.

Contains:

- accepted log entries
- canonical object updates
- derived-state snapshots
- authority metadata
- worker-placement hints later

This keeps all nodes aligned without pretending each node computes truth independently.

---

## Log Model

The sync substrate should be an append-only replicated log with per-entry metadata.

Required fields:

- `log_entry_id`
- `authority_epoch`
- `origin_node_id`
- `origin_local_seq`
- `entry_type`
- `object_kind`
- `object_id` optional
- `occurred_at`
- `payload_json`
- `idempotency_key`
- `causation_id` optional
- `correlation_id` optional

Important rule:

The authority node may reject or normalize incoming entries, but it must preserve auditability.

---

## Authority Epoch

Every accepted log entry and every downstream state publication must include:

- `authority_node_id`
- `authority_epoch`

When authority changes:

- increment epoch
- publish authority-change event
- require downstream nodes to treat prior authority assumptions as stale

This prevents subtle split-brain confusion during failover and recovery.

---

## Cursors

Each node must maintain explicit sync cursors.

Minimum cursors:

- `upstream_last_acked_local_seq`
- `downstream_last_applied_log_entry_id`
- `downstream_last_applied_state_version`

Optional later:

- per-stream cursors for actions, derived state, artifacts, and swarm telemetry

Cursors must be durable.

Without durable cursors, retries become duplication machines.

---

## Idempotency

All upstream writes must be idempotent.

Required idempotency keys for:

- captures
- acknowledgements
- commitment mutations
- signal ingests when a stable external reference exists
- action-worker receipts
- work-unit completion uploads

Authority node behavior:

- duplicate idempotency key with same payload: acknowledge existing result
- duplicate idempotency key with conflicting payload: reject and emit conflict metadata

---

## Conflict Model

The sync layer uses deterministic conflict handling, not distributed consensus.

### Domain rules

- `done` beats `snooze`
- stale escalation must not resurrect resolved nudge state
- later accepted authority epoch beats stale prior-epoch assumptions
- duplicate source-ref signals dedupe where possible

### Conflict handling flow

1. authority receives upstream entry
2. validate epoch assumptions and object preconditions
3. apply domain conflict rules
4. emit accepted, normalized, or rejected result
5. publish downstream canonical result

Never allow silent impossible states to persist.

---

## State Publication Model

Derived state should be published as versioned snapshots, not treated as the primary mutable truth.

Required metadata per snapshot:

- `state_kind`
- `state_version`
- `authority_epoch`
- `generated_at`
- `payload_json`

Examples:

- `current_context`
- `active_nudges`
- `commitments_slice`
- `threads_slice`
- `synthesis_refs`

Edge clients may cache these snapshots.

They must never treat a locally edited snapshot as canonical state.

---

## Cluster-Aware Swarm Sync

The swarm layer needs more than generic client sync.

It also needs synchronized visibility into:

- worker availability
- work-unit assignment
- work-unit completion
- result integration status

### Required swarm sync objects

- `worker_presence`
- `worker_capacity`
- `work_unit_assigned`
- `work_unit_started`
- `work_unit_completed`
- `work_unit_failed`
- `integration_completed`

These objects should replicate through the same sync substrate so the authority node can:

- rebalance work
- detect stuck workers
- avoid duplicate assignment
- replay swarm state after restart

---

## Worker Presence And Capacity

Every node that can run swarm work should publish lightweight presence:

- `node_id`
- `worker_classes`
- `max_concurrency`
- `current_load`
- `reachability`
- `latency_class`
- `compute_class`
- `power_class`
- `last_heartbeat_at`

Presence is advisory metadata, not a source of truth about user state.

Heartbeats should be cheap and bounded.

### Queued Work Assignment Lifecycle

Every queued work item must carry a unique `work_request_id`, the target node, target worker class, requested capability, and authority epoch. This metadata is propagated through the existing `client_branch_sync_requested` / `client_validation_requested` signals, and it should remain attached until the work completes (success, failure, or cancel).

Workers claim, start, and complete work via heartbeat/assignment-aware surfaces such as `POST /v1/sync/heartbeat` plus explicit receipt endpoints. Each heartbeat refreshes the worker registry and can include the optional `work_request_id` the worker is handling along with `component` (API/branch/validation) and a `receipt_state` (`claimed`, `started`, `completed`, `failed`). Receipts must be persisted (even temporarily) so replay, retry, and operator inspection can prove what ran, when, and whether it succeeded.

Receipts also enable idempotent retries: before re-queuing a request, the authority checks the latest receipt for that `work_request_id`. Duplicate requests must install idempotency keys and reuse receipts rather than re-executing side effects. Workers should emit receipts for every attempt along with optional failure reasons or logs so the supervisor can decide when to escalate. Queue inspection should filter out requests whose latest receipt is already terminal (`completed`, `cancelled`) and expose only work that is still pending or reclaimable.

Operator surfaces (`GET /v1/sync/cluster`, `GET /v1/cluster/workers`, CLI `vel sync workers`) expose placement history, current receipt state, and any outstanding `work_request_id` assignments. These inspection surfaces must show the last seen authority epoch, tailnet state, and whether the last receipt was `completed` or still `in-flight`. This helps detect stuck work units and ensures the planner can reassign stale receipts.

---

## Load-Balanced Placement Support

The sync layer must support scheduler decisions with cluster-aware metadata.

Required placement inputs available through sync:

- authority node identity
- currently reachable workers
- per-worker capacity summary
- per-worker recent failure rate
- data locality hints
- assignment receipts

This allows the swarm scheduler to place work intelligently without out-of-band guessing.

---

## Scheduler & Retry/Reclaim Policy

The scheduler is the first consumer of queued receipts. It drives placement, retries, and reclamation through the newly shipped queue endpoints:

- `GET /v1/sync/work-queue` shows all pending work for a node–worker-class combination after filtering out work whose latest receipt is terminal (`completed`, `cancelled`).
- `POST /v1/sync/work-assignments` claims a work request and records a `claimed` receipt; the scheduler includes its `worker_id`, `work_request_id`, and the targeted `node_id`.
- `PATCH /v1/sync/work-assignments` moves receipts through `started`, `completed`, `failed`, or `cancelled` so both the scheduler and operator surfaces can follow progress.
- `GET /v1/sync/work-assignments` exposes the receipt history for inspection, replay, and eventual audit.

The scheduler must treat receipts older than the configured stale window (currently ~300 seconds) as reclaimable and may reassign the work unless the latest receipt is `completed` or `cancelled`. Duplicate incoming requests should look up the latest receipt and reuse it if it is still `claimed`, `started`, or `completed`; only a terminal/fresh failure or stale timeout should cause the scheduler to enqueue the unit again.

Failure receipts should carry reason metadata so the scheduler can escalate, rerun on a different worker class, or present a clarification request rather than looping forever. Queue inspection also serves as a live backlog view so the scheduler can throttle new dispatches when too many items are already pending or when the configured `queue_depth` exceeds the worker’s advertised capacity.

These scheduler-friendly receipts are the bridge between simple queue routing and full-fledged DAG scheduling. They let the swarm layer keep per-unit bookkeeping durable, survive restarts, and compose retry/reclaim policies while the supervisory code still owns the final integration step.

---

## Temporary Authority Handoff

When the preferred authority disappears:

1. eligible fallback node detects sustained authority loss
2. fallback node promotes itself according to configured priority order
3. new authority increments `authority_epoch`
4. new authority begins accepting upstream entries
5. downstream nodes rebind to new authority

When preferred authority returns:

1. returned node does not automatically reclaim authority
2. current temporary authority syncs its authoritative log forward
3. explicit reassignment or policy-based reassignment occurs
4. epoch increments again if authority changes back

This prevents oscillation and hidden split-brain behavior.

---

## Reconciliation After Failover

After failover, reconciliation must proceed at the log level.

Required behavior:

- replay accepted temporary-authority log entries into preferred authority
- re-run derived-state computation on preferred authority
- publish refreshed downstream snapshots
- preserve old authority epochs for audit

Do not attempt to merge by comparing arbitrary database files.

---

## Discovery And Routing

Cluster sync should expose explicit routing metadata:

- preferred authority hostname/URL
- fallback authority order
- node reachability status
- sync endpoints

Clients should probe in configured order, but sync responses should also advertise the currently active authority so clients can rebind quickly.

Suggested response metadata:

- `active_authority_node_id`
- `active_authority_epoch`
- `sync_base_url`
- `sync_transport` (`tailscale`, `localhost`, `lan`, `direct`)
- `cluster_view_version`

Preferred routing order should be:

1. localhost when same-device
2. active authority over Tailscale
3. configured LAN fallback when explicitly allowed
4. degraded/offline mode

If a client is on the tailnet, it should prefer tailnet routing over ad hoc LAN guessing.

---

## Security Assumptions

For MVP, assume:

- single trusted user
- small trusted device set
- private network where possible

Still require:

- per-device identity
- signed or authenticated sync sessions
- explicit node trust state
- rejection of writes from unknown nodes

Tailscale node identity may contribute to trust bootstrap, but Vel should still maintain its own trusted-node registry.

Do not hardcode "all reachable nodes are trusted forever."

---

## Suggested Endpoints

These are interface-level suggestions, not final API law.

### Upstream

- `POST /v1/sync/actions`
- `POST /v1/sync/signals`
- `POST /v1/sync/swarm/events`
- `POST /v1/sync/heartbeat`

### Downstream

- `GET /v1/sync/bootstrap`
- `GET /v1/sync/log?after=...`
- `GET /v1/sync/state?after_version=...`
- `GET /v1/sync/cluster`

### Authority

- `GET /v1/cluster/authority`
- `POST /v1/cluster/claim-temporary-authority`

These endpoints may later collapse into fewer surfaces, but the responsibilities should remain distinct.

### Currently shipped minimal surfaces

As of the current repo state, the following read/write subset exists:

- `GET /v1/cluster/bootstrap`
- `GET /v1/cluster/workers`
- `GET /v1/sync/bootstrap`
- `GET /v1/sync/cluster`
- `POST /v1/sync/heartbeat`
- `GET /v1/sync/work-assignments`
- `POST /v1/sync/work-assignments`
- `PATCH /v1/sync/work-assignments`
- `GET /v1/sync/work-queue`
- `POST /v1/sync/actions`

This is only a bootstrap slice of the full cluster-sync design.

What is present now:

- authority metadata in bootstrap and sync-cluster views
- Tailscale-first endpoint selection metadata
- cache hydration for current context, nudges, and commitments
- heartbeat-backed worker presence registry with expiry
- batched low-risk client action ingestion
- worker-aware routing for queued validation and branch-sync requests
- receipt-backed work claim/start/complete/fail lifecycle
- queue inspection for pending node/worker-class work after receipt-state filtering

What is not yet present:

- durable multi-node membership registry
- append-only downstream log replication
- temporary authority claim/handoff
- cross-node scheduler-assignment replication
- richer retry policy beyond duplicate suppression and stale-receipt reclaim

---

## Implementation Order

1. Add durable per-node identities and sync cursors.
2. Add append-only upstream action/signal sync with idempotency.
3. Add downstream state publication for current context, nudges, and commitments.
4. Add authority epoch metadata and temporary authority handoff.
5. Add swarm worker presence and assignment events.
6. Add cluster-aware routing and load-balancing metadata.

Do not begin with automatic failover across every node type.

---

## Testing Requirements

### Sync basics

- offline capture syncs once and only once
- duplicate upstream action is idempotent
- downstream snapshot updates replace stale cache cleanly
- tailnet-routed sync succeeds against configured MagicDNS or Tailscale endpoint

### Failover

- temporary authority promotion increments epoch
- downstream clients rebind to new authority
- preferred authority later reconciles without duplicate effects
- authority rebinding over Tailscale updates clients without manual endpoint edits

### Swarm

- worker presence heartbeats expire correctly
- overloaded worker stops receiving new assignments
- work unit can be retried on another worker without duplicate side effects

### Conflict rules

- done vs snooze resolves deterministically
- stale prior-epoch replay is rejected or normalized
- duplicate signal source_ref dedupes

---

## Acceptance Criteria

The first acceptable cluster sync implementation satisfies all of these:

1. Nodes sync actions/signals upstream through append-only idempotent entries.
2. Authority node publishes versioned downstream state snapshots.
3. Every sync exchange carries explicit authority identity and epoch.
4. Temporary authority can take over and later reconcile back to preferred authority.
5. Swarm worker presence and work-unit lifecycle can be synchronized across nodes.
6. Cluster-aware load balancing has the metadata it needs without bypassing authority rules.
7. Tailscale is a first-class transport in routing, discovery metadata, failover, and test coverage.
8. No raw database-file sync is required for correctness.

That is the correct sync foundation for a load-balanced multi-client Vel swarm.

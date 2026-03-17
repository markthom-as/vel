---
title: Vel Client Connection And Sync Milestone Spec
status: proposed
owner: runtime / clients / web
created: 2026-03-17
updated: 2026-03-17
---

# Purpose

This spec defines the immediate milestone for Vel client connectivity and sync.

The milestone is narrower than full tester-readiness and much narrower than the long-term cluster/swarm vision. Its job is to make one authority node and one or more clients connect, bootstrap, surface freshness correctly, queue low-risk actions, and recover from common failures without forcing the operator to reconstruct the system from scattered docs.

# Why This Milestone Exists

Vel already has partial substrate for client sync:

- `GET /v1/cluster/bootstrap`
- `GET /v1/sync/bootstrap`
- `POST /v1/sync/actions`
- `GET /v1/sync/cluster`
- Apple bootstrap clients with offline cache and queued actions
- persisted sync metadata for `node_display_name`, `tailscale_base_url`, and `lan_base_url`
- macOS local-source auto-discovery
- web settings surfaces and diagnostics

What is missing is a consolidated milestone that answers one practical question:

> what exactly has to exist so a client can connect to Vel, stay meaningfully in sync, and tell the operator when that contract is degraded?

Today those requirements are spread across:

- cluster sync specs,
- Apple offline/client specs,
- web operator specs,
- multi-client swarm tickets,
- Apple monorepo tickets,
- docs and tester-readiness planning.

This document makes the immediate slice explicit.

# Milestone Definition

The milestone is complete when Vel can reliably support this flow:

1. an operator configures one authority node,
2. a client discovers or imports the authority endpoint set,
3. the client verifies node identity and bootstrap reachability,
4. the client hydrates canonical cached state,
5. the client can submit low-risk queued actions with idempotent replay,
6. the operator can see freshness, degraded state, and sync errors clearly,
7. the operator has one place to inspect which clients are connected and whether they are healthy.

This is a client-connection milestone, not a distributed-replication milestone.

# Out Of Scope

Do not block this milestone on:

- authority handoff,
- multi-authority failover,
- generalized multi-node replication,
- scheduler-driven swarm execution,
- app-store distribution,
- fully productized onboarding/installers,
- advanced integration-expansion work.

If a requirement is only needed for broader swarm execution, it should not gate this milestone.

# Current Truth Baseline

Per `docs/status.md`, the following are already real and should be treated as the baseline, not re-speculated:

- cluster bootstrap metadata exists,
- sync bootstrap payload exists,
- low-risk sync action batching exists,
- worker presence and queue inspection exist,
- Apple clients already cache bootstrap data and queue actions,
- the web/operator surface already exposes some sync metadata and settings controls,
- Tailscale/LAN/localhost endpoint metadata is already part of the system model.

This milestone should consolidate and finish the missing connection/sync contract around those shipped pieces.

# Canonical User Story

## Operator side

The operator runs `veld`, confirms the authority node is healthy, and exposes one preferred client-reachable endpoint plus optional fallbacks.

## Client side

The client receives endpoint metadata, reaches the authority node, verifies the intended node, hydrates current context/nudges/commitments/bootstrap metadata, and shows sync freshness.

## Offline/degraded side

If the authority becomes unreachable, the client continues to render cached state, queues allowed low-risk actions, and exposes stale/disconnected state explicitly.

## Recovery side

When connectivity returns, the client replays queued actions idempotently, refreshes bootstrap state, and clears degraded status or reports the precise remaining error.

# Milestone Workstreams

## 1. Authority bootstrap contract

Vel needs one daemon-side contract that defines:

- authority identity,
- preferred endpoint,
- fallback endpoints,
- sync transport labels,
- bootstrap payload version,
- readiness of the authority node for clients.

Existing pieces should be normalized rather than replaced.

Required outputs:

- stable authority identity in bootstrap responses,
- one ordered endpoint set,
- explicit protocol/version fields,
- explicit readiness flags for “client can connect now”.

## 2. Client bootstrap contract

Every client surface should use the same logical bootstrap sequence:

1. resolve/import endpoint candidates,
2. call health/bootstrap,
3. verify authority identity,
4. hydrate cached state,
5. record freshness/sync metadata,
6. expose degraded state when any step fails.

Apple and web do not need identical UI, but they must not invent different sync semantics.

## 3. Local cache and queued-action spine

The milestone requires one consistent offline/degraded model:

- canonical server state cached locally,
- bounded set of allowed queued actions,
- idempotency keys on queued mutations,
- ordered replay,
- durable retry/error state,
- visible pending/failed action state.

The Apple bootstrap clients already approximate this; the milestone is to formalize and complete that contract rather than leaving it as bootstrap-only behavior.

## 4. Freshness and degraded-state UX

Clients must consistently represent:

- `fresh`
- `aging`
- `stale`
- `error`
- `disconnected`

Operators must be able to tell the difference between:

- no data yet,
- stale cached data,
- missing configuration,
- bootstrap failure,
- action replay failure.

## 5. Client/operator sync visibility

Vel needs one canonical operator view for client connectivity and sync health.

Minimum required fields:

- `client_id` or equivalent durable identity,
- `node_id`,
- display name,
- platform/client type,
- protocol/build version if known,
- current authority epoch/id if applicable,
- last successful bootstrap/sync time,
- pending queued action count,
- last sync error,
- transport/reachability metadata.

This may live in `/v1/sync/cluster` initially, but the view contract itself should be explicit.

# Required Milestone Deliverables

## Deliverable A: normalized bootstrap/readiness DTO

Add or normalize a transport DTO that combines:

- authority identity,
- endpoint candidates,
- transport preference,
- readiness flags,
- protocol version,
- freshness timestamps.

This should become the common bootstrap contract for Apple and web clients.

## Deliverable B: client-link/import path

For this milestone, “linking” can remain simple.

Required:

- clients can ingest one normalized endpoint bundle,
- clients do not require manual multi-field re-entry for normal setup.

This may initially be a copyable JSON/text bundle rather than a full tokenized pairing system.

## Deliverable C: shared sync-state semantics

Define one shared client-side state vocabulary for:

- bootstrap in progress,
- healthy,
- stale,
- disconnected,
- queued action pending,
- replay failed,
- version mismatch.

## Deliverable D: operator sync-health surface

Expose a single surface that answers:

- which clients are connected,
- which node they think is authority,
- when they last synced,
- whether they have pending or failed actions,
- which transport they are using,
- what the next recovery step is.

## Deliverable E: docs path

The user/operator docs must include one short path for:

- choosing an endpoint,
- connecting a client,
- interpreting freshness,
- recovering from disconnected/stale state.

# Canonical API/Contract Requirements

Exact route names may evolve, but the milestone requires these conceptual contracts.

## Health and readiness

- daemon reachable
- authority node identified
- client-safe readiness status exposed

## Bootstrap payload

- current context
- active nudges
- relevant commitments
- authority metadata
- endpoint metadata
- freshness metadata

## Action submission

- idempotent low-risk action submission
- durable acknowledgement or actionable failure
- replay-safe semantics

## Cluster/client view

- consolidated read model for connected clients and worker-like nodes
- operator-facing sync health interpretation

# Cross-Spec Consolidation

This milestone supersedes ad hoc reading across multiple documents by selecting the relevant slice from each:

## Primary authority docs for this milestone

- `docs/status.md`
- `docs/specs/vel-client-connection-and-sync-milestone-spec.md`
- `docs/tickets/client-connect-sync/README.md`

## Source specs this milestone draws from

- `docs/specs/vel-cluster-sync-spec.md`
- `docs/specs/vel-apple-offline-mode-spec.md`
- `docs/specs/vel-apple-and-voice-client-spec.md`
- `docs/specs/vel-web-operator-surface-spec.md`
- `docs/specs/vel-tester-readiness-onboarding-spec.md`

## Source ticket packs this milestone draws from

- `docs/tickets/multi-client-swarm/`
- `docs/tickets/ios-watch-monorepo/`
- `docs/tickets/web-ui-convergence/`
- `docs/tickets/vel-docs/`

Interpretation rule:

- when those documents discuss broader future architecture, this milestone takes only the subset required for connection and sync,
- when any of them conflict with shipped behavior, `docs/status.md` wins.

# Execution Lanes

## Lane 1: daemon bootstrap normalization

Focus:

- normalize authority/bootstrap/readiness DTOs,
- publish one endpoint set,
- make bootstrap versioning explicit.

## Lane 2: Apple sync spine completion

Focus:

- local cache contract,
- durable queued actions,
- replay/error semantics,
- authority verification on reconnect.

## Lane 3: web setup and degraded-state UX

Focus:

- settings/control IA for connection and sync controls,
- freshness/degraded-state semantics,
- recovery guidance close to the failing surface.

## Lane 4: operator client-sync view

Focus:

- enumerate clients,
- show sync freshness and errors,
- surface mismatch/staleness clearly.

## Lane 5: docs consolidation

Focus:

- one documented setup path,
- one documented sync-health interpretation path,
- one documented recovery path.

# Acceptance Criteria

This milestone is complete when:

1. one authority node can publish a normalized bootstrap/readiness contract for clients,
2. Apple and web clients consume the same logical endpoint/bootstrap model,
3. a client can reconnect after temporary loss and replay queued low-risk actions safely,
4. clients render consistent freshness/degraded semantics,
5. operators can inspect connected clients and their sync health from one canonical surface,
6. the setup and recovery path is documented without requiring the reader to chase cluster/swarm specs.

# Recommended Immediate Sequence

1. normalize daemon bootstrap/readiness contract,
2. define the shared client sync-state vocabulary,
3. complete Apple queued-action and replay semantics against that contract,
4. implement web/operator freshness and recovery UX,
5. expose the operator client-sync view,
6. update setup and troubleshooting docs.

# Deferred Follow-On Work

After this milestone lands, broader work can build on it:

- richer client-link artifacts,
- multi-node authority handoff,
- append-only upstream/downstream replication,
- broader swarm scheduling,
- packaged tester distribution.

Those are valuable, but they should no longer be prerequisites for simply getting clients connected and syncing.

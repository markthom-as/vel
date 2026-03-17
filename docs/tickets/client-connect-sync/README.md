---
title: Client Connect And Sync Milestone
status: active
owner: agent
class: convergence
authority: design
source_of_truth: docs/status.md
created: 2026-03-17
updated: 2026-03-17
---

# Client Connect And Sync Milestone

This directory is a cross-pack execution index for the immediate milestone: get clients connected to Vel and meaningfully syncing.

It is not a new implementation ledger. Use `docs/status.md` for shipped behavior.

The canonical milestone spec is:

- [docs/specs/vel-client-connection-and-sync-milestone-spec.md](../../specs/vel-client-connection-and-sync-milestone-spec.md)

This README consolidates the existing specs and tickets that already cover pieces of the problem, then narrows them to one execution path.

## Milestone question

What is the minimum complete slice required so a client can:

1. find or import the right authority node,
2. bootstrap canonical state,
3. stay useful while disconnected,
4. replay low-risk actions safely,
5. show sync freshness honestly,
6. let the operator inspect which clients are healthy or degraded?

## Authority and boundaries

This milestone owns:

- client connection/bootstrap/readiness contract,
- offline cache and queued low-risk action semantics,
- sync freshness/degraded-state semantics,
- client/operator sync-health visibility,
- docs needed to connect and recover clients.

This milestone does not own:

- broader swarm scheduling,
- authority handoff,
- provider-expansion work,
- app-store distribution,
- generalized tester packaging beyond what is needed to exercise client sync.

## Canonical source set

Read in this order:

1. [docs/status.md](../../status.md)
2. [docs/specs/vel-client-connection-and-sync-milestone-spec.md](../../specs/vel-client-connection-and-sync-milestone-spec.md)
3. [docs/specs/vel-cluster-sync-spec.md](../../specs/vel-cluster-sync-spec.md)
4. [docs/specs/vel-apple-offline-mode-spec.md](../../specs/vel-apple-offline-mode-spec.md)
5. [docs/specs/vel-web-operator-surface-spec.md](../../specs/vel-web-operator-surface-spec.md)
6. [docs/specs/vel-tester-readiness-onboarding-spec.md](../../specs/vel-tester-readiness-onboarding-spec.md)

## Consolidated execution lanes

### Lane 1: daemon bootstrap/readiness normalization

Goal:

- one normalized authority/bootstrap/readiness contract for all clients.

Primary sources:

- [docs/specs/vel-client-connection-and-sync-milestone-spec.md](../../specs/vel-client-connection-and-sync-milestone-spec.md)
- [docs/specs/vel-cluster-sync-spec.md](../../specs/vel-cluster-sync-spec.md)
- [docs/specs/vel-tester-readiness-onboarding-spec.md](../../specs/vel-tester-readiness-onboarding-spec.md)

Existing shipped substrate to preserve:

- `GET /v1/cluster/bootstrap`
- `GET /v1/sync/bootstrap`
- persisted `node_display_name`, `tailscale_base_url`, `lan_base_url`

Gap to close:

- one explicit readiness/bootstrap DTO and one ordered endpoint model.

### Lane 2: Apple sync spine completion

Goal:

- complete the cache + queued-action + replay contract for Apple clients against the normalized bootstrap model.

Primary sources:

- [docs/specs/vel-apple-offline-mode-spec.md](../../specs/vel-apple-offline-mode-spec.md)
- [docs/specs/vel-apple-and-voice-client-spec.md](../../specs/vel-apple-and-voice-client-spec.md)
- [APPLE-003](../ios-watch-monorepo/03-local-store-and-sync-spine.md)

Existing shipped substrate to preserve:

- cached bootstrap hydration,
- queued low-risk actions,
- endpoint candidate resolution,
- stale-aware/offline fallback behavior.

Gap to close:

- formalized replay semantics, sync-state vocabulary, and authority verification behavior.

### Lane 3: web connection and degraded-state UX

Goal:

- make web setup, freshness, and recovery behavior explicit and consistent.

Primary sources:

- [docs/specs/vel-web-operator-surface-spec.md](../../specs/vel-web-operator-surface-spec.md)
- [WUI-005](../web-ui-convergence/WUI-005-settings-and-integrations-ia.md)
- [WUI-006](../web-ui-convergence/WUI-006-global-page-state-and-freshness-ux.md)

Existing shipped substrate to preserve:

- settings controls for sync metadata,
- sync/bootstrap metadata already surfaced in the UI.

Gap to close:

- control-first connection UI,
- truthful freshness/degraded states,
- next-step recovery affordances.

### Lane 4: operator client-sync view

Goal:

- one operator surface for connected clients and sync health.

Primary sources:

- [SWARM-009](../multi-client-swarm/SWARM-009-swarm-client-view.md)
- [docs/specs/vel-cluster-sync-spec.md](../../specs/vel-cluster-sync-spec.md)
- [docs/specs/vel-client-connection-and-sync-milestone-spec.md](../../specs/vel-client-connection-and-sync-milestone-spec.md)

Existing shipped substrate to preserve:

- `/v1/sync/cluster`
- heartbeat-backed worker/client presence
- queue/work receipt metadata

Gap to close:

- one explicit client-oriented read model with sync freshness, versioning, pending actions, and recovery guidance.

### Lane 5: docs and setup consolidation

Goal:

- one practical path for connecting clients and understanding sync failures.

Primary sources:

- [docs/specs/vel-tester-readiness-onboarding-spec.md](../../specs/vel-tester-readiness-onboarding-spec.md)
- [docs/specs/vel-user-documentation-spec.md](../../specs/vel-user-documentation-spec.md)
- `docs/user/setup.md`
- `clients/apple/README.md`

Gap to close:

- current docs still require too much repo/operator inference for client connection and sync recovery.

## Selected existing tickets

These are the existing tickets most directly aligned to this milestone.

### Must-use tickets

- [APPLE-003](../ios-watch-monorepo/03-local-store-and-sync-spine.md)
- [WUI-005](../web-ui-convergence/WUI-005-settings-and-integrations-ia.md)
- [WUI-006](../web-ui-convergence/WUI-006-global-page-state-and-freshness-ux.md)
- [SWARM-009](../multi-client-swarm/SWARM-009-swarm-client-view.md)

### Context tickets, not milestone gatekeepers

- [SWARM-002](../multi-client-swarm/SWARM-002-cluster-sync-substrate.md)

Reason:

- it contains important long-term replication rules, but broader append-only multi-node replication should not block the immediate client-connect milestone as long as the current authority/bootstrap/action contract is normalized and observable.

## Immediate implementation order

1. normalize daemon bootstrap/readiness and endpoint publication,
2. align Apple sync-state and replay semantics to that contract,
3. make web settings/freshness/recovery UX match the same contract,
4. expose the operator client-sync view,
5. update user/client setup docs around the consolidated path.

## Exit criteria

The milestone can be considered complete when:

1. one authority node exposes a clear client bootstrap/readiness contract,
2. Apple and web clients rely on the same endpoint/bootstrap semantics,
3. degraded/offline behavior is explicit instead of ad hoc,
4. operators can inspect client sync health from one place,
5. the docs explain connection and sync recovery without cross-pack archaeology.

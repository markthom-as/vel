---
title: Canonical Data Sources And Connectors
doc_type: spec
status: proposed
owner: staff-eng
created: 2026-03-17
updated: 2026-03-17
keywords:
  - integrations
  - data sources
  - connectors
  - manifests
index_terms:
  - integration catalog
  - source catalog
  - connector contract
  - local source model
related_files:
  - docs/MASTER_PLAN.md
  - docs/cognitive-agent-architecture/integrations/plugin-system.md
  - crates/vel-core/src/integration.rs
  - docs/user/integrations/README.md
summary: Canonical list of Vel data-source families, current and planned connector modes, and the contract shape that integrations should follow.
---

# Purpose

Define the canonical list of Vel data-source families and the contract that every connector or integration path should follow.

# Problem

Vel already has real integration surfaces in code and user docs, but the contract is fragmented:

- user docs describe shipped integrations
- `vel-core` defines integration families and connection records
- `veld` services implement sync paths
- settings surfaces present integration cards
- capability-broker and connect docs describe future mediated execution

Without one canonical connector model, every new integration risks inventing its own rules.

# Goals

- define one canonical list of data-source families
- define one connector contract that works for local files, snapshots, OAuth APIs, brokered tools, and delegated runtimes
- make source modes and trust boundaries explicit
- tie connector work back to tickets and existing domain types

# Non-Goals

- implementing every planned integration family now
- requiring a remote connector architecture for local-first core loops

# Current State

Current shipped truth lives in [MASTER_PLAN.md](../../MASTER_PLAN.md).

Today, the strongest shipped integration model is still local-first source ingestion plus a small number of credential-backed integrations.

# Proposed Design

## Canonical Integration Families

These families should remain the stable product-facing source categories:

| Family | Current Status | Common Providers / Modes | Canonical Notes |
| --- | --- | --- | --- |
| `calendar` | shipped | local ICS, Google Calendar | schedule and event windows |
| `tasks` | shipped | Todoist | actionable backlog and commitments |
| `activity` | shipped | local snapshots, Apple export | workstation/activity evidence |
| `health` | shipped | local snapshots, Apple export | health and medication context |
| `git` | shipped | local snapshots | recent code and repo activity |
| `messaging` | shipped | local snapshots, Apple export | response debt and thread state |
| `notes` | shipped | local directory | local knowledge input |
| `transcripts` | shipped | local snapshots | conversation and transcript context |
| `documents` | planned | local exports, future connectors | document-centric recall |
| `gaming` | planned | local exports, future connectors | personal-context signal family |

## Canonical Source Modes

Every integration should declare one source mode:

- `local_file`: a user-controlled file such as `.ics` or JSON
- `local_directory`: a user-controlled directory tree such as notes
- `local_snapshot`: a local snapshot export produced by another process
- `oauth_api`: a provider-backed API connection
- `brokered_tool`: an action performed through a capability broker instead of direct raw credentials
- `delegated_runtime`: a future worker or plugin runtime that still obeys connector contracts

## Connector Contract

Every connector should eventually declare:

```json
{
  "integration_family": "calendar",
  "provider_key": "google_calendar",
  "source_mode": "oauth_api",
  "read_capabilities": ["read_events"],
  "write_capabilities": [],
  "secret_mode": "brokered_injection",
  "config_keys": ["google_client_id", "google_client_secret"],
  "source_refs": ["IntegrationSourceRef"],
  "freshness_sla_seconds": 900,
  "emits": {
    "signals": ["calendar_event"],
    "entities": ["IntegrationConnection", "IntegrationConnectionEvent"]
  },
  "allowed_hosts": ["www.googleapis.com"]
}
```

## Connector Rules

### Identity

- Product-facing identity should come from `IntegrationFamily`.
- Provider identity should come from `IntegrationProvider`.
- Connector-specific implementation details should not invent parallel family names.

### Trust Boundary

- Local file and snapshot connectors should remain inspectable and user-controlled.
- OAuth/API connectors should prefer mediated execution and scoped secrets.
- Delegated runtimes must still surface stable manifests, traces, and denial behavior.

### Freshness And Health

- Every connector should expose freshness expectations, last sync state, and degraded/error state.
- Unavailable or stale integrations should degrade confidence explicitly instead of pretending data is fresh.

### Output Provenance

- Signals and higher-level entities should carry `IntegrationSourceRef` or equivalent provenance when sourced from a connector.
- Connector health and connection lifecycle should remain inspectable through `IntegrationConnection` and `IntegrationConnectionEvent`.

### Writes

- Read-only connectors are the default.
- Write-capable connectors must declare the exact capability and the review or confirmation requirement.
- “Can write anything the provider supports” is not an acceptable contract.

# Boundaries

- `vel-core` owns families, providers, connection records, events, and source refs.
- `veld` services own sync execution and runtime mapping.
- the capability broker owns secret mediation and scoped external execution.
- user docs own setup and operational guidance, not connector contract authority.

# Cross-Cutting Traits

- modularity: required — connector implementations should conform to one family/provider contract model.
- accessibility: required — the catalog of source families and modes should be easy to inspect from docs and UI surfaces.
- configurability: required — source paths, provider settings, and participation controls must be explicit and inspectable.
- data logging and observability: required — syncs, failures, auth events, and freshness need structured runtime evidence.
- rewind/replay: affected — snapshot-based and event-based connectors need replay-safe provenance and time semantics.
- composability: required — connectors should plug into shared source refs, connection records, and manifests instead of bespoke per-provider logic.

# Operational Considerations

- keep local-first source modes as the preferred default for core loops
- add new connector families only when they add real user value, not because a provider API exists
- do not broaden capability scope just to make a connector convenient

# Acceptance Criteria

1. The repo has one canonical list of integration families and source modes.
2. New integration work can point to one connector contract doc instead of inventing its own shape.
3. Ticket coverage exists for implementing the connector contract and family catalog in code.

# Open Questions

- Which planned families deserve phase-1 schema coverage versus later implementation attention?
- Should connector manifests live in code first or as generated docs from code?

# Related Terms

- canonical name: data sources and connectors
- aliases: integration catalog, connector contract, source family catalog
- related packs or subsystems: capability broker, plugin system, local sources, sync

# Search Terms

- connector contract
- data source catalog
- integration family
- source mode

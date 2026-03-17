# Plugin / Integration System

Vel may expose a plugin or delegated connector layer for integrations, but it is subordinate to the canonical connector contract, not a parallel architecture.

Use these authorities first:

- [canonical-data-sources-and-connectors.md](canonical-data-sources-and-connectors.md) for source-mode vocabulary and manifest rules
- [data-source-catalog.md](data-source-catalog.md) for concrete provider inventory and rollout status

## Scope

This document covers the delegated-runtime/plugin interpretation of the connector contract:

- how a plugin advertises capabilities
- how it reports freshness, failure, and provenance
- how it participates in capability mediation and deny-by-default execution

It should not redefine integration families or invent a second provider taxonomy.

## Delegated Connector Contract

Each delegated connector should declare:

- `integration_family`
- `provider_key`
- `source_mode` with value `delegated_connector`
- auth requirements and secret handling mode
- read and write capabilities
- freshness expectations and failure semantics
- allowed hosts or resource scopes
- emitted entity and signal schemas

## Rule

Unavailable integrations degrade confidence. They should never be silently assumed healthy.

Delegated connectors should prefer mediated capability execution over handing raw credentials to the calling agent.

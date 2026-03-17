---
title: Ticket 400 - Add external source registry and ontology
status: proposed
owner: codex
priority: high
---

# Goal

Make external awareness explicit in domain code so Todoist, Google Calendar, and Codex project registry are first-class concepts rather than adapter-specific JSON blobs.

# Why

Current Vel has:
- adapters in `crates/veld/src/adapters/`
- integration settings in `services/integrations.rs`
- signals and commitments as durable state

But it lacks:
- a shared external-source vocabulary
- a normalized external item model
- stable project identity

That causes ontology drift and adapter duplication.

# Scope

## Create domain types
New files:
- `crates/vel-core/src/external.rs`
- `crates/vel-core/src/project.rs`

Add:
- `ExternalSourceKind`
- `ExternalItemKind`
- `ExternalItem`
- `ExternalItemLinkKind`
- `ProjectIdentity`
- `ProjectSourceMapping`

## Update exports
Change:
- `crates/vel-core/src/lib.rs`

## Add API-facing shapes if needed
Change:
- `crates/vel-api-types/src/lib.rs`

# Concrete implementation notes

## `ExternalSourceKind`
Include:
- `todoist`
- `google_calendar`
- `codex_project_registry`
- `brain_vault`
- `metrics`

Implement:
- `Display`
- `FromStr`
- serde snake_case conversion

## `ProjectIdentity`
Fields:
- `project_id`
- `slug`
- `display_name`
- `status`
- `project_type`
- `owner`
- `aliases: Vec<String>`
- `metadata_json`

Slug rules:
- lower-case
- preserve dots where meaningful (`spucchi.com`)
- spaces -> hyphens
- collapse duplicate separators

# Code changes

## New tests
Add unit tests for:
- source kind serde round trip
- project slug normalization
- alias matching
- invalid enum strings rejected

# Acceptance criteria

- all adapters can target the same `ExternalItem` type
- project identity has a stable slug model
- repo compiles with new domain exports

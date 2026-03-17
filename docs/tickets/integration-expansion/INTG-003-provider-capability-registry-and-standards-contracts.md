---
id: INTG-003
title: Provider capability registry and standards contracts
status: proposed
priority: P0
estimate: 2-4 days
dependencies:
  - INTG-001
---

# Goal

Define how Vel knows what each provider supports and which open standards or exchange formats it can use.

# Scope

- Add a capability manifest model for providers.
- Track support for:
  - multiple connections
  - stable external ids
  - person-bearing data
  - local-first ingest
  - import/export
  - writeback
  - scheduling/webhook/watcher sync
- Add standards metadata for:
  - ICS / CalDAV
  - vCard / CardDAV
  - Markdown / plaintext
  - JSON transcript envelopes
  - WebVTT / SRT
  - CSV import/export

# Deliverables

- provider capability types
- provider capability registry loader
- docs for standards strategy
- fixtures for provider manifest entries

# Acceptance criteria

- New providers can be registered without inventing ad hoc booleans in UI code.
- Operator/API surfaces can show what a provider can and cannot do.
- Standards support is explicit instead of tribal knowledge.

# Notes

This is the difference between an integration layer and a pile of adapters with vibes.

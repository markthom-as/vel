---
id: INTG-005
title: Messaging multi-vendor ingestion
status: proposed
priority: P1
estimate: 5-8 days
dependencies:
  - INTG-001
  - INTG-002
  - INTG-004
---

# Goal

Expand messaging from one generic snapshot path to a provider-aware model that supports iMessage, Signal, and WhatsApp.

# Scope

- Define canonical messaging provider payloads and normalization contracts.
- Introduce provider-specific adapters or bridges for:
  - iMessage
  - Signal
  - WhatsApp
- Preserve canonical `message_thread` semantics for downstream context and nudges.
- Attach participant identity candidates for person resolution.

# Deliverables

- provider-normalized thread/message ingest contracts
- adapter scaffolding for three providers
- messaging provenance fixtures
- tests for same-person-across-providers scenarios

# Acceptance criteria

- Vel can ingest multiple messaging providers without collapsing them into one untraceable source.
- Thread provenance identifies both provider and connection.
- Participants have a structured route into person identity resolution.

# Notes

The point is not “support chat apps.” The point is interpersonal state with provenance.

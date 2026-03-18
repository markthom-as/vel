---
created: 2026-03-18T07:25:40.260Z
title: Ticket 014 - define protocol versioning strategy
area: docs
files:
  - docs/tickets/phase-4/014-swarm-execution-sdk.md
---

## Problem

Ticket 014 (Swarm Execution SDK) creates `vel-protocol` — the first external-facing contract in the system. Unlike internal crates, external SDK consumers will pin protocol versions. The ticket says "versioned protocol envelopes" but doesn't specify the versioning granularity or negotiation semantics.

Without a versioning strategy defined before SP1 contract work, the protocol crate will be designed for internal convenience rather than external stability, and breaking it later will be painful for SDK consumers.

## Solution

Add a versioning strategy decision to the ticket before Phase 4 SP1:

**Recommended: envelope-level version field with graceful unknown-version handling**
```json
{ "vel_protocol": "1.0", "type": "launch_request", ... }
```
- Simple, future-compatible, no runtime negotiation needed for v1
- Clients reject unknown major versions; servers accept unknown minor versions
- Breaking changes require major bump; additive changes are minor

This should be locked in Phase 4 SP1 contract work before the protocol crate is implemented.

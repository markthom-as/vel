---
created: 2026-03-18T07:25:40.260Z
title: Ticket 006 - add Current Baseline section
area: docs
files:
  - docs/tickets/phase-2/006-connect-launch-protocol.md
---

## Problem

Ticket 006 (Connect Launch Protocol) is marked "in-progress" but has no "Current Baseline" section documenting what has already shipped. An audit found that all 4 acceptance criteria are unmet — routes still return 403 via `deny_undefined_route()`. Without a clear baseline, future agents planning this ticket will re-investigate what exists instead of starting from ground truth.

Currently shipped: worker heartbeat TTL infrastructure in `client_sync.rs`, `ConnectInstanceData` DTOs in `vel-api-types`, CLI commands in `commands/connect.rs` (which 403 against a live daemon), and tests in `app.rs` that *assert* 403 responses. Zero launch/lease/terminate logic exists.

## Solution

Add a "Current Baseline (Already Present)" section to the ticket (same pattern as tickets 012 and 019) listing exactly what's shipped vs. what remains. Also clarify that the 403-asserting tests in `app.rs` will need to be inverted to positive-path tests as part of the implementation.

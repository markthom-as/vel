---
created: 2026-03-18T07:25:40.260Z
title: Ticket 016 - add broker scope decision record
area: docs
files:
  - docs/tickets/phase-2/016-capability-broker-secret-mediation.md
---

## Problem

Ticket 016 (Capability Broker) is ambiguous about whether existing integrations (Todoist, Google Calendar) should route through the broker or if the broker is strictly for agent-delegated external actions. This ambiguity will cause scope creep during implementation — implementors may assume integrations are in scope and over-engineer the broker API surface.

Decision made (2026-03-18 audit): **Agents now, integrations later.** Existing integrations retain their current direct credential path for this milestone. The capability broker mediates agent-delegated external actions only. Integration migration is explicitly deferred.

## Solution

Add a "Key Decisions" or "Scope Boundaries" section to the ticket with this decision record so it's locked before planning begins.

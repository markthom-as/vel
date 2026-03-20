# Canonical Scheduler Facets

## Purpose

Vel preserves the proven local rule system from `codex-workspace`, but raw provider labels and title syntax are not durable product truth.

Canonical scheduler facets are the Vel-owned semantics behind:

- same-day `reflow`
- assistant grounding
- recall and filtering
- future commitment-aware planning

## Current canonical facet set

Vel currently normalizes these scheduler semantics:

- `block_target`
- `duration_minutes`
- `calendar_free`
- `fixed_start`
- `time_window`
- `local_urgency`
- `local_defer`

These map to operator-visible reflow facets:

- `block:*`
- `30m`, `1h`
- `cal:free`
- `fixed_start`
- `time:prenoon`, `time:afternoon`, `time:evening`, `time:night`, `time:day`
- `urgent`
- `defer`

## Normalization rules

The current normalization seam is defined in `vel-core` so backend behavior can share it without shell-local parsing.

Current rules:

- `block:<name>` maps to `block_target = <name>`
- `cal:free` and `@cal:free` map to `calendar_free = true`
- duration tokens like `30m`, `@30m`, `1h`, `@1h` map to `duration_minutes`
- `time:*` labels map to a canonical `time_window`
- `urgent` and `@urgent` map to `local_urgency = true`
- `defer` and `@defer` map to `local_defer = true`
- explicit fixed labels or non-midnight due datetimes map to `fixed_start = true`

## Persistence rule

For commitments, canonical scheduler semantics are now persisted in commitment metadata under `scheduler_rules`.

That means:

- insert and update paths normalize scheduler semantics at the storage edge
- backend/domain consumers can load typed scheduler rules from the persisted commitment record
- raw labels remain available as compatibility/search inputs, but they are no longer the only durable source of scheduling truth

Assistant-facing bounded context now uses those persisted scheduler rules too.

That means:

- assistant context can surface open commitments with typed scheduler facets
- recall and grounding paths can explain commitment constraints without re-parsing raw labels on every call
- client shells can consume scheduler semantics through transport DTOs instead of inventing their own rule parsing

## Boundary rule

Raw upstream/provider labels remain compatibility metadata.

Vel should:

- ingest and preserve them when needed for compatibility
- normalize them into canonical scheduler semantics for backend behavior
- avoid making raw provider syntax the internal product model

## Current limit

This contract is intentionally same-day and bounded.

It does not claim:

- broad multi-day optimization
- universal planner coverage for every provider syntax
- shell-owned scheduling semantics
- autonomous upstream mutation beyond existing supervised lanes

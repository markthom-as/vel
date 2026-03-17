---
title: "Normalize API types, time fields, and client contracts across backend and web"
status: in_progress
owner: agent
type: implementation
priority: high
created: 2026-03-15
depends_on: []
labels:
  - vel
  - api
  - frontend
  - type-safety
---
The repo has a useful API surface now, but the contract between Rust and the web client still looks more hand-shaken than mechanically enforced.

That works right up until it doesn't.

## Concrete concerns

- Web types use raw `number` timestamps in multiple places, while Rust uses `OffsetDateTime` and serialized JSON. That is manageable, but only if the wire contract is explicit and stable.
- The frontend has several `unknown`, broad casts, and message-kind-specific assumptions that are drifting toward "runtime schema by vibes."
- Some UI card payloads appear hand-modeled in the client instead of generated or validated against backend DTOs.
- API helpers are minimal and do not centralize response/error/meta handling enough.

## Goal

Make the API contract explicit, typed, and boring.

## Tasks

- Define and document the wire format for all timestamps:
  - Unix seconds? milliseconds? ISO 8601? pick one and stick to it.
- Audit all existing API DTOs for consistency.
- Replace frontend `unknown` and ad hoc casts with explicit discriminated unions for message kinds where feasible.
- Add runtime validation or a generated client layer for the web app.
- Normalize API response handling in the client so meta/error/warnings are not silently discarded.
- If feasible, generate TypeScript types from Rust/OpenAPI/schema instead of hand-maintaining them.

## Recommended shape

For chat/message rendering, prefer something like:

- `message.kind` as discriminant
- typed payload per kind
- one renderer registry keyed by discriminant
- compile-time exhaustiveness check

## Acceptance Criteria

- Time fields are documented and consistent.
- Web client no longer relies on broad `unknown as { ... }` casts for core message kinds.
- API response envelopes are handled consistently.
- Adding a new message kind requires updating one typed registry, not sprinkling casts around the UI.

## Notes for Agent

Cross-language contract drift is how repos become spiritually TypeScript but operationally YAML.

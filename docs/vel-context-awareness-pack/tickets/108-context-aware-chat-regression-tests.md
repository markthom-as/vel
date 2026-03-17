---
id: 108
title: context-aware-chat-regression-tests
status: proposed
owner: vel
priority: high
updated: 2026-03-16
---

# context-aware-chat-regression-tests

## Summary

Add end-to-end and service-level tests proving that context-aware chat actually grounds replies.

## Why this exists

A feature like this will regress quietly. The most dangerous failure mode is not a crash; it is a plausible generic answer that ignores live context.

## Scope

- Add server tests for packet selection, persistence, and inspection.
- Add frontend tests for mode controls and packet inspection.
- Add fixture-driven tests for status questions.

## Deliverables

- Context-aware chat test fixtures.
- Service tests for selector heuristics.
- App tests hitting chat send + packet inspection route.
- UI tests for composer and inspection drawer.

## Implementation notes

- Seed current context, commitments, and stale integration states in test storage.
- Assert that asking “what is current status” causes packet creation and grounded assistant behavior.
- Assert that `none` mode omits packet attachments.
- Assert that stale warnings surface.
- Keep tests deterministic by stubbing the LLM provider or inspecting built prompt/metadata where appropriate.

## Acceptance criteria

- Tests fail if packet creation disappears.
- Tests fail if status replies stop including current-context facts under seeded fixtures.
- Tests fail if `manual` mode does not honor selected refs.
- CI runtime remains reasonable.

## Files likely touched

- `crates/veld/src/app.rs`
- service test modules
- `clients/web/src/components/MessageComposer.test.tsx`
- `clients/web/src/components/ThreadView.test.tsx`

## Risks / gotchas

- Don’t test only happy-path serialization. Test the behavioral contract, or the whole thing becomes decorative QA tax.

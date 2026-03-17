---
id: NOW-012
status: proposed
title: Update status docs and UI contracts for the repaired Now experience
owner: docs
priority: P2
---

## Goal

Make repository docs reflect the actual Now-page contract and behavior.

## Why

If docs lag the implementation, the repo teaches future contributors the wrong thing.

## Files likely touched

- `docs/status.md`
- any UI/API docs for context/now surfaces
- optionally add a short `docs/ui/now-page.md`

## Requirements

1. Document the new `/v1/now` endpoint.
2. Document freshness semantics.
3. Document timezone behavior.
4. Document that sync routes affecting awareness trigger evaluate.
5. Document debug-vs-operator labeling policy.

## Acceptance criteria

- A new contributor can understand how the Now page gets its state without reverse-engineering four endpoints and a prayer.

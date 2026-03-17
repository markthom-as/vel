---
id: 109
title: docs-status-and-dogfooding-updates
status: proposed
owner: vel
priority: high
updated: 2026-03-16
---

# docs-status-and-dogfooding-updates

## Summary

Update canonical status/docs and add a dogfooding script/checklist for attached-context chat.

## Why this exists

This repo already has a docs/status drift problem history. Shipping the feature without updating the canonical ledger would be reenacting the same symptom in better clothes.

## Scope

- Update canonical status docs only after implementation lands.
- Document API/UX behavior.
- Add a simple dogfooding checklist for verifying attached-context awareness locally.

## Deliverables

- `docs/status.md` update.
- `docs/chat-interface-status-and-outstanding.md` update.
- brief usage doc or checklist for developers.
- example curl / CLI / browser flow.

## Implementation notes

- Describe the feature exactly as shipped, not as dreamed.
- Include where packet inspection lives and what guarantees it offers.
- Add a local verification recipe using seeded context and one or two canonical prompts.

## Acceptance criteria

- Canonical docs match implementation.
- A developer can follow the checklist and verify the feature locally.
- No stale “planned” language remains once shipped.

## Files likely touched

- `docs/status.md`
- `docs/chat-interface-status-and-outstanding.md`
- possibly `docs/api/chat.md`
- new dogfooding note under `docs/reviews/` or `docs/specs/`

## Risks / gotchas

- Don’t mark it done until the behavior is actually end-to-end. Vel should not become self-mythologizing.

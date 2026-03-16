---
id: VSM-012
title: Prompt and Context Registry Refactor
status: proposed
priority: P1
owner: platform
labels: [prompts, context, registry, refactor]
---

## Summary
Move mutable prompt and context assembly logic into registry-driven assets with schema validation.

## Why
If prompt behavior is hardcoded deep in business logic, self-modification will keep reaching for a chainsaw.

## Scope
- Extract prompts/context templates into dedicated assets.
- Add schemas and test fixtures.
- Ensure runtime assembly is declarative and diff-friendly.

## Implementation tasks
1. Inventory current prompt/context surfaces.
2. Define registry schema for assets and policies.
3. Refactor runtime to load from registry.
4. Add schema validation and prompt eval tests.
5. Document conventions for mutable prompt assets.

## Acceptance criteria
- Prompt assets are separated from core logic.
- Changes are reviewable as asset diffs.
- Schema and eval checks run on modification.
- Existing behavior preserved or explicitly migrated.

## Dependencies
- VSM-001, VSM-005.


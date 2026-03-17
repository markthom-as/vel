---
title: Frontend surface decomposition plan
status: done
owner: agent
type: architecture
priority: medium
created: 2026-03-17
depends_on:
  - 004-architecture-map-and-module-boundary-audit.md
labels:
  - vel
  - web
  - frontend
---

Plan decomposition of oversized web surfaces only after the architecture map exists.

## Scope

- `SettingsPage.tsx`
- `types.ts`
- typed domain boundaries versus transport decoding
- candidate feature slices and shared UI/state modules

## Current audit anchors

Primary files:

- [clients/web/src/components/SettingsPage.tsx](/home/jove/code/vel/clients/web/src/components/SettingsPage.tsx)
- [clients/web/src/types.ts](/home/jove/code/vel/clients/web/src/types.ts)
- [clients/web/src/data/resources.ts](/home/jove/code/vel/clients/web/src/data/resources.ts)

Current boundary judgment:

- `SettingsPage.tsx` is mixing page shell, query subscriptions, mutation orchestration, per-tab state, and tab-specific rendering
- `types.ts` is actually both the DTO authority and the decoder runtime
- `resources.ts` is tightly coupled to that transport monolith, so decoder extraction and resource realignment need to move together

## Stable extraction seams

- `settings-shell`
  - tab selection, top-level layout, minimal shared query wiring
- `settings-integrations`
  - provider actions, local-source guidance, logs, and integrations section rendering
- `settings-runs`
  - retry/block drafts and run-control rendering
- `settings-loops`
  - loop config draft/update behavior and loop-control rendering
- `settings-panels`
  - reusable settings-specific display panels and badges
- `transport-decoder-core`
  - `decodeApiResponse`, collection/null helpers, primitive validators, JSON coercion
- `transport-by-domain`
  - chat, now/context, settings/integrations, runtime control, suggestions/provenance, ws events

## Sequencing

1. define the target boundary map:
   page shell, settings tabs, settings action hooks, decoder core, domain decoders, resources
2. extract decoder utilities and domain decoder modules from [types.ts](/home/jove/code/vel/clients/web/src/types.ts)
3. align [resources.ts](/home/jove/code/vel/clients/web/src/data/resources.ts) to those decoder modules and remove duplicate inline decoding
4. extract `settings-integrations`
5. extract `settings-runs`
6. extract `settings-loops`
7. extract shared settings panels/components

## Guardrails

- do not split `SettingsPage` by visual chunks alone
- do not let extracted settings sections bypass the resource/query layer with direct transport calls
- do not split `types.ts` into page-local files; split by transport domain
- do not move decoder helpers without moving resource consumers in the same slice
- do not duplicate integration defaults or fallback logic across extracted modules

## Acceptance criteria

- `SettingsPage.tsx` no longer owns every major settings tab body plus all tab-specific action state
- `types.ts` no longer acts as one global decoder choke point for unrelated surfaces
- transport decoding and resource ownership are aligned by domain
- the first decomposition slices reduce real ownership overlap instead of only moving JSX around

## Completed planning outputs

- stable extraction seams are defined for settings shell, integrations, runs, loops, shared panels, decoder core, and domain decoders
- sequencing and guardrails are explicit enough for later implementation slices without reopening the architecture audit

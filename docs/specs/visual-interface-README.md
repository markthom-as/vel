# Vel Visual Interface Repo-Ready Starter

This package set is a repo-ready starter for Vel's cross-platform visual embodiment system.

Core line:
- one shared affect engine
- one shared morphology layer
- one sync protocol
- multiple embodiments
- state sync, not frame sync

## Recommended order

1. Read `docs/visual-interface/01-architecture-overview.md`
2. Read `docs/visual-interface/02-affect-schema.md`
3. Implement tickets in numerical order under `/tickets`
4. Use package scaffolds as the canonical module boundaries

## Package layout

- `packages/vel-affect-core` — canonical affect schema, reducer, smoothing, presets
- `packages/vel-visual-morphology` — state-to-body mapping and design tokens
- `packages/vel-protocol` — phone/watch sync packet and serializers
- `packages/vel-render-web` — desktop/mobile renderer reference implementation
- `packages/vel-render-watch` — pre-authored basis states and interpolation logic

## Ground rules

- Do not add eyes, pupils, sclera, or gaze tracking.
- Preserve the feeling of a face emerging from a field, not a mascot.
- Watch is a translation of Vel, not a tiny port of desktop.
- Keep the affect model renderer-agnostic.

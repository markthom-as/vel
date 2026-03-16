# Package Boundaries

## `packages/vel-affect-core`
Owns:
- canonical types
- reducer
- event inputs
- smoothing helpers
- presets
- baseline state

Must not own:
- rendering concerns
- shader logic
- platform details

## `packages/vel-visual-morphology`
Owns:
- state-to-morph mapping
- visual tokens
- presets for topology/material response

Must not own:
- network sync
- rendering surfaces

## `packages/vel-protocol`
Owns:
- packet types
- serialization
- deserialization
- protocol validation

Must not own:
- affect transition logic
- renderer-specific interpolation logic

## `packages/vel-render-web`
Owns:
- web embodiment
- shader/material implementation
- debug app
- adaptive quality behavior

## `packages/vel-render-watch`
Owns:
- watch basis states
- blending rules
- local fallback behavior
- watch-specific interpolation

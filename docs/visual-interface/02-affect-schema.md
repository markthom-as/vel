# Canonical Affect Schema

The affect schema is the source of truth for all renderers.

## Type definition

```ts
export type VelMode =
  | "idle"
  | "listening"
  | "thinking"
  | "speaking"
  | "reflecting"
  | "warning"
  | "overloaded"
  | "sleeping";

export type VelAffectState = {
  timestamp: number;

  valence: number;         // -1..1
  arousal: number;         // 0..1
  coherence: number;       // 0..1
  confidence: number;      // 0..1
  sociality: number;       // 0..1
  turbulence: number;      // 0..1

  attention: number;       // 0..1
  speaking: number;        // 0..1
  listening: number;       // 0..1
  latencyPressure: number; // 0..1
  uncertainty: number;     // 0..1

  faciality: number;       // 0..1
  asymmetry: number;       // 0..1
  fragmentation: number;   // 0..1

  mode: VelMode;

  hueBias?: number;        // 0..1
  glow?: number;           // 0..1
  heartbeat?: number;      // 0..1
};
```

## Semantic rules

- `faciality` controls how clearly face-like organization emerges from the field.
- `fragmentation` controls how much the body splits into multiple lobes/subselves.
- `sociality` controls singular-vs-choral behavior and multi-blob composition.
- `uncertainty` and `coherence` must not be treated as opposites by default. Some states are uncertain yet coherent.
- `attention` must elevate felt presence without creating gaze.

## Ranges

All normalized values must be clamped to the documented range.

## Hard prohibition

Do not add categorical emotions like `happy`, `sad`, or `angry` to the canonical state.
Those may exist as convenience presets, but not as the core model.

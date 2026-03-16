# Morphology Layer

The morphology layer converts affect state into render-usable body parameters.

## Reference type

```ts
export type VelMorphology = {
  headScale: number;
  verticalStretch: number;
  horizontalCompression: number;
  tiltX: number;
  tiltY: number;

  primaryMassRadius: number;
  satelliteCount: number;
  satelliteScale: number;
  mergeThreshold: number;

  browHeight: number;
  browAngle: number;
  browCompression: number;

  noseBridgeStrength: number;
  mouthWidth: number;
  mouthCurve: number;
  mouthOpen: number;
  jawWeight: number;
  cheekTension: number;

  surfaceNoiseAmp: number;
  surfaceNoiseFreq: number;
  edgeSoftness: number;
  internalGlow: number;
  colorHue: number;
  colorSaturation: number;
  colorValue: number;

  faceVisibility: number;
};
```

## Mapping guidance

- High `coherence`:
  - fewer satellites
  - stronger merge threshold
  - smoother silhouette
  - clearer facial topology

- High `fragmentation`:
  - more satellites
  - weaker merge threshold
  - more asymmetry
  - interrupted face emergence

- High `listening`:
  - slight forward tilt
  - mouth stillness
  - greater faciality
  - lower turbulence than thinking

- High `speaking`:
  - mouth seam activation
  - rhythmic lower-face motion
  - warmer color bias
  - partial reintegration of satellites

- High `warning` / `latencyPressure`:
  - tighter mouth line
  - stronger central compression
  - sharper brow angle
  - warmer or more urgent color shift

## Anti-patterns

Avoid:
- realistic lips
- visible eyeballs
- detailed sockets
- skin shading
- rigid left/right symmetry

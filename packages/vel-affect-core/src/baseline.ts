import type { VelAffectState } from "./types";

export const baselineState: VelAffectState = {
  timestamp: 0,
  valence: 0.05,
  arousal: 0.18,
  coherence: 0.72,
  confidence: 0.62,
  sociality: 0.18,
  turbulence: 0.12,
  attention: 0.22,
  speaking: 0,
  listening: 0,
  latencyPressure: 0.05,
  uncertainty: 0.12,
  faciality: 0.42,
  asymmetry: 0.18,
  fragmentation: 0.08,
  mode: "idle",
  hueBias: 0.52,
  glow: 0.16,
  heartbeat: 0.12,
};

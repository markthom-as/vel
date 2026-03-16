import type { VelAffectState } from "./types";
import { baselineState } from "./baseline";

export const presets: Record<string, VelAffectState> = {
  idle: baselineState,
  listening: {
    ...baselineState,
    mode: "listening",
    attention: 0.92,
    listening: 1,
    speaking: 0,
    turbulence: 0.18,
    faciality: 0.68,
  },
  thinking: {
    ...baselineState,
    mode: "thinking",
    arousal: 0.45,
    turbulence: 0.58,
    fragmentation: 0.34,
    faciality: 0.56,
    uncertainty: 0.42,
  },
};

import type { VelAffectState, VelEvent } from "./types";

const clamp01 = (n: number) => Math.max(0, Math.min(1, n));
const clamp11 = (n: number) => Math.max(-1, Math.min(1, n));

export function reduceAffect(state: VelAffectState, event: VelEvent): VelAffectState {
  switch (event.type) {
    case "SET_MODE":
      return { ...state, mode: event.mode, timestamp: event.now };
    case "USER_SPEECH_START":
      return {
        ...state,
        timestamp: event.now,
        mode: "listening",
        attention: 1,
        listening: 1,
        speaking: 0,
        turbulence: clamp01(state.turbulence + 0.08),
        faciality: clamp01(state.faciality + 0.18),
      };
    case "AGENT_THINKING_START":
      return {
        ...state,
        timestamp: event.now,
        mode: "thinking",
        arousal: clamp01(state.arousal + 0.2),
        turbulence: clamp01(state.turbulence + 0.28),
        fragmentation: clamp01(state.fragmentation + 0.22),
        uncertainty: clamp01(state.uncertainty + 0.2),
      };
    case "AGENT_SPEAKING_START":
      return {
        ...state,
        timestamp: event.now,
        mode: "speaking",
        speaking: 1,
        listening: 0,
        coherence: clamp01(state.coherence + 0.12),
        fragmentation: clamp01(state.fragmentation - 0.12),
        valence: clamp11(state.valence + 0.08),
      };
    case "WARN":
      return {
        ...state,
        timestamp: event.now,
        mode: "warning",
        arousal: clamp01(state.arousal + (event.intensity ?? 0.35)),
        latencyPressure: clamp01(state.latencyPressure + (event.intensity ?? 0.35)),
        valence: clamp11(state.valence - 0.18),
      };
    case "OVERLOAD":
      return {
        ...state,
        timestamp: event.now,
        mode: "overloaded",
        turbulence: clamp01(state.turbulence + (event.intensity ?? 0.4)),
        fragmentation: clamp01(state.fragmentation + (event.intensity ?? 0.4)),
        coherence: clamp01(state.coherence - 0.24),
      };
    case "TICK":
      return { ...state, timestamp: event.now };
    default:
      return state;
  }
}

import type { VelAffectState } from "../../vel-affect-core/src/types";
import type { VelMorphology } from "./types";

export function mapStateToMorphology(state: VelAffectState): VelMorphology {
  return {
    headScale: 1 + state.arousal * 0.06,
    verticalStretch: 1 + state.faciality * 0.08,
    horizontalCompression: 1 - state.coherence * 0.06,
    tiltX: (state.attention - 0.5) * 0.08,
    tiltY: (state.asymmetry - 0.5) * 0.12,

    primaryMassRadius: 0.8 + state.coherence * 0.16,
    satelliteCount: Math.round(1 + state.fragmentation * 4 + state.sociality * 2),
    satelliteScale: 0.12 + state.fragmentation * 0.12,
    mergeThreshold: 0.55 + state.coherence * 0.25 - state.fragmentation * 0.2,

    browHeight: 0.56 + state.attention * 0.06,
    browAngle: -0.1 + state.latencyPressure * 0.28,
    browCompression: 0.12 + state.uncertainty * 0.22 + state.latencyPressure * 0.18,

    noseBridgeStrength: 0.2 + state.faciality * 0.4,
    mouthWidth: 0.24 + state.coherence * 0.08,
    mouthCurve: state.valence * 0.08,
    mouthOpen: state.speaking * 0.22,
    jawWeight: 0.16 + state.confidence * 0.16,
    cheekTension: 0.12 + state.attention * 0.18,

    surfaceNoiseAmp: 0.03 + state.turbulence * 0.14,
    surfaceNoiseFreq: 0.5 + state.turbulence * 1.8,
    edgeSoftness: 0.88 - state.latencyPressure * 0.22 - state.fragmentation * 0.14,
    internalGlow: 0.12 + (state.glow ?? 0.1) * 0.42,
    colorHue: 220 + (state.hueBias ?? 0.5) * 40 - state.latencyPressure * 12,
    colorSaturation: 0.18 + state.arousal * 0.3,
    colorValue: 0.22 + state.coherence * 0.22 + (state.glow ?? 0.1) * 0.12,

    faceVisibility: state.faciality,
  };
}

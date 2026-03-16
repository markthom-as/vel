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
  valence: number;
  arousal: number;
  coherence: number;
  confidence: number;
  sociality: number;
  turbulence: number;
  attention: number;
  speaking: number;
  listening: number;
  latencyPressure: number;
  uncertainty: number;
  faciality: number;
  asymmetry: number;
  fragmentation: number;
  mode: VelMode;
  hueBias?: number;
  glow?: number;
  heartbeat?: number;
};

export type VelEvent =
  | { type: "SET_MODE"; mode: VelMode; now: number }
  | { type: "USER_SPEECH_START"; now: number }
  | { type: "USER_SPEECH_STOP"; now: number }
  | { type: "AGENT_THINKING_START"; now: number }
  | { type: "AGENT_THINKING_STOP"; now: number }
  | { type: "AGENT_SPEAKING_START"; now: number }
  | { type: "AGENT_SPEAKING_STOP"; now: number }
  | { type: "WARN"; now: number; intensity?: number }
  | { type: "OVERLOAD"; now: number; intensity?: number }
  | { type: "TICK"; now: number; dt: number };

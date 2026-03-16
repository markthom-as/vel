# Sync Protocol

The sync protocol is used primarily between phone and watch.

## Principle

Send **state**, not frames.

## Reference packet

```ts
export type VelSyncPacket = {
  version: 1;
  timestamp: number;
  mode: "idle" | "listening" | "thinking" | "speaking" | "reflecting" | "warning" | "overloaded" | "sleeping";

  valence: number;
  arousal: number;
  coherence: number;
  confidence: number;
  sociality: number;
  turbulence: number;
  attention: number;
  speaking: number;
  listening: number;
  uncertainty: number;
  faciality: number;
  asymmetry: number;
  fragmentation: number;

  accentHue?: number;
  pulseRate?: number;
  eventCue?: "none" | "resolve" | "warn" | "summon" | "reflect";
};
```

## Delivery guidance

- Active paired sessions: 2–4 Hz packet updates are sufficient.
- Always send immediately on:
  - mode change
  - reminder escalation
  - speech start/stop
  - task completion
  - warning/high-priority event

## Fallback guidance

When watch connectivity drops:
- continue local animation from the last packet
- decay toward watch-local idle baseline
- preserve breathing/pulse behavior
- do not hard-freeze unless necessary

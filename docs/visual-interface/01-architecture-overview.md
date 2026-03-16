# Architecture Overview

Vel should feel human-ish without producing the sensation of being watched.

The right abstraction is:
- not a character
- not a mascot
- not a fixed face
- a field of personality that intermittently condenses into faciality

## Core architecture

Separate three layers:

1. **Affect core**
   - canonical state object
   - transition logic
   - event reducer
   - smoothing and decay
   - presets

2. **Morphology**
   - transforms affect state into facial/body parameters
   - head mass
   - satellites
   - brow pressure
   - mouth seam
   - nose bridge
   - jaw and cheek tension
   - color and turbulence

3. **Embodiment renderers**
   - desktop/mobile web renderer
   - watch renderer
   - both consume the same canonical state

## Product constraint

Do not stream rendered frames from phone to watch as the default architecture.

Preferred architecture:
- phone/desktop may render rich realtime visuals
- watch receives compact state packets
- watch renders a lightweight local embodiment using pre-authored basis states and interpolation

## Design constraint

Vel must remain:
- affective
- expressive
- non-surveillant
- partially anthropomorphic
- materially synthetic
- dynamic across color, turbulence, and single-vs-many blob scales

## Deliverables

A correct implementation includes:
- shared TypeScript packages for affect, morphology, and protocol
- a web reference renderer
- a watch basis-state system
- a debug harness with sliders and event simulation
- performance guards
- uncanny-valley QA rules

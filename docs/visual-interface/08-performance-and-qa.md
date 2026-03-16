# Performance and QA

## Performance budgets

Desktop:
- rich rendering allowed, but avoid pathological overdraw

Mobile:
- must degrade quality gracefully
- target stable interaction over maximum effects

Watch:
- use pre-authored basis states plus lightweight interpolation
- avoid continuous heavy rendering paths

## QA gates

Reject any change that introduces:
- obvious gaze
- visible pupils or sclera
- uncanny realistic mouth shapes
- dead symmetry
- emoji-like categorical expression logic
- watch dependence on continuous phone frame streaming

## Acceptance checks

Vel should read as:
- alive
- human-ish
- expressive
- not watching the user
- a face emerging from personality, not a cartoon face pasted on a blob

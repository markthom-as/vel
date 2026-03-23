# Phase 71 Context

## Goal

Finish the line with deletion, proof, and Apple handoff.

## Boundary

This phase closes `v0.5.1`. It must not widen into Apple implementation or new feature lanes.

## Required Outputs

- explicit cleanup result for deprecated routes/shims
- web proof set
- drift/no-fallback proof
- Apple handoff/spec packet
- browser-executed evidence plus short human-readable notes for each major proof flow
- lightweight checked-in browser harness and deterministic proof commands
- controlled degraded-state fixture and browser-visible no-silent-fallback proof
- documentation of intentional bounded read surfaces: `/v1/agent/inspect` and `/api/integrations/connections`

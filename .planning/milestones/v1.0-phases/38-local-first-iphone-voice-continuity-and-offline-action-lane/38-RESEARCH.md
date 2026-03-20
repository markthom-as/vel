# 38 Research

## Goal

Prove the first local-first iPhone loop on top of the new embedded-capable Apple seam: voice capture with queued continuity, cached `Now`, offline-safe quick actions, and clean merge back into canonical thread/`Now` state.

## Inputs

- operator acceptance and priority decisions captured in [38-CONTEXT.md](/home/jove/code/vel/.planning/phases/38-local-first-iphone-voice-continuity-and-offline-action-lane/38-CONTEXT.md)
- the Phase 37 embedded runtime contract and bridge seam
- current Apple quick-loop implementation in `clients/apple/Apps/VeliOS`
- current offline queue/cache behavior in `VelAPI`

## Key Findings

- the proving flow is not “general offline Apple”; it is one specific fast loop: speak, get acknowledgment, survive disconnect, merge back cleanly
- the embedded-capable seam is now present, so Phase 38 should use it for bounded local helpers and queue preparation rather than broad policy migration
- the existing Apple queue and cached `Now` behavior already provide part of the substrate, but the continuity story still needs to feel deliberate rather than incidental
- thread continuity must remain one model across offline and online use; separate offline voice history would be a product fork

## Risks

- if offline voice invents separate thread semantics, the later merge will feel confusing and untrustworthy
- if the iPhone shell tries to answer backend-owned reasoning questions locally, Phase 38 will blur into failed parity claims
- if quick actions, cached `Now`, and voice continuity do not share one recovery story, the flow will still feel stitched together

## Recommended Shape

1. publish a contract for local-first voice continuity over embedded and daemon-backed seams
2. implement local voice capture, queue packaging, and offline-safe quick actions
3. align `Now`, drafts, and thread merge behavior around one continuity model
4. verify the “magical” loop and document remaining daemon-backed limits honestly

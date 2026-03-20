# Phase 38 Context

## Title

Local-first iPhone voice continuity and offline action lane

## Why this phase exists

The operator identified one proving flow for Apple embedded-core work with no hesitation:

voice capture with smooth queued continuity.

This is the highest-frequency, highest-leverage local-first Apple interaction:

- it proves offline capability
- it proves sync reliability
- it proves continuity quality
- it differentiates Vel immediately

## Product problem

Apple can already participate in backend-owned voice and thread continuity, but not yet through a truly local-first loop that feels immediate and robust when the daemon is unavailable.

## Phase goal

Make iPhone voice capture, cached `Now`, queued quick actions, and local thread draft continuation feel dependable and low-friction through the new embedded-capable path.

## Must stay true

- minimum acceptable offline baseline:
  - cached `Now`
  - local voice capture → queued
  - local quick actions
  - local thread draft continuation
- heavy recall and heavy reasoning remain daemon-backed
- local/offline and online continuity remain one thread-backed model
- the phase should prove local-first value, not chase total local parity

## “Magical” acceptance definition

The target interaction is:

- tap and speak
- instant acknowledgment
- survives offline
- later appears correctly in thread and `Now`
- no duplicate, lost, or confusing sync state

## Likely touch points

- iPhone voice input surface
- Apple local queue/offline cache
- thread draft persistence and merge logic
- `Now` Apple summary surfaces
- embedded-capable Rust bridge from Phase 37
- Apple/user docs

## Expected next step

Phase 38 planning should break this into:

1. contract for local-first iPhone voice continuity
2. implementation of queued voice/offline-safe quick actions
3. clean local-to-daemon continuity merge behavior
4. verification of the “magical” offline voice flow

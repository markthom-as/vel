---
id: TKT-013
status: proposed
title: Document post-MVP Apple roadmap for HealthKit, Siri/App Intents, and Shortcuts
priority: P2
estimate: 1-2 days
depends_on: [TKT-006, TKT-008, TKT-010]
owner: agent
---

## Goal

Capture the next layer of platform leverage without stuffing it into MVP.

## Topics to document

- App Intents / Shortcuts for “mark meds taken,” “what is next,” “capture thought”
- Siri phrase entry points
- HealthKit read/write boundaries, if ever justified
- Focus filters
- Smart Stack relevance
- StandBy / lock screen strategy
- Whether any features should move into a shared Rust core later via FFI

## Deliverables

- `/docs/roadmap/apple-post-mvp.md`
- recommendation on what stays Swift-native vs what migrates into Vel core

## Acceptance criteria

- Roadmap clearly separates MVP from seductive side quests
- Team has explicit decision record for Apple-native vs core-owned logic

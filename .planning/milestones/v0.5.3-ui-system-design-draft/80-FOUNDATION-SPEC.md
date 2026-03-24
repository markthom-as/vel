---
phase: 80
slug: foundation-design-system-and-theming
status: draft
created: 2026-03-23
---

# Phase 80 — Foundation Spec

## Purpose

Define the shared visual language for the web client.

This document should answer:

- what the product should feel like visually
- how typography is structured
- how icons are used
- how spacing and density work
- how color semantics map to states, object types, and projects

## Brand and Theme Direction

The design direction should be intentional, serious, and operational.

It should avoid:

- generic dashboard polish
- marketing-page hero aesthetics
- decorative color noise
- accidental visual systems created one component at a time

This phase should decide:

- how the current orange/copper dark system is refined without losing distinctiveness
- whether light mode remains deferred beyond this milestone
- what the background, surface, and emphasis hierarchy is

Imported shell hierarchy that the visual system must support:

- top orientation band
- stable nudge zone
- bottom floating action bar

## Typography

Define:

- display family
- body family
- mono/meta family
- scale
- weights
- line-height system
- case rules
- truncation rules
- tabular numeral usage

Typography roles should include at minimum:

- display
- heading
- section title
- body
- label
- caption
- meta / provenance

Typography direction:

- technical-instrumental overall
- slight editorial layer
- not corporate and not playful

## Iconography

Define:

- primary icon family
- stroke or fill posture
- default sizes
- alignment rules
- semantic meanings
- when icon-only actions are allowed
- when text labels are mandatory

Icon rule:

- one canonical icon system in MVP
- custom overrides may be allowed later

## Spacing and Layout Scale

Define:

- base spacing unit
- gap scale
- padding scale
- row heights
- density modes
- max widths
- section rhythm
- mobile collapse thresholds

Density rule:

- slightly denser than the current web line
- clearer, not cramped
- one canonical density for MVP

## Color System

Color must be defined in three coordinated layers:

### 1. Semantic State Colors

Stable meanings for states such as:

- success
- warning
- error
- blocked
- active
- muted
- info
- offline
- syncing
- pending
- done

Distinct state identities required at minimum:

- warning
- degraded
- blocked
- active
- done
- syncing
- offline

### 2. Object-Type Colors

Stable visual identities for:

- task
- thread
- event
- capture
- client
- message
- artifact
- action
- relation
- run / agent
- person
- project
- note
- review
- nudge
- system

### 3. Project Colors

Projects should get stable assignment from a bounded palette with rules for:

- chip treatment
- border/accent use
- background tint use
- contrast floor
- collision handling

Project color rule:

- algorithmic default now
- user override later
- primary use in chips/tags
- secondary use as subtle accents only

## Color Precedence Rules

This phase should explicitly define:

- state color overrides project color when urgency or trust must dominate
- project color expresses identity, not severity
- object color provides recognition without overpowering state
- rows should avoid multiple competing high-intensity colors
- nudge severity should not collapse object identity into generic alert coloring
- thread rows do not inherit project color by default
- client/provider colors stay separate from project/object colors

## Shape, Borders, and Elevation

Define:

- radius scale
- border strategy
- divider usage
- shadow/elevation rules
- flat vs inset vs raised surfaces

## Motion

Define:

- allowed transitions
- duration ranges
- reduced-motion fallback
- functional versus decorative motion rules

Motion non-negotiable:

- reduced motion must be supported

## Working Decisions Already Imported

- the active task is the dominant object on `Now` and should receive the clearest visual hierarchy
- persistent entry actions should stay visible but visually subordinate to the active work object
- nudge rendering may use color, icon, and project metadata together when that improves explainability
- mono is reserved for timestamps, IDs, logs, and provenance rather than broad UI copy
- object colors should be selective instead of universal to avoid confetti

## Remaining Questions To Resolve In This Phase

- What specific fonts best fit the product posture?
- Which object types deserve distinct color families versus neutral treatment?
- What exact copper/orange palette and neutral ladder should become canonical?

## Done When

1. The client has one explicit visual system.
2. Typography, iconography, spacing, and theme tokens are globally defined.
3. State, object, and project color rules are stable enough to guide component design.
4. Surface discussions can reference named tokens and rules instead of ad hoc taste.

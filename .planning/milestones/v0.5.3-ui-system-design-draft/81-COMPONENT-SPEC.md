---
phase: 81
slug: component-system-contract
status: draft
created: 2026-03-23
---

# Phase 81 — Component Spec

## Purpose

Define the reusable building blocks of the web client and how they compose.

This document should answer:

- what primitives exist
- what each primitive is for
- which variants are allowed
- how components inherit theme and interaction law
- which composition patterns are canonical

## Component-System Principles

- Components exist to express product meaning, not just to bundle styles.
- Shared primitives should be composable enough for all three top-level surfaces.
- Surface-specific components may compose primitives, but should not fork the design language.
- New primitives should only be added when an existing one cannot represent the needed role.

## Primitive Families

### Layout Primitives

- page frame
- top orientation band
- nudge zone
- floating action bar
- page section
- section header
- split pane
- stacked lane
- drawer
- inset tray

Shell rules:

- same top band, nudge zone position, and action bar concept across surfaces
- surfaces may vary content density, not shell structure

### Object Presentation Primitives

- row
- item headline/meta band
- object card
- thread block
- metric strip
- state banner
- provenance block
- tag / chip / pill

Primitive posture:

- row-first system
- selective cards only where the content genuinely needs a richer container

### Action Primitives

- primary button
- secondary button
- icon button
- inline action link
- filter control
- segmented toggle
- quick-complete / acknowledge / open-thread affordance

Imported base action grammar should map consistently onto these primitives:

- confirm / reject
- complete / defer
- open / inspect
- run / dismiss
- discuss / feedback
- undo / review / retry

Critical-action rule:

- critical actions must not be hover-only
- secondary actions may reveal on hover if they are not required for comprehension or safety

### Input Primitives

- composer
- text input
- search field
- select / filter input
- confirmation dialog

### State Primitives

- empty state
- loading skeleton
- error state
- degraded/offline banner
- inline success/error feedback

### Thread Primitives

- block container
- compact block view
- expanded block view
- focused block view
- block action rail
- breadcrumb/orientation strip

## Composition Rules

This phase should define:

- what earns a card versus a row
- what earns a full section
- what can appear containerless
- how many emphasis zones a page may have at once
- how drawers interact with the base layout
- how list density changes across desktop and mobile

Imported thread-content rule:

- generic block rendering is acceptable in MVP if the schema is stable
- richer object-native renderers can come later without rewriting the interaction model

Locked card use:

- nudges
- run/action blocks
- media/artifact blocks
- config blocks

Locked `Now` rules:

- nudges live in a dedicated lane, not mixed inline with work rows
- project identity appears as metadata/tagging only
- completed rows disappear immediately except optional transient acknowledgement

System `Control` posture:

- dense but readable
- structured rows with expandable detail
- not playful card grids
- not raw tables by default

## Variant Rules

For each primitive, define:

- required props / content shape
- allowed variants
- prohibited variants
- responsive behavior
- accessibility constraints
- token dependencies

## Anti-Patterns To Explicitly Ban

- page-local color semantics
- page-local spacing systems
- duplicate button variants with overlapping meaning
- giant catch-all cards for unrelated object types
- hidden hover-only critical actions
- containers added solely for visual weight without semantic purpose

## Working Decisions Already Imported

- shell-level injection is brokered by core rather than freely module-defined
- thread content must support compact, expanded, and focused views
- bounded config blocks are valid in thread context
- object browsers should live under `System > Control` rather than becoming a new top-level surface
- metric strips should be reduced in prominence
- rows may carry actions, tags, status, and minimal metadata, but should not become overloaded catch-alls
- drawers are a sparing primitive, not the default answer to every detail view

## Remaining Questions To Resolve In This Phase

- Which existing `core/` primitives survive, merge, or split?
- What is the canonical row pattern across `Now`, `Threads`, and `System`?
- Which current components are too presentation-specific to remain “core”?

## Done When

1. The shared primitive set is explicit.
2. Each primitive has a stable role and limited variants.
3. Page-level specs can compose the system without inventing new semantics.
4. The follow-on implementation phase can refactor toward a clear target instead of a vague “cleanup.”

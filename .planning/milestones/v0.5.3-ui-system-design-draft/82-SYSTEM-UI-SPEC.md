# Phase 82 — `System` UI Spec

## Purpose

Define the page-level contract for `System` as the structural, trust, configuration, and repair surface.

## Surface Role

`System` answers:

- what is trustworthy
- what is connected or degraded
- what structural objects exist
- what can be repaired or configured

It does not act as:

- a fourth general work surface
- a dumping ground for unrelated documentation
- a disconnected set of mini-admin apps

## Subsections

- `Overview`
- `Operations`
- `Integrations`
- `Control`
- `Preferences`

## Posture

- `Overview` and `Integrations` are read-first
- `Operations` and `Control` are more operational
- `Preferences` is for personal presentation, accessibility, and interaction settings

## Page Structure

1. Top orientation band
2. Section navigation / browse rail
3. Main detail pane
4. Docked action bar

## Disclosure Map

### Inline

- row expansion
- toggles
- compact settings
- summary states

### Detail Pane / Split View

- integrations
- object browsers
- mappings
- log summaries
- control object detail

### Dedicated Detail Page

- very large log sets
- complex configuration structures
- workflow admin
- schema-adjacent editors

## Visual Hierarchy

- dense but readable
- structured rows with expandable detail
- not playful card grids
- not raw tables by default
- provider identity recognizable but subdued
- state color overrides provider identity

## Integrations Rules

- browse/detail split, not giant expanding spaghetti
- provider identity uses icon plus subtle tint
- no full-card provider brand coloring
- trust state outranks provider branding

## Banned Patterns

- fragmented disconnected subsystem views
- raw logs dumped inline by default
- dashboard-style metric walls
- provider-brand takeover
- card-grid “app launcher” posture

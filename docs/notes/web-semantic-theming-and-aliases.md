# Web semantic theming and alias system

## Why this is parked

The web client currently spreads semantic UI decisions across:
- theme token constants
- feature-local view models
- component-local Tailwind class strings
- icon selection inside view components
- ad hoc display-label derivation for projects, nudges, calendars, and status/mode surfaces

This creates drift and makes simple appearance changes require multi-file edits.

## Current repo shape

Current seams identified in the live tree:
- `clients/web/src/core/Theme/tokens.ts` holds layout and brand primitives, but not a semantic appearance registry
- `clients/web/src/views/now/nudgeViewModel.ts` now centralizes some nudge semantics, but only for nudges
- `clients/web/src/views/now/nowNudgePresentation.tsx` still carries shared icon and presentation logic
- `clients/web/src/core/FilterToggleTag/ProjectTag.tsx` uses hashed project colors rather than semantic project families
- `clients/web/src/core/FilterToggleTag/NudgeKindTag.tsx` still hardcodes urgent/default tag colors
- `clients/web/src/core/Icons/IconGlyphs.tsx` exposes raw glyphs, but there is no semantic icon registry
- `clients/web/src/types.ts` and current settings/config surfaces do not expose a durable theme or alias contract beyond limited web settings
- `config/templates/vel.toml.template` and `config/examples/app-config.example.toml` expose `now` presentation policy, but not general semantic UI configuration

## Goal

Move toward a centrally themable, configurable UI where:
- semantic concepts own colors and icons
- aliasable concepts own display labels separately from canonical backend values
- feature components consume semantic keys, not hardcoded classes
- a future durable config/settings contract can override aliases and appearance safely

## Initial semantic scope

Start with these concept families:
- nudges
- alerts and status warnings
- projects
- modes, phases, risk, drift, and severity labels
- calendars
- providers and integration identities
- thread or inbox state tags where those meanings recur

Each family should resolve:
- semantic key
- display label
- icon key
- appearance token bundle
- fallback behavior for unknown values

## Recommended implementation path

### 1. Central semantic appearance registry

Add a web-owned semantic registry that maps keys such as:
- `nudge.system`
- `nudge.warning`
- `project.default`
- `calendar.personal`
- `mode.focus`
- `alert.degraded`

Each entry should expose:
- text token
- border token
- background token
- accent/glow token
- icon key
- optional alias label

### 2. Semantic icon registry

Keep raw SVG exports as implementation detail, but add semantic icon lookup so feature components ask for:
- `semanticIcon("nudge.system")`
- `semanticIcon("calendar.personal")`
- `semanticIcon("mode.focus")`

### 3. Alias/view-model layer

Add central helpers that resolve:
- canonical value
- optional alias override
- semantic appearance key
- semantic icon key

Target helpers:
- nudge semantic model
- project display model
- calendar display model
- mode/status display model

### 4. Refactor highest-friction callsites first

Migrate these surfaces first:
- sidebar nudges
- Now nudge strip
- project tags
- nudge kind tags
- calendar filter chips and labels
- operator/system status chips

### 5. Durable config path

Keep v1 web-local, but shape it so it can later map onto:
- future `appearance_settings`
- future semantic alias config
- checked-in config template and schema when promoted to runtime config

Do not require backend persistence in the first implementation slice.

## Refactor opportunities still open

- extract `CalendarRail` from `NudgeZone`
- make `SurfaceState` variants explicit instead of inferring from message text
- centralize system deep-link labels and targets instead of embedding cross-surface label knowledge in feature logic
- split theme tokens into layout tokens, brand tokens, and semantic appearance tokens
- replace project-tag hashing with semantic project-family mapping plus fallback hashing only when no semantic family exists

## Acceptance criteria for the future slice

A later implementation should count as complete when:
- changing one semantic registry entry updates all matching web surfaces
- icons for shared concepts are chosen centrally
- labels for aliasable concepts can be overridden without changing canonical IDs
- feature components stop embedding semantic color/icon choices inline
- the semantic layer is structured to support eventual durable config/schema ownership

## Defaults and assumptions

- v1 should be web-local typed config and registry, not runtime-persistent config
- typography and layout theming are out of scope for the first slice
- focus first on colors, icons, and display aliases for semantic concepts
- future durable config should extend existing config and schema governance instead of inventing a parallel ad hoc file

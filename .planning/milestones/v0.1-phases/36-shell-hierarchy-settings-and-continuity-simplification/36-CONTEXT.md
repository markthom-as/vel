# Phase 36 Context

## Title

Shell hierarchy, settings, and continuity simplification

## Why this phase exists

After `Now` truth repair and current-day correction, the surrounding shell still needs cleanup. The operator has called out a broader “slop” problem:

- too much internal/sync language
- unclear iconography
- wrong affordances
- sidebar context that does not add value
- `Settings` structure that feels cluttered and misplaced

## Product problem

Even with a corrected `Now`, the product will still feel messy if:

- `Threads` surfaces too much by default
- `Settings` remains text-heavy and poorly grouped
- the sidebar stays too prominent or noisy
- links and buttons remain mismatched to real actions

## Phase goal

Make the shell hierarchy match the intended product model: `Now` primary, `Threads` contextual continuity, `Settings` advanced management, and sidebar context available but ignorable.

## Must stay true

- `Now` remains the default operator surface
- `Threads` is not a chat-first inbox
- only one resurfaced thread appears on `Now`, and only if confidence is high
- the web sidebar becomes a thin icon rail by default
- `Settings` is category-driven and summary-first rather than prose-heavy

## Explicit operator direction

- sidebar role:
  - thin icon rail
  - optional/collapsible
  - secondary state only
- `Settings` should emphasize:
  - display
  - integrations
  - policies
  - backup/restore
  - LLM configuration
  - documentation
- documentation/help should become globally available, not awkwardly buried
- sync controls and connect state should be less confusing and less manually repetitive

## Likely touch points

- `clients/web/src/components/MainPanel.tsx`
- `clients/web/src/components/ThreadView.tsx`
- `clients/web/src/components/SettingsPage.tsx`
- `clients/web/src/components/*`
- `docs/user/daily-use.md`
- `docs/user/setup.md`
- `docs/product/operator-mode-policy.md`

## Expected next step

Phase 36 planning should break this into:

1. hierarchy and affordance contract publication
2. `Settings` restructuring and clutter removal
3. thread/surface/sidebar simplification
4. `Vel.csv`-backed verification of shell cleanup

---
id: APPLE-010
title: Create Apple design system package aligned with Vel product tone
status: proposed
owner: agent
priority: p2
area: ui
depends_on: [APPLE-004]
---

# Goal

Create a minimal design system package so Apple surfaces look like parts of Vel rather than unrelated native prototypes wearing a fake mustache.

# Requirements

- `VelAppleUI` package
- semantic colors:
  - neutral
  - due
  - warning
  - danger
  - resolved
- typography helpers
- spacing/radius/shadow tokens
- reusable components:
  - urgency badge
  - due card
  - action row
  - timeline item
  - sync/debug badge

# Constraints

- keep accessibility contrast sane
- avoid overdesign
- no giant bespoke component zoo before product truth exists

# Acceptance criteria

- at least 5 reusable components adopted in iOS app
- watch uses reduced subset where appropriate
- tokens documented in `clients/apple/docs/design-system.md`

---
title: "Clean up the web shell and remove starter-template residue"
status: done
owner: agent
type: cleanup
priority: medium
created: 2026-03-15
depends_on: []
labels:
  - vel
  - web
  - ux
  - cleanup
---
The web client has real Vel UI work in it now, but it is still haunted by starter-template residue.

That stuff is not just cosmetic. It muddies architectural intent and makes the repo feel less finished than it is.

## Current residue / smells

- Vite starter assets still exist in `clients/web/src/assets/`.
- `index.css` still contains generic template-era typography/layout rules that do not clearly belong to the product UI.
- There is mixed styling authority between global CSS, App.css, and Tailwind utility classes.
- The current shell is serviceable, but visual hierarchy and information density still feel like scaffold rather than operator-grade interface.

## Tasks

- Remove unused Vite starter assets and leftover template CSS.
- Decide on styling authority:
  - Tailwind-first with thin globals, or
  - explicit component CSS boundaries
  Pick one. The repo currently smells like both.
- Tighten sidebar/main/context panel proportions and hierarchy for actual daily use.
- Add shared UI primitives for:
  - empty state
  - loading state
  - error state
  - section headers
  - action bars
- Make the message bubble/card distinction cleaner so structured assistant outputs read as product objects, not just chat blobs.

## Acceptance Criteria

- No leftover starter assets or irrelevant template styles remain.
- Styling strategy is consistent and documented.
- The shell feels intentionally Vel-shaped rather than Vite-with-ambition.
- Structured cards are visually legible and distinct from plain text messages.

## Notes for Agent

Nothing kills conceptual integrity faster than a repo that still has `react.svg` hanging around like a landlord-special chandelier.

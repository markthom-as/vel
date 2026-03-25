# Debug Harness

A debug harness is mandatory.

## Features

- slider control for every affect dimension
- discrete mode selector
- preset selector
- event buttons:
  - start listening
  - start thinking
  - start speaking
  - resolve
  - warn
  - overload
  - sleep
- packet preview panel
- morphology preview panel
- FPS / frame-time display
- storybook-like component gallery for:
  - attachment cards by kind (image, video, audio, markdown, link, file, raw object)
  - top-level thread object rendering variants
  - edge-case input payloads and empty/error states

## Why this matters

Without a harness, the agent will tune visuals by superstition and vibes alone, which is charming in theory and bad in repos.

## Future tooling

- Add a dedicated component preview toolchain (Storybook, Histoire, or equivalent) to own visual behavior for `clients/web/src/core` card and attachment primitives.
- Keep these fixtures deterministic and CI-validatable through screenshot or interaction checks before thread-level regression tests are considered complete.

## Current lightweight preview

- Runtime entrypoint: `clients/web/gallery.html`
- Dev command:
  - `npm run gallery:web` (from repo root)
  - `npm run gallery` (from `clients/web`)
- Coverage focus:
  - attachment cards (image/video/audio/markdown/link/file/raw)
  - top-level object cards (`thread`, `run`, `artifact`, `config`, `link`, `markdown`, `audio`)
  - action chips and markdown variants

## Tooling recommendation for vel-driven UX work

For rapid, low-friction iteration on nudges/chips/cards:

- **Now:** keep the Vite-powered gallery as the lightweight default.
- **Future:** evaluate `Storybook` or `Histoire` when the component matrix and interaction tests grow large enough to need fixture-level isolation.

This should remain the source of truth for component-level UX experiments before they are validated in full thread screens.

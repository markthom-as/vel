---
id: VEL-DOC-004
title: Clarify Apple client status and config defaults
status: proposed
priority: P0
owner: apple / docs
---

# Goal

Align Apple-related docs with current bootstrap implementation and default configuration.

# Scope

- `docs/status.md`
- Apple client docs under `clients/apple` (README or equivalents)
- any spec or status doc describing Apple client status

# Required changes

## 1. Adopt staged maturity language for Apple

Update wording to reflect:
- `bootstrap implemented`
- `product maturity deferred`

Avoid binary implemented/deferred language that obscures reality.

## 2. Fix daemon URL and port references

Update Apple README(s) so they document the actual current defaults:
- `127.0.0.1:4130` (or whatever the current veld defaults are)

If multiple environments or ports are supported, document them explicitly instead of implying a single hardcoded URL.

## 3. Cross-link to canonical status

Ensure Apple docs include a short note:
- repo-wide implementation status is tracked in `docs/status.md`
- this document focuses on Apple-specific architecture and usage.

# Acceptance criteria

- Apple docs no longer reference outdated host/port values.
- Apple status language uses bootstrap/product maturity framing.
- Apple docs do not contradict `docs/status.md`.


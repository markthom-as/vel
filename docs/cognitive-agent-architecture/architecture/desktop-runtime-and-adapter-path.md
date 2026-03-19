---
title: Desktop Runtime And Adapter Path
doc_type: spec
status: active
owner: staff-eng
created: 2026-03-19
updated: 2026-03-19
keywords:
  - desktop
  - tauri
  - daemon
  - adapter
  - runtime
index_terms:
  - desktop runtime path
  - tauri adapter
  - local daemon
  - in-process host
related_files:
  - docs/cognitive-agent-architecture/architecture/cross-surface-core-and-adapters.md
  - docs/cognitive-agent-architecture/architecture/cross-surface-contract-vocabulary.md
  - docs/api/runtime.md
summary: Future desktop and Tauri integration path for Vel, defining desktop as a shell over shared contracts rather than an independent product-core owner.
---

# Purpose

Define how future desktop packaging should consume the same Rust-owned product core as web, Apple, and CLI.

This document is intentionally future-facing. It does not imply that a desktop shell already exists.

# Current Truth

Today, desktop access is provided through:

- the local `veld` daemon
- the browser-based web shell
- the CLI

There is no shipped Tauri shell today.

# Core Rule

Desktop is a shell choice, not a product-core owner.

Future desktop work must consume the same:

- commands
- queries
- read models
- transport or adapter boundaries

used elsewhere in the system.

# Valid Future Desktop Modes

## Mode A: In-Process Desktop Host

The desktop host runs Rust in-process and exposes commands/events to the UI shell.

Best fit:

- early desktop experiments
- a thin packaged desktop shell where one process is operationally simpler

Tradeoff:

- weaker continuity for long-running runtime tasks
- tighter coupling between UI host lifecycle and runtime lifecycle

## Mode B: Desktop Shell Over Local Daemon

The desktop shell talks to a local `veld` daemon over localhost or another local boundary.

Best fit:

- long-running local authority
- shared access across multiple local surfaces
- crash/restart resilience for UI shells

Tradeoff:

- more packaging and process-management complexity

# Tauri Stance

Tauri is a valid future desktop packaging path.

Tauri should be treated as:

- a shell and host choice
- an adapter boundary choice

Tauri should not be treated as:

- the owner of business logic
- the place where product semantics are redefined
- the reason to create screen-shaped or desktop-only contract vocabularies

# Recommended Sequence

1. Keep desktop as a planned future shell while architecture and product discovery mature.
2. Prefer a local-daemon-compatible design even if an early experiment starts in-process.
3. Reuse existing backend-owned contracts first.
4. Introduce desktop-specific adapter crates or glue only when there is a concrete packaged desktop need.

# What Desktop May Own

- tray and menu integration
- packaged update flow
- desktop notification UX
- file and path affordances
- windowing and shell navigation behavior

# What Desktop Must Not Own

- product logic
- policy evaluation
- review-gate semantics
- domain invariants

# Acceptance Criteria

1. Tauri and future desktop are defined as shell/adaptor choices, not product-core owners.
2. In-process and local-daemon desktop modes are both named explicitly.
3. Future desktop work has a durable reference path that keeps it aligned with the same Rust-owned contracts as other shells.

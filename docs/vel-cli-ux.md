# Vel — CLI UX Specification

## Purpose

This document defines the initial command-line experience for Vel.

The CLI should feel like:

- a reliable operator shell
- a thin client to `veld`
- a natural daily interface
- something fast enough to use habitually

## Top-Level Command Shape

Recommended top-level commands:

```text
vel health
vel config
vel capture
vel search
vel today
vel morning
vel reflect
vel suggestions
vel project
vel goal
vel jobs
vel behavior
```

## Bootstrap Commands

These should exist first:

- `vel health`
- `vel capture`
- `vel config show`

## Command Notes

### `vel health`

Quickly verify daemon and dependency health.

### `vel capture`

Fast intake of ideas, notes, reminders, and quick thoughts.

### `vel config show`

Display resolved configuration and resolved daemon URL, DB path, artifact root, and log level.


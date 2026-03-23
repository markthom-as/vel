# Phase 66 Context

## Goal

Freeze the client truth model for `v0.5.1` before implementation begins.

## Boundary

This phase defines law. It does not implement client rebinding yet.

## Locked Decisions

- `v0.5.1` is a truth-alignment milestone, not a redesign or backend renegotiation
- surfaces are reduced to `Now`, `Threads`, and `System`
- `Inbox` and `Settings` are not first-class surfaces
- Apple is out of implementation scope
- backend contracts are immutable during this milestone except for provable bugs

## Required Outputs

- truthful-surface doctrine
- milestone requirement set
- explicit phase map for client reconnection work
- kill-list expectation for stale routes/shims

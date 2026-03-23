# Phase 73 UI Baseline Snapshot

## Purpose

This file preserves the recoverable reference point for the pre-Phase-73 web UI before shared shell and surface embodiment changes begin.

## Baseline Reference

- baseline commit: `a65801263fe2359a9f8529a0a5f7b42c666c997e`
- baseline tag: `v0.5.2-ui-baseline-pre-phase73`

## Recovery Rule

If the new shell or surface embodiment work needs to compare against or recover the prior UI state, use this baseline commit/tag rather than trying to reconstruct the old surface from memory.

## Baseline Scope

This baseline captures the shipped state immediately after:

- `v0.5.2` activation
- doctrine freeze
- Phase 72 UI contract approval

It is the last recoverable point before Phase 73 shared-shell work begins.

## Key Surface Files At Baseline

- [App.tsx](/home/jove/code/vel/clients/web/src/App.tsx)
- [AppShell.tsx](/home/jove/code/vel/clients/web/src/shell/AppShell/AppShell.tsx)
- [Navbar.tsx](/home/jove/code/vel/clients/web/src/shell/Navbar/Navbar.tsx)
- [MainPanel.tsx](/home/jove/code/vel/clients/web/src/shell/MainPanel/MainPanel.tsx)
- [NowView.tsx](/home/jove/code/vel/clients/web/src/views/now/NowView.tsx)
- [ThreadView.tsx](/home/jove/code/vel/clients/web/src/views/threads/ThreadView.tsx)
- [SystemView.tsx](/home/jove/code/vel/clients/web/src/views/system/SystemView.tsx)

## Notes

- This snapshot is for recovery and comparison, not authority. Ongoing implementation authority for `v0.5.2` remains the doctrine, packet, and approved [72-UI-SPEC.md](/home/jove/code/vel/.planning/milestones/v0.5.2-operator-surface-embodiment/72-UI-SPEC.md).

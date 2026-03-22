# Phase 64 Account And Window Proving Matrix

## Purpose

Make the bounded multi-account import posture concrete enough to verify and hard to erode later.

## Matrix

| Scenario | Required In Phase 64 | Notes |
| --- | --- | --- |
| one account / one calendar | yes | basic proving path |
| one account / multiple calendars | yes | proves calendar-level distinction under one account |
| multiple accounts / overlapping calendar concepts | yes | proves account identity remains canonical and separate |
| bounded import start/end enforcement | yes | default past 90 / future 365 posture |
| explicit expansion beyond default window | yes, narrow proving path | must remain operator- or policy-explicit |
| re-import idempotence | yes | must not duplicate canonical objects or `SyncLink` truth |
| `SyncLink` continuity across repeated imports | yes | preserves canonical linkage history |

## Rule

The default Google import posture in `0.5` is bounded and explainable.

It is not:

- infinite history by default
- provider-led mirroring without canonical reconciliation
- a shortcut around `IntegrationAccount` and `SyncLink` law

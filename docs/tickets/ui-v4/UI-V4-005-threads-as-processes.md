# UI-V4-005 — Upgrade threads into process-oriented continuity surfaces

Status: todo
Priority: P1
Lane: B

## Why

The screenshot set suggests threads are still under-expressed as continuity/process objects.

Evidence:

- `~/Downloads/localhost_5173_ (2).png`

## Goal

Make threads visibly encode continuity and process state.

## Required thread fields

- status
- type
- last activity

## Ownership / likely write scope

- thread list/detail UI
- thread projection/read model if more state is needed
- docs for thread semantics

## Acceptance criteria

- threads read like live processes rather than flat records
- continuity, recency, and type are visible without opening raw detail

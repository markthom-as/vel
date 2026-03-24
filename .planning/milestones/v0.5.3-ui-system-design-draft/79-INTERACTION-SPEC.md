---
phase: 79
slug: interaction-system-and-ux-state-law
status: draft
created: 2026-03-23
---

# Phase 79 — Interaction Spec

## Purpose

Define the UX-state model and interaction rules that all surfaces and components must share.

This document should answer:

- which states exist
- how actions behave
- what feedback timing is acceptable
- how disclosure, selection, and confirmation work
- how keyboard, pointer, and touch interactions stay consistent

## Global UX States

Every surface and primitive should use one shared state vocabulary:

- loading
- refreshing
- empty
- success
- warning
- blocked
- error
- degraded
- stale
- offline
- syncing
- selected
- expanded
- disabled
- destructive-confirmation

Each state should define:

- meaning
- visual treatment
- allowed actions
- whether the user can still proceed
- whether the state is transient or persistent

Confidence is a first-class cross-cutting signal and should influence:

- escalation decisions
- nudge treatment
- action affordances
- review posture

## Action Model

Actions should be categorized explicitly:

- primary action
- secondary action
- inline quick action
- navigational action
- escalatory action
- destructive action
- background/system action

Imported base action grammar:

- `Confirm`
- `Reject`
- `Complete`
- `Defer`
- `Open`
- `Inspect`
- `Run`
- `Dismiss`
- `Discuss`
- `Feedback`

Lifecycle and post-action verbs:

- `Archive`
- `Delete`
- `Restore`
- `Undo`
- `Review`
- `Retry`

For each category, define:

- placement rules
- visual emphasis
- confirmation requirements
- optimistic vs confirmed behavior

## Feedback Rules

- Lightweight actions may acknowledge optimistically if rollback is clear.
- Trust-sensitive actions should wait for backend confirmation before implying success.
- Full-surface reloads after small row actions are a failure mode unless unavoidable.
- Success and error feedback should appear near the acted object when possible.
- Action records should be durable and reviewable, not treated as disposable button clicks.

Optimistic by default:

- complete task
- dismiss nudge
- defer nudge
- toggle preference

Confirmation required:

- delete
- disconnect
- revoke auth
- destructive resets
- high-risk external actions

Destructive means:

- delete
- revoke
- reset
- disconnect

Not destructive by default:

- archive
- dismiss
- resolve

## Disclosure Rules

Supported disclosure modes:

- inline expansion
- surface drawer
- routed surface change
- thread escalation

Imported thread-content rule:

- bounded config work may appear inside a thread when it is local and contextual
- broad structural configuration, browsing, and installation complexity should escalate to `System`

Define:

- when each is appropriate
- how disclosure opens and closes
- how state persists across refresh and route changes
- how nested disclosure is limited

## Selection Rules

- Selected state must be durable enough to survive incremental refresh.
- Single-select versus multi-select patterns should be deliberate, not accidental.
- Selection and focus must not rely on color alone.

Thread-specific rules:

- continuity stream is the default open state
- filters are sticky per thread
- provenance stays collapsed by default and expands inline or through review

## Input and Navigation Rules

Define shared rules for:

- keyboard focus rings
- tab order
- escape behavior
- enter / space activation
- mobile touch targets
- hover-only affordance limitations
- reduced-motion behavior

## Shell Interaction Doctrine

Imported shell model:

- top orientation band for awareness
- persistent nudge zone for attention
- bottom floating action bar for voice, capture, ask, and command

Interpretation:

- top = awareness
- nudge zone = attention
- bottom = action/entry

Additional shell rules:

- shell chrome stays instrument-like and spatially consistent across surfaces
- nudge zone is always present, but compresses outside `Now`
- action bar is always visible except in extreme focus modes where it must be instantly recallable
- mobile uses a docked version of the action bar rather than a floating desktop-style overlay
- breadcrumbs appear only when needed in focused subviews

## Working Decisions Already Imported

- inline actions are preferred for low-risk, bounded, locally understandable work
- escalation is required for ambiguous, high-impact, multi-step, or history-heavy work
- `Feedback` is a first-class action, not an afterthought
- `Review` and `Retry` are primary post-action affordances where relevant
- inline feedback plus persistent review path is the default trust model
- critical actions must never hide behind hover
- secondary actions may reveal on hover, but primary and safety-critical actions stay visible

## Remaining Questions To Resolve In This Phase

- How should stale/degraded/offline differ in both copy and behavior?
- Which specific disclosures should be drawers versus route changes on each page?
- Which interaction patterns are explicitly banned beyond hover-hidden critical actions?

## Done When

1. State handling is consistent across all surfaces.
2. Action categories are globally defined.
3. Disclosure, selection, and confirmation patterns are explicit.
4. Components can inherit behavior rules instead of inventing them locally.

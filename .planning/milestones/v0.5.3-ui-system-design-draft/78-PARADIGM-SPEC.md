---
phase: 78
slug: product-paradigms-and-surface-doctrine
status: draft
created: 2026-03-23
---

# Phase 78 — Paradigm Spec

## Purpose

Define the operating model for the web client before visual and implementation decisions spread.

This document should answer:

- what each top-level surface is for
- what kinds of work stay inline
- what escalates into deeper context
- what qualifies as a primary action
- how supervision, continuity, and trust should feel across the product

## Core Product Posture

- Vel is an operator surface, not a dashboard and not a chatbot wrapper.
- The user should be able to identify current context, next useful action, and escalation path with minimal interpretation.
- The UI should prefer compact current-day usefulness over broad information browsing.
- The system must remain trust-forward: visible states must be explainable from backend-owned truth.

Imported doctrine from the 2026-03-23 UI packs:

- `Now`: what demands action now
- `Threads`: where action is understood, discussed, executed, and reviewed
- `System`: where the machine is inspected, configured, repaired, and personalized

## Surface Paradigms

### `Now`

`Now` is the execution-first current-day control surface.

It should answer:

- what matters now
- what is active
- what is next
- what needs intervention
- where deeper context lives if the answer is not inline

Its dominant first-class object is:

- the active task

Its secondary first-class objects are:

- one subordinate next item slot
- nudges
- current or near-horizon events

Persistent voice/input may remain visible, but should be visually subordinate to active work.

Hard bounds:

- active task remains the dominant row
- at most one or two next items
- at most two visible events: current and next
- no meaningful long-scroll behavior
- no project grouping
- no today-dump posture

`Now` is not:

- a dashboard
- a backlog browser
- a planner studio
- a transcript-first assistant surface

Explicitly forbidden on `Now`:

- messages
- threads
- artifacts
- runs or logs
- people
- raw integrations
- config

### `Threads`

`Threads` is the continuity and depth surface.

It should answer:

- what conversation or context already exists for this object
- what changed
- what evidence or provenance matters
- what next action or response is appropriate

`Threads` is not:

- the default first-glance landing page
- generic chat detached from objects and history

Thread doctrine imported from the v2 pack:

- threads are stable continuity frames
- threads support mixed content and focused subviews
- objects are not owned by threads and may relate to multiple threads
- thread context may contain conversation, workflows, media, traces, generated artifacts, and bounded config work

### `System`

`System` is the structural and support surface.

It should answer:

- whether the system is trustworthy
- what capabilities are connected or degraded
- what configuration or recovery actions are available
- what the user can inspect without widening product logic

`System` is not:

- a second product dashboard
- a dumping ground for docs or unrelated controls

`System` remains one top-level surface with these first-level subsections:

- `Overview`
- `Operations`
- `Integrations`
- `Control`
- `Preferences`

Posture by subsection:

- `Overview` and `Integrations`: read-first
- `Operations` and `Control`: more operational and console-like
- `Preferences`: personal presentation, accessibility, and interaction settings

## Escalation Rules

The client should use clear escalation boundaries:

- inline when the user can understand and act without opening another context
- drawer when more detail is useful but the current task remains primary
- thread when continuity, judgment, history, or collaboration is required
- system detail when the problem is trust, sync, capability, or configuration

Imported escalation rule:

- inline when risk is low, confidence is high enough, side effects are bounded, and local context suffices
- escalate when ambiguity is high, the action is multi-step, review/history is needed, or side effects exceed local context

Escalation target clarification:

- drawer = small inspection and shallow context
- thread = thinking, resolving, deciding
- system detail = trust, integration, repair, and structural configuration

## Primary-Action Doctrine

- Each surface should have one obvious primary action zone at a time.
- Primary actions should correspond to canonical product behavior, not convenience shortcuts that bypass the model.
- Secondary actions may be present but should not compete with the primary action.
- Destructive actions must never look equivalent to progress actions.

`Now` rule:

- one dominant slot plus one subordinate slot
- anything beyond that becomes decision fatigue

## Object and Meaning Taxonomy

The milestone should settle the durable object language used in copy, color, and component behavior:

- task
- thread
- event
- capture
- project
- client
- message
- artifact
- action
- relation
- run / agent activity
- person
- note
- review
- nudge
- system condition

Each object type should have:

- a stable meaning
- a stable visual identity family
- a known set of allowed actions
- a clear escalation path

## Working Decisions Already Imported

- the active task is dominant on `Now`
- persistent entry lives in a bottom floating action bar
- shell awareness stays in a top orientation band
- nudges belong in a stable attention zone rather than scattered ad hoc
- `System` owns trust/config/repair depth instead of letting those concerns leak across the shell
- projects are tag-only on `Now`, stronger in `Threads`, and first-class in `System`
- task wins visually on `Now`, while event wins behaviorally as a constraint
- nudges render in a dedicated lane on `Now`
- trust state on `Now` appears only when degraded or critical
- completed `Now` items disappear immediately rather than accumulating history

## Remaining Questions To Resolve In This Phase

- Which current UI patterns violate the intended surface doctrine and must be retired immediately?
- Are there any additional object types that deserve thread-level prominence without becoming `Now` rows?

## Done When

1. Each top-level surface has a short, testable product definition.
2. Inline versus escalation rules are explicit.
3. Primary-action doctrine is clear enough to constrain page designs.
4. Object language is stable enough to drive the interaction and visual system.

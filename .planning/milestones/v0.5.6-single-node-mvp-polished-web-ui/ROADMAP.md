# Milestone v0.6.0: Single-Node MVP and Polished Web UI

**Status:** QUEUED
**Milestone:** v0.6.0
**Source of truth:** [TODO.md](/home/jove/code/vel/TODO.md)

## Overview

`v0.6.0` is the next milestone after the closed `0.5.5` API/functionality/polish line.

Its purpose is to turn the currently accepted three-surface web line into a credibly working single-node MVP:

- one local node
- working Google/Todoist/chat-backed behavior
- truthful current-day operator state
- polished web UI across `Now`, `Threads`, and `System`

Additional phases after this line should come from subsequent operator feedback instead of being planned speculatively now.

## Inputs

- [TODO.md](/home/jove/code/vel/TODO.md)
- [v0.5.5-MILESTONE-AUDIT.md](/home/jove/code/vel/.planning/v0.5.5-MILESTONE-AUDIT.md)
- [00-FEEDBACK-TODO.md](/home/jove/code/vel/.planning/milestones/v0.6.0-single-node-mvp-polished-web-ui/00-FEEDBACK-TODO.md)

## In Scope

- resolve the remaining single-node usability gaps still visible in `TODO.md`
- make integrations/settings/chat/provider selection work as part of the product, not as partial demos
- finish the accepted `Now`/`Threads`/`System` fidelity issues that block a polished web UI
- keep the work bounded to one-node truth and operator use, not distributed follow-on architecture

## Out of Scope

- every `TODO.md` bullet prefixed with `!`
- new top-level surfaces
- Apple implementation
- speculative multi-node, swarm, or planner-studio widening
- broader future-product phases not yet requested by operator feedback

## Requirement Buckets

| ID | Description |
|----|-------------|
| MVP-60-01 | Single-node setup, settings, integrations, and runtime behavior are operational enough for real operator use. |
| NOW-60-01 | `Now` reflects truthful current-day task and calendar state with correct inbox / next-up / backlog semantics. |
| CHAT-60-01 | Multimodal assistant/chat works across the supported local and hosted providers with bounded configuration. |
| SYSTEM-60-01 | `System` becomes the truthful operator surface for single-node config, integrations, and developer-mode disclosure. |
| POLISH-60-01 | The web UI reaches accepted polish across navbar, nudges, composer, `Now`, `Threads`, and `System`. |
| VERIFY-60-01 | Browser/API/manual proof closes the milestone honestly. |

## Planned Phases

### Phase 97: MVP scope lock and feedback-to-contract mapping

**Goal:** convert `TODO.md` into explicit single-node MVP acceptance criteria without losing the operator’s exact wording.  
**Depends on:** `0.5.5` closeout
**Status:** QUEUED

Expected outcomes:

- direct mapping from verbatim `TODO.md` feedback into requirement buckets and acceptance checks
- explicit in-scope vs ignored-`!` separation
- clarified single-node MVP contract for integrations, chat, and current-day truth
- no silent carryover from previous milestone debt lists

### Phase 98: Single-node truth and settings/integrations completion

**Goal:** make the single-node runtime/config/integration seams truthful enough for real use.  
**Depends on:** Phase 97
**Status:** QUEUED

Expected outcomes:

- Google and Todoist integrations are configurable/editable in `System` and feed `Now` truthfully
- assistant/chat provider backing and priority are configurable through truthful persisted settings
- supported chat paths work for local `llama.ccp`, Claude, and OpenAI including OAuth/key handling where required
- client location/config support exists at the backend/settings seam

### Phase 99: Web surface polish and operator-flow completion

**Goal:** finish the accepted web UI and interaction model across the three surfaces.  
**Depends on:** Phase 98
**Status:** QUEUED

Expected outcomes:

- navbar, composer, and nudge behavior match the accepted feedback
- `Now` task/event/lane semantics and drag/drop feel polished and trustworthy
- `Threads` default active state, duplicate-send behavior, archive/header treatment, and modern message layout are fixed
- `System` density, sticky navigation, developer-mode disclosure, iconography, and section layout hit the intended quality bar

### Phase 100: MVP proof, audit, and milestone closeout

**Goal:** prove the single-node MVP works end to end and close the line honestly.  
**Depends on:** Phase 99
**Status:** QUEUED

Expected outcomes:

- browser proof exists for `Now`, `Threads`, and `System`
- manual/API checks exist for integrations, settings mutation, and chat provider paths
- final audit runs directly against verbatim copied feedback
- anything still open is deferred explicitly instead of being normalized away

## Execution Order

Planned sequence:

`97 -> 98 -> 99 -> 100`

## Verbatim Feedback Copy

This milestone intentionally keeps the raw operator feedback inside the packet.

See [00-FEEDBACK-TODO.md](/home/jove/code/vel/.planning/milestones/v0.6.0-single-node-mvp-polished-web-ui/00-FEEDBACK-TODO.md).

## Initial Acceptance Standard

`v0.6.0` should only close when:

- one local node can be used as a working MVP through the web UI
- integrations and chat paths required by the feedback are actually working, not simulated
- the web UI is polished across the existing three surfaces
- the copied `TODO.md` feedback has been re-audited directly before closeout

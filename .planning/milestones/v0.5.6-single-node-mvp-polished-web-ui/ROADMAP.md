# Milestone v0.5.6: Single-Node MVP and Polished Web UI

**Status:** SHIPPED 2026-03-25
**Milestone:** v0.5.6
**Source of truth:** [TODO.md](/home/jove/code/vel/TODO.md)

## Overview

`v0.5.6` was the follow-on milestone after the closed `0.5.5` API/functionality/polish line.

Its purpose is to turn the currently accepted three-surface web line into a credibly working single-node MVP:

- one local node
- working Google/Todoist/chat-backed behavior
- truthful current-day operator state
- polished web UI across `Now`, `Threads`, and `System`

The MVP bar for this milestone is now explicitly defined by operator answers:

- first-run onboarding through an onboarding nudge into Core settings
- Core setup for user identity, node identity, at least one LLM provider, at least one synced provider, and universal agent-profile context
- floating composer disabled until minimum Core setup is complete, with a developer override
- Google and Todoist integrations fully usable through `System` and reflected truthfully in `Now`
- local `llama.ccp` and OpenAI working as the minimum required multimodal assistant providers
- thread-level call mode using speech-to-text plus text-to-speech over the normal assistant flow
- polished desktop Chrome experience as the acceptance browser

Additional phases after this line should come from subsequent operator feedback instead of being planned speculatively now.

## Inputs

- [TODO.md](/home/jove/code/vel/TODO.md)
- [v0.5.5-MILESTONE-AUDIT.md](/home/jove/code/vel/.planning/v0.5.5-MILESTONE-AUDIT.md)
- [00-FEEDBACK-TODO.md](/home/jove/code/vel/.planning/milestones/v0.5.6-single-node-mvp-polished-web-ui/00-FEEDBACK-TODO.md)

## In Scope

- resolve the remaining single-node usability gaps still visible in `TODO.md`
- make integrations/settings/chat/provider selection work as part of the product, not as partial demos
- finish the accepted `Now`/`Threads`/`System` fidelity issues that block a polished web UI
- keep the work bounded to one-node truth and operator use, not distributed follow-on architecture
- keep manual operator QA as the closeout authority

## Out of Scope

- every `TODO.md` bullet prefixed with `!`
- new top-level surfaces
- Apple implementation
- speculative multi-node, swarm, or planner-studio widening
- broader future-product phases not yet requested by operator feedback
- per-thread/per-provider operator-profile overrides
- broader task-field depth such as subtasks, comments, and recurrence
- mobile-web acceptance beyond not-breaking obvious layouts

## Requirement Buckets

| ID | Description |
|----|-------------|
| MVP-56-01 | Single-node setup, settings, integrations, and runtime behavior are operational enough for real operator use. |
| NOW-56-01 | `Now` reflects truthful current-day task and calendar state with correct inbox / next-up / backlog semantics. |
| CHAT-56-01 | Multimodal assistant/chat works across the supported local and hosted providers with bounded configuration. |
| SYSTEM-56-01 | `System` becomes the truthful operator surface for single-node config, integrations, and developer-mode disclosure. |
| POLISH-56-01 | The web UI reaches accepted polish across navbar, nudges, composer, `Now`, `Threads`, and `System`. |
| VERIFY-56-01 | Browser/API/manual proof closes the milestone honestly. |

## Planned Phases

### Phase 97: MVP scope lock and feedback-to-contract mapping

**Goal:** convert `TODO.md` into explicit single-node MVP acceptance criteria without losing the operator’s exact wording.  
**Depends on:** `0.5.5` closeout
**Status:** COMPLETE

Expected outcomes:

- direct mapping from verbatim `TODO.md` feedback into requirement buckets and acceptance checks
- explicit in-scope vs ignored-`!` separation
- clarified single-node MVP contract for integrations, chat, and current-day truth
- no silent carryover from previous milestone debt lists
- explicit Core-settings / onboarding / developer-override rules
- explicit operator-mode vs developer-mode disclosure rules
- execution checklist exists as a concrete phase packet rather than roadmap prose only

### Phase 98: Single-node truth and settings/integrations completion

**Goal:** make the single-node runtime/config/integration seams truthful enough for real use.  
**Depends on:** Phase 97
**Status:** COMPLETE

Expected outcomes:

- Google and Todoist integrations are configurable/editable in `System` and feed `Now` truthfully
- assistant/chat provider backing and priority are configurable through truthful persisted settings
- supported chat paths work for local `llama.ccp` and OpenAI including OAuth/key handling
- client location/config support exists at the backend/settings seam
- universal operator-profile context is editable and injected across providers
- onboarding nudge and Core-settings completion gating are truthful product behavior, not frontend-only state
- execution packet exists for Core settings, providers, integrations, `Now` day truth, and call-mode seams
- overdue-plus-today `Now` truth is verified across backend and web, while richer bedtime/end-of-day and sunrise boundary signals remain deferred until the runtime has those signals

### Phase 99: Web surface polish and operator-flow completion

**Goal:** finish the accepted web UI and interaction model across the three surfaces.  
**Depends on:** Phase 98
**Status:** COMPLETE

Expected outcomes:

- navbar, composer, and nudge behavior match the accepted feedback
- `Now` task/event/lane semantics and drag/drop feel polished and trustworthy
- `Threads` default active state, empty state, duplicate-send behavior, archive/header treatment, no-tail message layout, and call-mode entry are fixed
- `System` density, sticky navigation, developer-mode disclosure, iconography, and section layout hit the intended quality bar
- integration/provider failures surface through actionable nudges with retry when appropriate
- execution packet exists for navbar/composer/nudges plus `Now`, `Threads`, and `System` polish slices

### Phase 100: MVP proof, audit, and milestone closeout

**Goal:** prove the single-node MVP works end to end and close the line honestly.  
**Depends on:** Phase 99
**Status:** COMPLETE

Expected outcomes:

- browser proof exists for `Now`, `Threads`, and `System`
- manual/API checks exist for integrations, settings mutation, and chat provider paths
- final audit runs directly against verbatim copied feedback
- anything still open is deferred explicitly instead of being normalized away
- manual operator QA in desktop Chrome is the milestone close authority
- execution packet exists for onboarding/provider/integration proof and direct MVP audit

## Execution Order

Planned sequence:

`97 -> 98 -> 99 -> 100`

## Phase 97 Packet

- [97-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5.6-phases/97-mvp-lock-and-acceptance-checklist/97-CONTEXT.md)
- [97-01-PLAN.md](/home/jove/code/vel/.planning/milestones/v0.5.6-phases/97-mvp-lock-and-acceptance-checklist/97-01-PLAN.md)

## Phase 98 Packet

- [98-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5.6-phases/98-single-node-truth-and-settings-integrations/98-CONTEXT.md)
- [98-01-PLAN.md](/home/jove/code/vel/.planning/milestones/v0.5.6-phases/98-single-node-truth-and-settings-integrations/98-01-PLAN.md)

## Phase 99 Packet

- [99-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5.6-phases/99-web-surface-polish-and-operator-flow-completion/99-CONTEXT.md)
- [99-01-PLAN.md](/home/jove/code/vel/.planning/milestones/v0.5.6-phases/99-web-surface-polish-and-operator-flow-completion/99-01-PLAN.md)
- [99-01-SUMMARY.md](/home/jove/code/vel/.planning/milestones/v0.5.6-phases/99-web-surface-polish-and-operator-flow-completion/99-01-SUMMARY.md)
- [99-VERIFICATION.md](/home/jove/code/vel/.planning/milestones/v0.5.6-phases/99-web-surface-polish-and-operator-flow-completion/99-VERIFICATION.md)

## Phase 100 Packet

- [100-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5.6-phases/100-mvp-proof-audit-and-closeout/100-CONTEXT.md)
- [100-01-PLAN.md](/home/jove/code/vel/.planning/milestones/v0.5.6-phases/100-mvp-proof-audit-and-closeout/100-01-PLAN.md)

## Verbatim Feedback Copy

This milestone intentionally keeps the raw operator feedback inside the packet.

See [00-FEEDBACK-TODO.md](/home/jove/code/vel/.planning/milestones/v0.5.6-single-node-mvp-polished-web-ui/00-FEEDBACK-TODO.md).

## Initial Acceptance Standard

`v0.5.6` should only close when:

- one local node can be used as a working MVP through the web UI
- integrations and chat paths required by the feedback are actually working, not simulated
- the web UI is polished across the existing three surfaces
- the copied `TODO.md` feedback has been re-audited directly before closeout

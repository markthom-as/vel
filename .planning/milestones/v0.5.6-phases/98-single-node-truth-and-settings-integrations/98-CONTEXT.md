# Phase 98 Context

## Goal

Implement the single-node truth seams that make `0.5.6` a real MVP instead of a polished mock: Core setup, provider configuration, integration lifecycle, and the backend/settings contracts that `Now`, `Threads`, and the composer depend on.

## Preconditions

- [97-CONTEXT.md](/home/jove/code/vel/.planning/milestones/v0.5.6-phases/97-mvp-lock-and-acceptance-checklist/97-CONTEXT.md) has locked the MVP bar
- [97-01-PLAN.md](/home/jove/code/vel/.planning/milestones/v0.5.6-phases/97-mvp-lock-and-acceptance-checklist/97-01-PLAN.md) defines the execution checklist
- `0.5.6` scope is now bounded to one node, desktop Chrome acceptance, local `llama.ccp`, OpenAI, Google, and Todoist

## Main implementation targets

- create a truthful Core settings surface and backing contract
- persist onboarding-complete vs onboarding-blocked state
- gate the floating composer/task bar on Core readiness, with a developer override
- expose provider priority, ask-first behavior, and universal operator-profile injection through persisted settings
- make Google and Todoist connect/reconnect/edit flows truthful through `System`
- expose backend truth needed by `Now` day semantics, drag-to-commit behavior, and current-day event boundaries
- add the backend/runtime seams required for thread-level call mode and multimodal provider use

## Boundaries

### In scope

- Rust/service/storage/API work required to make the MVP settings and integrations truthful
- matching web boundary/data-layer updates in the same slice
- focused verification for changed contracts and integration flows

### Out of scope

- final visual polish
- broad provider expansion beyond local `llama.ccp` and OpenAI
- Apple/mobile implementation
- broad task-model expansion such as subtasks, comments, or recurrence

## Risks

- onboarding and Core settings can sprawl into a general settings redesign if not kept bounded
- provider abstraction can become over-generalized if provider-specific escape hatches are not preserved
- integration auth/edit flows may expose stale or UI-local behavior if persistence and boundary updates do not land together
- call mode can widen into a realtime-platform project unless it stays bounded to STT/TTS over the normal assistant thread flow

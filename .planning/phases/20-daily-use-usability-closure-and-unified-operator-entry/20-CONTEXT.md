# Phase 20 Context

## Purpose

Phase 20 exists because the current product substrate is broad enough to be impressive, but still not usable enough for repeated daily life. The operator explicitly chose to defer more milestone-closeout bookkeeping and prioritize usability work that makes Vel something they can actually live in.

That usability work is now explicitly assistant-centric: the user wants Vel's LLM to be aware of real Vel data, to use Vel as tools, to support morning/daily/closeout and thread-based resolution, and to let voice flow into the same assistant path. The immediate phase should therefore turn the grounded Rust-owned assistant into the default operator entry instead of treating it as an isolated chat feature.

## Product Direction

The core shape is already decided:

- `Now` is the compact urgent-first orientation surface
- `Inbox` is the explicit triage queue
- `Threads` is continuity, archive, and deeper interaction
- `Projects` is secondary context, not the primary work surface
- the backend owns policy, action semantics, `check_in`, `reflow`, trust/readiness, and thread escalation

Phase 20 should improve how that shipped model feels in use.

## Expected Focus

This phase should likely focus on:

1. `Now` usability
   - better contextual urgency handling
   - cleaner compact context strip / top area
   - stronger actionable cards when something really needs attention

2. Grounded assistant entry
   - one clearer path for capture, conversation, and quick operator intent over the grounded assistant seam
   - avoid separate fragmented “chat vs capture vs command” affordances where possible
   - keep the assistant bounded to real Vel data and explicit tool surfaces rather than generic freeform prompting
   - preserve optional localhost `openai_oauth` routing without making remote models a dependency for core use

3. `Inbox` / `Threads` usability
   - clearer triage versus continuity boundary in practice
   - reduced friction getting from summary state into the right deeper interaction
   - thread continuity for assistant work without inventing a separate assistant archive

4. Settings/setup friction reduction
   - make the default experience less settings-heavy
   - push more advanced/runtime complexity out of the operator’s immediate path

## Non-Goals

- broad new provider/platform expansion
- another architecture-definition phase
- milestone archival bookkeeping
- re-opening the already-decided product taxonomy unless real usage proves it wrong
- Apple voice parity, desktop push-to-talk parity, and assistant-capable daily loop/closeout closure beyond what is needed to establish the Phase 20 grounded entry seam

## Inputs

- [docs/product/operator-surface-taxonomy.md](/home/jove/code/vel/docs/product/operator-surface-taxonomy.md)
- [docs/product/now-inbox-threads-boundaries.md](/home/jove/code/vel/docs/product/now-inbox-threads-boundaries.md)
- [docs/product/operator-action-taxonomy.md](/home/jove/code/vel/docs/product/operator-action-taxonomy.md)
- [docs/product/operator-mode-policy.md](/home/jove/code/vel/docs/product/operator-mode-policy.md)
- [docs/api/chat.md](/home/jove/code/vel/docs/api/chat.md)
- [docs/user/daily-use.md](/home/jove/code/vel/docs/user/daily-use.md)
- the shipped Phase 15-17 seams and shell embodiment work

## Exit Condition

Phase 20 is successful when the grounded assistant is a credible default entry for daily use and the default operator loop feels materially more usable in practice, not just more feature-complete on paper.

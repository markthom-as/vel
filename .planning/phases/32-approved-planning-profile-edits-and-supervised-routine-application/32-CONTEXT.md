# Phase 32 Context

## Phase

32 — Approved planning-profile edits and supervised routine application

## Why this phase exists

Phase 30 made the canonical planning profile directly manageable from web `Settings`.
Phase 31 then extended read parity and staged edit parity across CLI, Apple, and assistant/voice entry.

What is still missing is the real supervised follow-through lane after a planning-profile edit is staged conversationally.
Today assistant entry and Apple voice can recognize a bounded routine/planning request, stage a typed `PlanningProfileEditProposal`, and attach explicit `Threads` continuity metadata, but that proposal does not yet have a shipped approval/application path that resolves back through the canonical planning-profile mutation seam.

That leaves an important usability gap:

- the conversational path can propose changes
- the canonical backend profile can be edited directly from `Settings`
- but the supervised bridge between those two states is still incomplete

Phase 32 closes that gap without weakening planner authority.

## Intended outcome

Vel should support one supervised story for planning-profile changes:

- direct operator edits still go through the canonical backend planning-profile mutation seam
- assistant and Apple voice may stage bounded proposals over that same model
- approved proposals then apply through the same canonical mutation seam, not through shell-local shortcuts
- proposal state, approval, failure, and applied outcomes remain explicit and inspectable in `Threads` continuity and other summary-first surfaces

## Constraints

- Do not create a second planner API or shell-owned planning model.
- Do not let assistant or voice paths silently mutate the saved planning profile.
- Keep the authority in Rust backend layers and reuse the existing typed planning-profile mutation model.
- Preserve explicit review, provenance, and thread continuity for conversational edits.
- Keep scope bounded to routine blocks and planning constraints already covered by the canonical planning-profile vocabulary.
- Do not widen into broad calendar editing, multi-day autonomous planning, or open-ended natural-language planner mutation.

## Likely touch points

- `crates/vel-core/src/planning.rs`
- `crates/vel-api-types/src/lib.rs`
- `crates/veld/src/services/planning_profile.rs`
- `crates/veld/src/services/chat/messages.rs`
- `crates/veld/src/services/apple_voice.rs`
- `crates/veld/src/routes/chat.rs`
- `crates/veld/src/routes/planning_profile.rs`
- `crates/veld/src/routes/threads.rs`
- `clients/web/src/components/`
- `clients/web/src/types.ts`
- `clients/apple/`
- `crates/vel-cli/`
- `docs/api/`
- `docs/user/`
- `docs/product/`

## Planning notes

- Prefer reusing the existing proposal-lifecycle and supervised-application patterns from assistant action phases rather than inventing planning-profile-specific exceptions.
- Keep `Threads` as the durable continuity lane for multi-step follow-through.
- Keep `Settings` summary-first; it can reflect approved outcomes and proposal pressure, but it should not become a second review queue.
- Preserve the current Apple/browser split for transport (`/v1/apple/voice/turn` vs `/api/assistant/entry`) while keeping one backend-owned proposal/application model.

# Onboarding, Trust, and Recovery Journeys

This document defines the canonical operator journeys for onboarding, trust, recovery, and adjacent summary-first routing in Vel.

It exists to answer one product question clearly:

- when Vel needs setup, clarification, or recovery work, how should it route the operator without teaching raw implementation categories first?

This is a Phase 14 discovery artifact and product-contract document. It is not a final UI spec.

## Core Rule

Vel should route the operator through:

1. summary-first awareness
2. one suggested next action
3. optional deeper inspection
4. only then raw diagnostics or implementation-aware recovery

This applies across:

- onboarding
- connector setup
- trust/readiness
- freshness degradation
- reflow and schedule drift
- context repair and missing metadata

## Journey Shapes

### 1. First-Time Onboarding

Goal:

- get the operator to a usable daily loop quickly without forcing them through every advanced capability first

Default path:

1. first-use advisory appears
2. operator starts onboarding from the advisory or Settings
3. Vel gathers identity, goals, baseline connectors, and preferences
4. Vel returns the operator to `Now` with the next safe setup step summarized there

Happy-path onboarding should gather:

- user name
- goals
- calendar connection
- todo connection
- notes connection
- wake time and bed time when not otherwise available
- nudging preferences, with sensible defaults and a skip path

On Apple platforms, onboarding should also ask whether to enable Apple-specific capability groups such as:

- health
- reminders
- messages
- activity
- location

The result should determine which functionality modules are turned on rather than pretending every capability exists by default.

### 2. Ongoing Setup And Recovery

Goal:

- help the operator repair or finish setup without treating Settings as a junk drawer

Default path:

1. `Now` or Settings shows a short summary of the missing or degraded setup state
2. Vel proposes one next action
3. the operator can accept that action directly or open the deeper setup surface
4. only unresolved or implementation-aware issues should push into advanced inspection

Examples:

- connector needs setup
- linked client is not fully paired
- local Apple path discovery failed
- writeback trust is disabled

### 3. Trust And Readiness

Goal:

- let the operator know whether Vel is safe enough to trust for the current workflow

Default path:

1. `Now` shows summary-level trust/readiness state
2. if action is needed, the operator sees a short explanation plus a primary action
3. deeper trust inspection remains available in Settings or the relevant advanced surface

Examples of summary-level trust state:

- backup freshness
- writeback disabled
- blocked review count
- stale or degraded source data
- pending supervised review

Implementation rule for the current migration lane:

- the first backend-owned trust/readiness seam may compose backup trust, freshness, pending writebacks/conflicts, and supervised execution review pressure into one typed `Now` summary
- that projection should stay summary-first and action-oriented, with deeper inspection still routed through advanced surfaces
- degraded trust/readiness should also expose typed backend-owned follow-through actions, reusing the canonical queue/action model rather than inventing shell-specific recovery buttons

### 4. Freshness Recovery

Goal:

- recover degraded or stale input state without making the user spelunk into diagnostics

Default path:

1. `Now` shows a freshness summary and any source-specific warnings
2. if appropriate, Vel suggests `recover_freshness`
3. if the stale state undermines the current day plan, Vel may also suggest `reflow`
4. raw diagnostic details remain one step deeper

Key rule:

- stale data should first produce a summary and a recovery action
- it should not immediately dump the operator into runtime internals

### 5. Check-In For Context Repair

Goal:

- ask the operator for the missing piece of reality when inference alone is not enough

Default path:

1. `Now` shows an inline `check_in` card
2. the card explains what Vel is unsure about
3. the card offers a suggested action or answer when possible
4. if the situation is more complex, the operator can continue in `Threads`

Behavior note:

- a blocking `check_in` should remain pinned until handled or explicitly bypassed
- bypass is acceptable, but it should require a warning and a brief operator note
- suggested bypass reasons with optional custom voice/text input are preferred

Typical uses:

- confirm whether something was completed, skipped, or changed
- gather missing metadata
- update current context after drift
- gather input before a schedule reflow

### 6. Reflow And Schedule Drift

Goal:

- recalculate the day when the current plan is no longer trustworthy

Default path:

1. Vel detects drift or stale schedule state
2. `Now` surfaces a heavier `reflow` action with a compact `Day changed` preview
3. the operator confirms the reflow
4. Vel presents the recalculated result and any leftovers that still need review

`Reflow` should be:

- auto-suggested
- user-confirmed
- visually heavier than routine nudges or check-ins
- editable through `Threads` when the operator wants to shape the recalculation instead of simply accepting it
- severity-aware in how much confirmation it requires before applying

Implementation rule for the current migration lane:

- the first backend-owned `reflow` seam may derive from typed current-context drift plus schedule freshness/event timing inputs
- this seam should stay daily-loop-adjacent and `Now`-consumable rather than becoming a broad planner abstraction
- `Accept` and `Edit` branching should remain typed backend metadata, with `Edit` escalating toward `Threads`
- once handled, the backend should persist a typed reflow follow-up status so shells can suppress the original card and render durable applied/editing consequences without inferring them locally

Likely triggers:

- stale schedule
- missed event
- slipped planned block
- major sync change affecting today
- task no longer fits the remaining available time

## Surface Routing Rules

### `Now`

Use `Now` for:

- summary-level onboarding blockers
- trust/readiness summaries
- freshness warnings
- inline `check_in` cards
- heavier `reflow` suggestions

Visibility rule:

- if something is not urgent, `Now` should usually summarize it with badges, counts, or deep links rather than render it as a direct action card
- those lighter indicators should usually live in a subtle context bar or equivalent compact area and should normally deep-link into the relevant filtered `Inbox` view

`Now` should usually answer:

- what is wrong right now?
- what is the next safe action?

### `Inbox`

Use `Inbox` for:

- explicit triage and review items created by the journeys above
- unresolved items that require sorting, commitment, reply, review, or classification

Identity rule:

- if an inbox item is project-scoped, the project tag/color should remain visible in the queue rather than disappearing into generic triage

`Inbox` should not become the main setup or trust explanation surface.

### `Threads`

Use `Threads` for:

- longer interactive clarification flows
- deeper thread-linked recovery context
- extended check-in conversations

Durability rule:

- a longer clarification or edit flow should usually become durable once it is meaningfully multi-step
- not every one-step inline interaction needs its own durable thread

`Threads` should support the journey, but it should not be the default first-contact trust or onboarding surface.

### `Settings`

Use `Settings` for:

- relaunchable onboarding
- connector configuration and repair
- trust inspection
- advanced runtime and implementation-aware recovery

`Settings` should remain reachable, but the default product should not require the operator to begin there.

## Apple, Web, and CLI Alignment

Apple:

- should stay summary-first
- should surface bounded onboarding, trust, freshness, and check-in cues
- should escalate deeper control to web or CLI unless there is a strong mobile reason

Web:

- is the richest default shell for summary-first routing plus deeper drill-down

CLI:

- may expose more direct inspection and recovery paths
- should still preserve the same journey semantics instead of inventing a separate product model

## What This Avoids

This journey model prevents:

- setup being buried in Settings with no default guidance
- trust being taught as diagnostics first
- stale schedule and missed events becoming passive warnings with no recalculation path
- check-ins being treated as random chat instead of product-owned context repair
- shell-specific routing rules diverging across Apple, web, and CLI

## Acceptance Criteria

1. Onboarding, trust, freshness recovery, check-in, and reflow all have summary-first default paths.
2. `Now` is the main summary-and-next-action surface for these journeys.
3. `Inbox` remains the triage queue rather than becoming the setup or trust home.
4. `Threads` is available for longer clarification flows without becoming the default first-contact surface.
5. Settings remains the deeper configuration and inspection surface rather than the first place the operator must learn.

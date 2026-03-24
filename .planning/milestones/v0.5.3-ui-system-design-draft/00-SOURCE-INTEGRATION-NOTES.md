# Source Integration Notes

This draft milestone packet incorporates decisions from two external UI spec packs found in `/home/jove/Downloads`:

- `vel_ui_spec_v2.zip` dated 2026-03-23
- `vel_ui_spec.zip` dated 2026-03-23

The v2 pack was treated as the higher-authority input where the two differed because it is more complete and more implementation-oriented.

## Imported decisions

- Top-level surfaces remain `Now`, `Threads`, and `System`.
- `Now` remains first-glance operational and execution-first.
- `Threads` remains a stable continuity frame with mixed-content blocks and focused subviews.
- `System` remains a single top-level surface with first-level subsections:
  - `Overview`
  - `Operations`
  - `Integrations`
  - `Control`
  - `Preferences`
- Shell doctrine is spatially stable:
  - top orientation band for awareness
  - persistent nudge zone for attention
  - bottom floating action bar for agency and entry
- Persistent bottom actions should include:
  - voice
  - capture
  - ask / command
- The dominant `Now` item is the active task.
- Persistent voice/input entry remains subordinate to the active work object, not the primary visual focus.
- Objects are not owned by threads and may relate to multiple threads.
- `Project`, `Client`, `Message`, `Run`, `Action`, `Relation`, and `Artifact` are first-class objects in the design model.
- Nudges are object-targeted by default and act as structured intervention primitives rather than generic notifications.
- Actions are durable and reviewable, using a stable base grammar and lifecycle.
- Confidence is a first-class cross-cutting signal used in actions, nudges, inference, and escalation.

## Intent

These notes exist so the milestone packet is traceable to the imported packs without leaving planning authority trapped outside the repo.

## Additional operator decisions captured in chat on 2026-03-23

- `Now` is strictly bounded and must not drift into inbox breadth.
- `Now` allows only:
  - active task
  - one or two next items
  - current and next event only
  - nudges
  - trust state
- `Now` forbids:
  - long queues
  - scrolling task lists
  - project grouping
  - generic today-view dumping
- The bottom action bar is always visible except in extreme focus modes where it must be instantly recallable.
- `Threads` is object/context first and chronology second.
- `System` is hybrid:
  - read-first in `Overview` and `Integrations`
  - more operational in `Operations` and `Control`
- Projects are:
  - tag-only on `Now`
  - stronger context in `Threads`
  - first-class in `System`
- `Now` first-class rows are limited to:
  - task
  - event
  - nudge
  - trust/system state
- `Now` explicitly forbids:
  - messages
  - threads
  - artifacts
  - runs/logs
  - people
  - raw integrations
  - config
- `Now` follows one dominant slot plus one subordinate slot.
- Drawer is for shallow inspection; thread is for thinking, resolving, and deciding.
- Bounded config edits are allowed inside threads, but browsing and schema-level config stays in `System`.
- Shell chrome stays instrument-like and consistent across surfaces.
- Nudge zone is always present, but compresses outside `Now`.
- Action bar includes one brokered contextual quick slot in addition to voice/capture/ask/command.
- On mobile, the action bar is docked rather than free-floating.
- Breadcrumbs appear only when needed in focused subviews.
- On `Now`, task wins visually while event wins behaviorally as a constraint.
- `Now` shows at most two events: current and next.
- Nudges render in a dedicated lane on `Now`, not mixed inline with work rows.
- Completed items disappear immediately from `Now`, at most allowing a brief transient ghost acknowledgement.
- Trust/status band on `Now` appears only when degraded or critical.
- Thread ordering is hybrid: recency first with relevance/pinned context influence.
- Threads open to the continuity stream by default.
- Provenance is collapsed by default.
- Thread filters are sticky per thread.
- Run/action blocks are visually distinct from messages.
- `Control` should feel dense but readable, using structured rows with expandable detail.
- Logs are summary-first with drill-down.
- Optimistic by default:
  - complete task
  - dismiss nudge
  - defer nudge
  - toggle preference
- Confirmation required:
  - delete
  - disconnect
  - revoke auth
  - destructive resets
  - high-risk external actions
- Standardized everywhere:
  - inline feedback plus persistent review path
  - retry/review affordances
- Critical actions must never be hover-only.
- Visual direction stays copper/orange and dark-first.
- Typography should feel technical-instrumental with a slight editorial layer.
- Mono is reserved for timestamps, IDs, logs, and provenance.
- Object colors should be selective, not universal.
- Required distinct states:
  - warning
  - degraded
  - blocked
  - active
  - done
  - syncing
  - offline
- Project color defaults algorithmically with user override later.
- Thread color should not inherit project color by default.
- Client/provider colors should stay separate from project/object colors.
- Component system posture:
  - row-first
  - cards only for nudges, runs/actions, media/artifacts, and config
  - reduced metric-strip prominence
  - one canonical density
  - drawers used sparingly
- Accessibility non-negotiables:
  - high contrast
  - keyboard-first support
  - clear focus states
  - reduced motion
  - color never stands alone
  - minimum touch targets enforced
- Milestone deliverables should include:
  - docs
  - clickable mockups
  - browser proofs / screenshots / fixtures

# Milestone v0.5.6 Requirements

**Status:** IN PROGRESS  
**Milestone:** v0.5.6
**Theme:** single-node MVP and polished web UI

## Primary Inputs

- [TODO.md](/home/jove/code/vel/TODO.md)
- [v0.5.5-MILESTONE-AUDIT.md](/home/jove/code/vel/.planning/v0.5.5-MILESTONE-AUDIT.md)
- [00-FEEDBACK-TODO.md](/home/jove/code/vel/.planning/milestones/v0.5.6-single-node-mvp-polished-web-ui/00-FEEDBACK-TODO.md)

## Milestone Goal

Ship a working single-node MVP with a polished web UI. This line should make one local Vel node credibly usable end to end before any follow-on widening driven by later feedback.

## MVP Must-Pass Flows

- [ ] first-run onboarding through an onboarding nudge into new Core settings
- [ ] Core setup can set user name, node name, at least one LLM provider, at least one synced provider, and universal agent-knowledge profile data
- [ ] floating composer/task bar is disabled until Core setup is complete, with a developer-mode override
- [ ] Google auth/connect, reconnect, settings edit, calendar read, calendar write, and `Now` event rendering work
- [ ] Todoist auth/connect, reconnect, settings edit, read/create/edit/complete/reopen, tags, projects, due dates, and `Now` task rendering work
- [ ] `Now` drag-to-commit behavior can assign tasks to today and persist the intended Todoist-backed day truth
- [ ] full nudge lifecycle works, including onboarding/degraded-state/action nudges and retry-capable failure nudges
- [ ] full thread lifecycle works, including latest-thread selection, empty state, message send, archive, and call-mode entry
- [ ] full config/system lifecycle works for the major MVP object families: calendar, events, tasks, providers, client/node identity, and operator settings
- [ ] multimodal assistant works with local `llama.ccp` and OpenAI as the bare minimum providers
- [ ] text, image/file input, voice input, and thread-level call mode work at MVP level

## Requirement Buckets

- [ ] **MVP-56-01**: one single-node runtime can be configured, connected to its supported integrations, and used through the web client without fake/local-only critical paths
- [ ] **NOW-56-01**: `Now` reflects truthful current-day task/calendar state for a single node, including Todoist-backed inbox/today semantics and durable task-lane behavior
- [ ] **CHAT-56-01**: multimodal assistant/chat works through the supported local and hosted providers with explicit configurable backing, priority, and credentials/settings boundaries
- [ ] **SYSTEM-56-01**: `System` exposes the operator-facing single-node configuration and status needed for the MVP while hiding deeper technical surfaces behind developer mode
- [ ] **POLISH-56-01**: navbar, composer, nudges, `Now`, `Threads`, and `System` reach accepted polished-web-ui quality
- [ ] **VERIFY-56-01**: browser proof, focused execution-backed checks, and an honest deferred list close the line

## Scope Rules

- [ ] use `TODO.md` as the source backlog for this milestone
- [ ] ignore every `TODO.md` bullet that starts with `!` for this milestone
- [ ] preserve the current three-surface model: `Now`, `Threads`, `System`
- [ ] keep the work single-node and local-first; no distributed-system widening
- [ ] do not add new top-level surfaces or speculative planner/workflow-builder scope
- [ ] further phases after this milestone come from subsequent operator feedback rather than pre-expanding the roadmap now

## Locked Product Decisions

- [ ] local `llama.ccp` and OpenAI are the minimum required chat providers for milestone close
- [ ] OpenAI OAuth is required in this milestone in addition to API-key support
- [ ] provider integration should use a provider-agnostic outer abstraction with room for provider-specific escape hatches rather than pretending all providers are identical
- [ ] Google integration must cover auth/connect, reconnect, settings edit, calendar read, event rendering in `Now`, and calendar write
- [ ] Todoist integration must cover auth/connect, reconnect, settings edit, read/create/edit/complete/reopen, tags, projects, due dates, and drag-to-commit behavior
- [ ] Vel may keep its own task classification layer, but Todoist remains the effective source of truth for this milestone
- [ ] overdue tasks should appear together with today’s uncompleted tasks, using overdue tab/warning styling
- [ ] the current-day event boundary is bedtime/end-of-day when present, with sunrise-next-day as fallback if no bedtime/end marker exists
- [ ] on `Now`, the critical durable drag/drop outcome is that dragging to `Next Up` commits the task to today
- [ ] multimodal means text, file/image input, voice input, and thread-level two-way speech through a call-mode flow
- [ ] task mutation defaults to suggest-then-confirm, but direct-mutation mode must also be available as a configurable setting
- [ ] `Threads` should use standard modern left/right bubbles with no tails in this milestone
- [ ] docs should render inside the site frame
- [ ] unavailable services stay visible in `System` inside a collapsed-by-default section
- [ ] broken or partially configured integrations should surface through nudges rather than hidden failure
- [ ] normal operator mode should hide MVP-irrelevant technical/runtime internals, with developer mode exposing overrides and deeper settings
- [ ] Core settings must include structured and freeform “what the agent should know about you” profile data injected universally across providers
- [ ] assistant/provider failures should surface as nudges and offer retry when appropriate
- [ ] default thread selection is last thread or no thread, with a deliberate empty state when none exists
- [ ] desktop Chrome is the acceptance browser for this milestone
- [ ] manual operator QA is the closeout authority instead of screenshot-checklist gating
- [ ] provider routing for this milestone is a priority list plus optional ask-first behavior per provider
- [ ] universal injected operator profile is global for now, with overrides deferred to a later milestone

## Coverage Map From Feedback

### Navbar / Shell

- [ ] sync icon only spins during active sync
- [ ] current event and active task tags match navbar tag sizing
- [ ] docs link renders actual markdown content
- [ ] docs link matches neighboring navlink icon sizing/color
- [ ] composer shadow behaves correctly over scrolled content

### Nudges

- [ ] nudge pills share styling language with their associated icon rings
- [ ] icon rings move left and stop touching pills
- [ ] action buttons are standardized and correctly iconed
- [ ] nudges stay sticky with the navbar while scrolling

### Now

- [ ] duplicated active-task header content is removed and icon placement corrected
- [ ] active-task heading/pill iconography matches the accepted style
- [ ] inbox / next-up / backlog semantics follow the Todoist-backed rules in the feedback
- [ ] frontend shows all uncompleted Todoist tasks for today on `Now`
- [ ] current-day event window stays bounded by the current day / next sleep boundary
- [ ] overdue tasks are visible together with today tasks, with explicit overdue styling
- [ ] dragging a task to `Next Up` durably commits it to today

### Threads

- [ ] default latest thread renders as active on load
- [ ] no-thread state is intentional and visually polished
- [ ] current-thread header label changes from `LAST` to `LATEST`
- [ ] archive button uses the standardized action-chip component
- [ ] sending no longer duplicates the user message in thread view
- [ ] Vel chat bubble contrast is readable
- [ ] chat layout follows the accepted modern left/right split without tails
- [ ] thread-level call mode supports speech-to-text plus text-to-speech over the same assistant flow

### System

- [ ] onboarding nudge leads into Core settings
- [ ] Core settings gate the floating composer/task bar until minimum setup is complete, with developer override
- [ ] Core settings include user name, node name, provider setup, synced-provider setup, provider priority/order, ask-first behavior, and universal agent-profile fields
- [ ] left bar stays sticky with the navbar during scroll
- [ ] integrations/services show corresponding logo icons
- [ ] unavailable services collapse instead of cluttering the main view
- [ ] grounding upcoming-events section is removed from activity
- [ ] activity and similar sections use tighter table-like density
- [ ] clients have a location field in config/backend with auto-set support if feasible
- [ ] current mode no longer overlaps
- [ ] writeback/writegrant info moves behind developer mode
- [ ] deep technical settings move behind the top-line developer mode toggle

### General Functionality

- [ ] Google and Todoist integrations work, remain configurable/editable in settings, and feed `Now` truthfully
- [ ] multimodal chat works with local agent via `llama.ccp` and OpenAI including API-key and OpenAI OAuth paths
- [ ] chat backing / priority / related controls are configurable in `System`
- [ ] full drag-and-drop for `Now` tasks works truthfully
- [ ] LLM-driven task creation / tagging / editing including voice is working at MVP level

## Verbatim Feedback Copy

The following source text is copied directly so execution can refer back to exact wording rather than a normalized summary.

See [00-FEEDBACK-TODO.md](/home/jove/code/vel/.planning/milestones/v0.5.6-single-node-mvp-polished-web-ui/00-FEEDBACK-TODO.md).

## Completion Rule

This milestone is only ready to close when the single-node MVP is working, the web UI is polished to accepted level, and every in-scope `TODO.md` item is either completed or explicitly deferred with a reason.

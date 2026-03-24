# Milestone v0.6.0 Requirements

**Status:** QUEUED  
**Milestone:** v0.6.0
**Theme:** single-node MVP and polished web UI

## Primary Inputs

- [TODO.md](/home/jove/code/vel/TODO.md)
- [v0.5.5-MILESTONE-AUDIT.md](/home/jove/code/vel/.planning/v0.5.5-MILESTONE-AUDIT.md)
- [00-FEEDBACK-TODO.md](/home/jove/code/vel/.planning/milestones/v0.6.0-single-node-mvp-polished-web-ui/00-FEEDBACK-TODO.md)

## Milestone Goal

Ship a working single-node MVP with a polished web UI. This line should make one local Vel node credibly usable end to end before any follow-on widening driven by later feedback.

## Requirement Buckets

- [ ] **MVP-60-01**: one single-node runtime can be configured, connected to its supported integrations, and used through the web client without fake/local-only critical paths
- [ ] **NOW-60-01**: `Now` reflects truthful current-day task/calendar state for a single node, including Todoist-backed inbox/today semantics and durable task-lane behavior
- [ ] **CHAT-60-01**: multimodal assistant/chat works through the supported local and hosted providers with explicit configurable backing, priority, and credentials/settings boundaries
- [ ] **SYSTEM-60-01**: `System` exposes the operator-facing single-node configuration and status needed for the MVP while hiding deeper technical surfaces behind developer mode
- [ ] **POLISH-60-01**: navbar, composer, nudges, `Now`, `Threads`, and `System` reach accepted polished-web-ui quality
- [ ] **VERIFY-60-01**: browser proof, focused execution-backed checks, and an honest deferred list close the line

## Scope Rules

- [ ] use `TODO.md` as the source backlog for this milestone
- [ ] ignore every `TODO.md` bullet that starts with `!` for this milestone
- [ ] preserve the current three-surface model: `Now`, `Threads`, `System`
- [ ] keep the work single-node and local-first; no distributed-system widening
- [ ] do not add new top-level surfaces or speculative planner/workflow-builder scope
- [ ] further phases after this milestone come from subsequent operator feedback rather than pre-expanding the roadmap now

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

### Threads

- [ ] default latest thread renders as active on load
- [ ] current-thread header label changes from `LAST` to `LATEST`
- [ ] archive button uses the standardized action-chip component
- [ ] sending no longer duplicates the user message in thread view
- [ ] Vel chat bubble contrast is readable
- [ ] chat layout follows the accepted modern left/right split without tails

### System

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
- [ ] multimodal chat works with local agent via `llama.ccp`, Claude, and OpenAI including API-key and OpenAI OAuth paths
- [ ] chat backing / priority / related controls are configurable in `System`
- [ ] full drag-and-drop for `Now` tasks works truthfully
- [ ] LLM-driven task creation / tagging / editing including voice is working at MVP level

## Verbatim Feedback Copy

The following source text is copied directly so execution can refer back to exact wording rather than a normalized summary.

See [00-FEEDBACK-TODO.md](/home/jove/code/vel/.planning/milestones/v0.6.0-single-node-mvp-polished-web-ui/00-FEEDBACK-TODO.md).

## Completion Rule

This milestone is only ready to close when the single-node MVP is working, the web UI is polished to accepted level, and every in-scope `TODO.md` item is either completed or explicitly deferred with a reason.

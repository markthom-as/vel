---
title: Vel Task HUD system spec
status: ready
owner: product+engineering
priority: P0
---

# Vel Task HUD

## Summary
Vel should expose a task HUD that behaves less like a generic todo list and more like a live operational register of commitments under attention constraints.

## Boundary

This spec describes a candidate surface and policy layer, not an automatic new system authority.

- Existing commitments, nudges, risk, and threads already own meaningful runtime semantics in Vel.
- The HUD should default to being a derived surface over those systems.
- A new first-class `Task` domain should only be introduced if commitments cannot carry the required semantics cleanly and the ownership boundary is stated explicitly.
- If a new task domain is introduced, it must not duplicate risk, nudge, or thread authority by default.

The HUD should answer:
- What matters now?
- What is becoming urgent?
- What is blocked?
- What is drifting?
- What ritual or prep action is due?
- What risk is emerging?

## Product goals
- Make active commitments glanceable.
- Reduce executive-function friction.
- Expose task pressure, not just task existence.
- Support cross-surface rendering: desktop full panel, desktop compact HUD, mobile/watch glance mode, future AR overlay.
- Integrate with context, reminders, and the risk engine.

## Non-goals
- Replacing the entire note system.
- Building a full project management product before basic HUD semantics exist.
- Rendering dense task trees in compact or AR surfaces.

## Core model
The safest initial model is task-like HUD state derived from commitments plus risk/nudge/context signals.

If a separate durable `Task` model is later justified, it should be treated as an extension candidate rather than an assumed replacement for commitments.

### Canonical task states
- Pending
- Active
- Blocked
- Waiting
- Completed
- Cancelled

### Canonical task kinds
- Task
- Ritual
- Reminder
- Followup
- Checklist

### Canonical HUD groups
- Now
- Soon
- Waiting
- Drifting
- Ritual
- Threats

## Candidate schema
```rust
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub kind: TaskKind,
    pub source: TaskSource,
    pub priority: Priority,
    pub urgency: Urgency,
    pub estimated_duration: Option<Duration>,
    pub energy_required: Option<EnergyLevel>,
    pub deadline_at: Option<DateTime<Utc>>,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub snoozed_until: Option<DateTime<Utc>>,
    pub blocked_by: Vec<TaskId>,
    pub depends_on: Vec<TaskId>,
    pub last_touched_at: DateTime<Utc>,
    pub context_refs: Vec<ContextRef>,
    pub attention_score: f32,
    pub commitment_score: f32,
    pub lateness_risk: f32,
    pub decay_state: DecayState,
    pub visibility_mode: VisibilityMode,
}
```

This schema should not be treated as mandatory until the ownership boundary against commitments is resolved.

## Ranking
Attention score should be derived, not manually curated in most cases.

Base factors:
- urgency
- deadline proximity
- dependency pressure
- lateness risk
- decay pressure
- priority
- contextual fit

Negative factors:
- snoozed
- blocked with no action possible
- recently completed
- hidden by policy
- redundant sibling tasks

## HUD policy
Only glanceworthy tasks should show in compact or ambient modes.

Allow if:
- high attention score
- deadline soon
- ritual due
- risk detected
- user pinned task

Disallow if:
- completed
- snoozed
- blocked with no user-actionable next step
- low urgency backlog
- explicitly hidden

## Surfaces

### Full panel
Canonical UI. Supports grouping, search/filter later, full metadata, and quick actions.

### Compact HUD
Always-on-top desktop surface. Show top 1-3 active items, one risk, one ritual, and fast actions.

### Ambient mode
Minimal pressure field, not a dense list. Resolve into text only on interaction.

### Watch/mobile glance
Single compressed payload:
- current task
- next task
- top risk
- next ritual

### AR
Do not port full desktop UI. Use the same core semantics with far tighter compression.

## Fast actions
- Complete
- Snooze
- Start
- Break down
- Block
- Defer
- Pin to HUD
- Hide

## Inference sources
- Manual entry
- Calendar
- Email
- Agent suggestion
- System inference

In the near term, these should be interpreted as sources for derived HUD entries or commitment-backed ranking inputs, not evidence that a separate task authority already exists.

## Notifications
Must be policy-bound, sparse, and escalation-aware.
- no repeated spam
- strict snooze respect
- escalate only if risk increases or context changes materially

## Possible crate layout
```text
crates/
  vel-task-core/
  vel-task-actions/
  vel-task-ranking/
  vel-task-policy/
  vel-task-hud/
  vel-task-inference/
```

## Rollout plan
1. Decide whether the HUD can be commitment-backed first
2. Ranking + policy + HUD view model over the chosen source of truth
3. Full task panel + compact desktop HUD
4. Risk/ritual/inference integration
5. Voice + watch/mobile glance
6. Ambient mode
7. AR protocol spec

## Acceptance criteria
- The ownership boundary between HUD state and commitments is explicit.
- If a new task model exists, it is persisted and test-covered.
- HUD grouping and ranking are deterministic enough to test.
- Desktop HUD exists behind a feature flag.
- Risk engine fields can influence ranking and display.
- At least one ritual flow and one inferred task flow exist.

## Implementation tickets

See [docs/tickets/task-hud/](../tickets/task-hud/README.md) for the ticket pack.
